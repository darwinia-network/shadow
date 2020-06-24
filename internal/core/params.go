package core

import (
	"github.com/darwinia-network/darwinia.go/internal/eth"
	"github.com/ethereum/go-ethereum/core/types"
)

// Get Header
type GetEthHeaderByNumberParams struct {
	Number uint64 `json:"number"`
}

type GetEthHeaderByHashParams struct {
	Hash string `json:"hash"`
}

type GetEthHeaderResp struct {
	Header types.Header `json:"header"`
}

// Get Header With Proof
type GetEthHeaderWithProofByNumberParams struct {
	Number  uint64                               `json:"block_num"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}

type GetEthHeaderWithProofByNumberOptions struct {
	Format string `json:"format"`
}

type GetEthHeaderWithProofByNumberRawResp struct {
	Header eth.DarwiniaEthHeader           `json:"eth_header"`
	Proof  []eth.DoubleNodeWithMerkleProof `json:"proof"`
	Root   string                          `json:"root"`
}

type GetEthHeaderWithProofByNumberJSONResp struct {
	Header eth.DarwiniaEthHeaderHexFormat  `json:"eth_header"`
	Proof  []eth.DoubleNodeWithMerkleProof `json:"proof"`
	Root   string                          `json:"root"`
}

type GetEthHeaderWithProofByNumberCodecResp struct {
	Header string `json:"eth_header"`
	Proof  string `json:"proof"`
	Root   string `json:"root"`
}

type GetEthHeaderWithProofByHashParams struct {
	Hash    string                               `json:"hash"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}

// Batch Header
type BatchEthHeaderWithProofByNumberParams struct {
	Number  uint64                               `json:"number"`
	Batch   int                                  `json:"batch"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}
