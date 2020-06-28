package eth

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"net/http"
	"reflect"
	"strings"

	"github.com/darwinia-network/shadow/internal/util"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rlp"
)

// The post api of fetching eth header
const GETBLOCK = "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByNumber\",\"params\": [\"0x%x\", false],\"id\":1}\n"
const GETBLOCK_BYHASH = "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByHash\",\"params\": [\"%s\", false],\"id\":1}\n"

// The response of etherscan api
type InfuraResponse struct {
	JsonRPC string       `json:"jsonrpc"`
	Id      uint32       `json:"id"`
	Result  types.Header `json:"result"`
}

// Get ethereum header by block number
func Header(block interface{}, api string, geth Geth) (types.Header, error) {
	// Get header from infura
	var (
		resp       *http.Response
		err        error
		infuraResp InfuraResponse
	)

	// try to get headers from db
	if !util.IsEmpty(geth) {
		block := geth.GetBlock(block)
		if !util.IsEmpty(block) {
			return block, nil
		}
	}

	// Request block by number or hash
	switch b := block.(type) {
	case uint64:
		resp, err = http.Post(
			api,
			"application/json",
			strings.NewReader(fmt.Sprintf(GETBLOCK, b)),
		)
	case string:
		resp, err = http.Post(
			api,
			"application/json",
			strings.NewReader(fmt.Sprintf(GETBLOCK_BYHASH, b)),
		)
	default:
		return infuraResp.Result, fmt.Errorf("Heaader function only accepts blockHash and blockNumber")
	}
	if err != nil {
		return infuraResp.Result, err
	}

	// Decode resp to json
	defer resp.Body.Close()
	err = json.NewDecoder(resp.Body).Decode(&infuraResp)
	if err != nil {
		return infuraResp.Result, err
	}

	// Empty result
	if reflect.DeepEqual(types.Header{}, infuraResp.Result) {
		return infuraResp.Result, fmt.Errorf("The requesting block does not exist")
	}

	// Return eth header
	return infuraResp.Result, nil
}

// Darwinia block
type DarwiniaEthHeader struct {
	ParentHash       string   `json:"parent_hash"`
	TimeStamp        uint64   `json:"timestamp"`
	Number           uint64   `json:"number"`
	Author           string   `json:"author"`
	TransactionsRoot string   `json:"transactions_root"`
	UnclesHash       string   `json:"uncles_hash"`
	ExtraData        string   `json:"extra_data"`
	StateRoot        string   `json:"state_root"`
	ReceiptsRoot     string   `json:"receipts_root"`
	LogBloom         string   `json:"log_bloom"`
	GasUsed          uint64   `json:"gas_used"`
	GasLimited       uint64   `json:"gas_limit"`
	Difficulty       uint64   `json:"difficulty"`
	Seal             []string `json:"seal"`
	Hash             string   `json:"hash"`
}

type DarwiniaEthHeaderHexFormat struct {
	ParentHash       string   `json:"parent_hash"`
	TimeStamp        string   `json:"timestamp"`
	Number           string   `json:"number"`
	Author           string   `json:"author"`
	TransactionsRoot string   `json:"transactions_root"`
	UnclesHash       string   `json:"uncles_hash"`
	ExtraData        string   `json:"extra_data"`
	StateRoot        string   `json:"state_root"`
	ReceiptsRoot     string   `json:"receipts_root"`
	LogBloom         string   `json:"log_bloom"`
	GasUsed          string   `json:"gas_used"`
	GasLimited       string   `json:"gas_limit"`
	Difficulty       string   `json:"difficulty"`
	Seal             []string `json:"seal"`
	Hash             string   `json:"hash"`
}

func (h *DarwiniaEthHeader) HexFormat() DarwiniaEthHeaderHexFormat {
	return DarwiniaEthHeaderHexFormat{
		h.ParentHash,
		hexutil.EncodeUint64(h.TimeStamp),
		hexutil.EncodeUint64(h.Number),
		h.Author,
		h.TransactionsRoot,
		h.UnclesHash,
		h.ExtraData,
		h.StateRoot,
		h.ReceiptsRoot,
		h.LogBloom,
		hexutil.EncodeUint64(h.GasUsed),
		hexutil.EncodeUint64(h.GasLimited),
		hexutil.EncodeUint64(h.Difficulty),
		h.Seal,
		h.Hash,
	}
}

// Convert EthHeader to Darwinia Eth Block
func IntoDarwiniaEthHeader(e types.Header) (DarwiniaEthHeader, error) {
	h := DarwiniaEthHeader{}
	mixh, err := rlp.EncodeToBytes(e.MixDigest)
	if err != nil {
		return h, err
	}

	nonce, err := rlp.EncodeToBytes(e.Nonce)
	if err != nil {
		return h, err
	}

	h.Seal = []string{
		"0x" + hex.EncodeToString(mixh),
		"0x" + hex.EncodeToString(nonce),
	}

	h.ParentHash = e.ParentHash.Hex()
	h.TimeStamp = e.Time.Uint64()
	h.Number = e.Number.Uint64()
	h.Author = e.Coinbase.Hex()
	h.TransactionsRoot = e.TxHash.Hex()
	h.UnclesHash = e.UncleHash.Hex()
	h.ExtraData = "0x" + hex.EncodeToString(e.Extra)
	h.StateRoot = e.Root.Hex()
	h.ReceiptsRoot = e.ReceiptHash.Hex()
	h.LogBloom = "0x" + hex.EncodeToString(e.Bloom.Bytes())
	h.GasUsed = e.GasUsed
	h.GasLimited = e.GasLimit
	h.Difficulty = e.Difficulty.Uint64()
	h.Hash = e.Hash().Hex()

	return h, nil
}
