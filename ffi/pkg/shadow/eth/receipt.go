package eth

import (
	"bytes"
	"context"
	"errors"
	"math"
	"strings"
	"sync"

	"github.com/darwinia-network/shadow/ffi/pkg/log"
	"github.com/darwinia-network/shadow/ffi/pkg/shadow/util"
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

func GetChainBlockInfo(api string, blockNum int64) (*BlockResult, error) {
	params := []interface{}{util.IntToHex(blockNum), true}
	if result, err := RPC(api, "eth_getBlockByNumber", params); err != nil {
		return nil, err
	} else {
		var block BlockResult
		err := mapstructure.Decode(result.Result, &block)
		return &block, err
	}
}

func RPC(api string, method string, params interface{}) (*dto.RequestResult, error) {
	replacer := strings.NewReplacer("https://", "", "http://", "")
	provider := providers.NewHTTPProvider(replacer.Replace(api), 10, strings.HasPrefix(api, "https://"))
	pointer := &dto.RequestResult{}
	err := provider.SendRequest(pointer, method, params)
	if err != nil {
		log.Error("rpc rquest api %v, method %v, %v", api, method, err)
		return nil, err
	}

	return pointer, nil
}

func GetReceiptLog(tx string, api string) (*Receipts, error) {
	params := make([]interface{}, 1)
	params[0] = tx
	var receipt Receipts
	if receiptResult, err := RPC(api, "eth_getTransactionReceipt", params); err != nil {
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

type ProofRecord struct {
	Index      string `json:"index"`
	Proof      string `json:"proof"`
	HeaderHash string `json:"header_hash"`
}

func GetReceipt(api string, tx string) (ProofRecord, string, error) {
	r, err := GetReceiptLog(tx, api)
	if err != nil || r == nil {
		log.Error("get receipt failed with api %s, err %v", api, err)
		return ProofRecord{}, "", err
	}

	return BuildProofRecord(api, r)
}

func BuildProofRecord(api string, r *Receipts) (ProofRecord, string, error) {
	proofRecord := ProofRecord{
		Index:      r.TransactionIndex,
		HeaderHash: r.BlockHash,
	}
	block, err := GetChainBlockInfo(api, util.U256(r.BlockNumber).Int64())
	if err != nil {
		return ProofRecord{}, "", err
	}
	receiptsMap := cmap.New()

	var wg sync.WaitGroup
	p, _ := ants.NewPoolWithFunc(10, func(i interface{}) {
		hash := i.([]string)
		func(hash []string) {
			b, err := GetReceiptRlpEncode(api, hash[1])
			if err == nil {
				receiptsMap.Set(hash[0], b)
			} else {
				log.Error("get receipt rlp failed error %v", err)
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

func GetReceiptRlpEncode(api string, tx string) (*types.Receipt, error) {
	client, err := ethclient.Dial(api)
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

	var receiptList types.Receipts
	for i := 0; i < len(receipts); i++ {
		receiptList = append(receiptList, receipts[util.IntToString(i)].(*types.Receipt))
	}

	for i := range receipts {
		path, err := rlp.EncodeToBytes(uint(util.StringToInt(i)))
		if err != nil {
			return nil, err
		}
		w := new(bytes.Buffer)
		receiptList.EncodeIndex(util.StringToInt(i), w)
		tr.Update(path, w.Bytes())
	}

	return tr, nil
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
