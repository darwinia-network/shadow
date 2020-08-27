package eth

import (
	"bytes"
	"context"
	"errors"
	"math"
	"strings"
	"sync"

	"github.com/darwinia-network/shadow/pkg/shadow"
	"github.com/darwinia-network/shadow/pkg/shadow/util"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/ethdb/memorydb"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/ethereum/go-ethereum/trie"
	"github.com/mitchellh/mapstructure"
	cmap "github.com/orcaman/concurrent-map"
	"github.com/panjf2000/ants"
	"github.com/regcostajr/go-web3/dto"
	"github.com/regcostajr/go-web3/providers"
)

var (
	API string
)

func init() {
	conf := new(shadow.Config)
	_ = conf.Load()
	API = conf.Api
}

func GetChainBlockInfo(blockNum int64) (*BlockResult, error) {
	params := []interface{}{util.IntToHex(blockNum), true}
	if result, err := RPC("eth_getBlockByNumber", params); err != nil {
		return nil, err
	} else {
		var block BlockResult
		err := mapstructure.Decode(result.Result, &block)
		return &block, err
	}
}

func RPC(method string, params interface{}) (*dto.RequestResult, error) {
	provider := providers.NewHTTPProvider(strings.ReplaceAll(API, "https://", ""), 10, true)
	pointer := &dto.RequestResult{}
	err := provider.SendRequest(pointer, method, params)
	if err != nil {
		return nil, err
	}

	return pointer, nil
}

func GetReceiptLog(tx string) (*Receipts, error) {
	params := make([]interface{}, 1)
	params[0] = tx
	var receipt Receipts
	if receiptResult, err := RPC("eth_getTransactionReceipt", params); err != nil {
		return nil, err
	} else {
		err := mapstructure.Decode(receiptResult.Result, &receipt)
		if err != nil {
			return nil, err
		}
		receipt.ChainSource = "Eth"
		receipt.Solidity = true
		return &receipt, err
	}
}

type EthHeader struct {
	ParentHash      string   `json:"parent_hash"`
	Timestamp       int64    `json:"timestamp"`
	Number          int64    `json:"number"`
	Auth            string   `json:"auth"`
	TransactionRoot string   `json:"transaction_root"`
	UnclesHash      string   `json:"uncles_hash"`
	ExtraData       string   `json:"extra_data"`
	StateRoot       string   `json:"state_root"`
	ReceiptsRoot    string   `json:"receipts_root"`
	LogBloom        string   `json:"log_bloom"`
	GasUsed         string   `json:"gas_used"`
	GasLimit        string   `json:"gas_limit"`
	Difficulty      string   `json:"difficulty"`
	Seal            []string `json:"seal"`
	Hash            string   `json:"hash"`
}

type EthReceiptLog struct {
	Address string   `json:"address"`
	Topics  []string `json:"topics"`
	Data    []byte   `json:"data"`
}

type EthReceipt struct {
	GasUsed  int64           `json:"gas_used"`
	LogBloom string          `json:"log_bloom"`
	Logs     []EthReceiptLog `json:"logs"`
	Outcome  int             `json:"outcome"`
}

type ProofRecord struct {
	Index      string `json:"index"`
	Proof      string `json:"proof"`
	HeaderHash string `json:"header_hash"`
}

type RedeemFor struct {
	Ring    *ProofRecord `json:"ring,omitempty"`
	Kton    *ProofRecord `json:"kton,omitempty"`
	Deposit *ProofRecord `json:"deposit,omitempty"`
}

func GetReceipt(tx string) (ProofRecord, string, error) {
	r, err := GetReceiptLog(tx)
	if err != nil || r == nil {
		return ProofRecord{}, "", err
	}

	return BuildProofRecord(r)
}

func BuildProofRecord(r *Receipts) (ProofRecord, string, error) {
	proofRecord := ProofRecord{
		Index:      r.TransactionIndex,
		HeaderHash: r.BlockHash,
	}
	block, err := GetChainBlockInfo(util.U256(r.BlockNumber).Int64())
	if err != nil {
		return ProofRecord{}, "", err
	}
	receiptsMap := cmap.New()

	var wg sync.WaitGroup
	p, _ := ants.NewPoolWithFunc(10, func(i interface{}) {
		hash := i.([]string)
		func(hash []string) {
			b, err := GetReceiptRlpEncode(hash[1])
			if err == nil {
				receiptsMap.Set(hash[0], b)
			}
		}(hash)
		wg.Done()
	})
	defer p.Release()

	for index, transaction := range block.Transactions {
		wg.Add(1)
		_ = p.Invoke([]string{util.IntToString(index), transaction.Hash})
	}
	wg.Wait()
	if receiptsMap.Count() != len(block.Transactions) {
		return ProofRecord{}, block.Hash, errors.New("get receipt Rlp errors")
	}
	tr, err := trieFromReceipts(receiptsMap.Items())
	if err != nil {
		return ProofRecord{}, block.Hash, err
	}
	proof := memorydb.New()
	buf := new(bytes.Buffer)
	_ = rlp.Encode(buf, uint(util.U256(r.TransactionIndex).Uint64()))
	key := buf.Bytes()
	proofs := make([]interface{}, 0)
	_ = tr.Prove(key, 0, proof)
	if it := trie.NewIterator(tr.NodeIterator(key)); it.Next() && bytes.Equal(key, it.Key) {
		for _, p := range it.Prove() {
			proofs = append(proofs, p)
		}
	}
	buf.Reset()
	_ = rlp.Encode(buf, proofs)
	length := rlpLength(len(buf.Bytes()), 0xc0)
	ProofRlpBytes := append(length[:], buf.Bytes()...)
	proofRecord.Proof = util.AddHex(util.BytesToHex(ProofRlpBytes))
	return proofRecord, block.Hash, nil
}

func GetReceiptRlpEncode(tx string) (*types.Receipt, error) {
	client, err := ethclient.Dial(API)
	if err != nil {
		return nil, err
	}
	r, err := client.TransactionReceipt(context.Background(), common.HexToHash(tx))
	if err != nil {
		return nil, err
	}
	return r, nil
}

func trieFromReceipts(receipts map[string]interface{}) (*trie.Trie, error) {
	tr := new(trie.Trie)

	for i, r := range receipts {
		path, err := rlp.EncodeToBytes(uint(util.StringToInt(i)))
		if err != nil {
			return nil, err
		}

		rawReceipt, err := encodeReceipt(r.(*types.Receipt))
		if err != nil {
			return nil, err
		}

		tr.Update(path, rawReceipt)
	}

	return tr, nil
}

func encodeReceipt(r *types.Receipt) ([]byte, error) {
	buf := new(bytes.Buffer)
	if err := r.EncodeRLP(buf); err != nil {
		return nil, err
	}

	return buf.Bytes(), nil
}

func rlpLength(dataLen int, offset byte) []byte {
	if dataLen < 56 {
		return []byte{byte(dataLen) + offset}
	} else if dataLen < math.MaxInt32 {
		var output []byte
		b := toBinary(dataLen)
		output = append(output, byte(len(b)+int(offset)+55))
		return append(output, b...)
	} else {
		return []byte{}
	}
}

func toBinary(d int) []byte {
	var b []byte
	for d > 0 {
		b = append([]byte{byte(d % 256)}, b...)
		d /= 256
	}
	return b
}

func RlpEncode(hexStr string) string {
	prefix := rlpLength(len(util.HexToBytes(hexStr)), 0x80)
	return util.BytesToHex(prefix) + util.TrimHex(hexStr)
}
