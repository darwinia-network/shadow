package eth

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rlp"
)

// Get ethereum header by block number
func Header(api string, block uint64) (types.Header, error) {
	// Get header from infura
	var (
		resp       *http.Response
		err        error
		infuraResp InfuraResponse
	)

	resp, err = http.Post(
		api,
		"application/json",
		strings.NewReader(fmt.Sprintf(GETBLOCK, block)),
	)

	if err != nil {
		return infuraResp.Result, err
	}

	// Decode resp to json
	defer resp.Body.Close()
	err = json.NewDecoder(resp.Body).Decode(&infuraResp)
	if err != nil {
		return infuraResp.Result, err
	}

	// Return eth header
	return infuraResp.Result, nil
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
func IntoDarwiniaEthHeader(e *types.Header) (DarwiniaEthHeader, error) {
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
	h.TimeStamp = e.Time
	h.Number = e.Number.Uint64()
	h.Author = strings.ToLower(e.Coinbase.Hex())
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
