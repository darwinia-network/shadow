package core

import (
	"github.com/darwinia-network/shadow/internal/eth"
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

type GetEthHeaderWithProofByHashParams struct {
	Hash    string                               `json:"hash"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}

type GetEthHeaderWithProofJSONResp struct {
	Header eth.DarwiniaEthHeaderHexFormat  `json:"eth_header"`
	Proof  []eth.DoubleNodeWithMerkleProof `json:"ethash_proof"`
	Root   string                          `json:"mmr_root"`
}

type GetEthHeaderWithProofCodecResp struct {
	Header string `json:"eth_header"`
	Proof  string `json:"ethash_proof"`
	Root   string `json:"mmr_root"`
}

type GetEthHeaderWithProofRawResp struct {
	Header eth.DarwiniaEthHeader           `json:"eth_header"`
	Proof  []eth.DoubleNodeWithMerkleProof `json:"ethash_proof"`
	Root   string                          `json:"mmr_root"`
}

func (r *GetEthHeaderWithProofRawResp) IntoCoedc() GetEthHeaderWithProofCodecResp {
	return GetEthHeaderWithProofCodecResp{
		encodeDarwiniaEthHeader(r.Header),
		encodeProofArray(r.Proof),
		r.Root,
	}
}

func (r *GetEthHeaderWithProofRawResp) IntoJSON() GetEthHeaderWithProofJSONResp {
	return GetEthHeaderWithProofJSONResp{
		r.Header.HexFormat(),
		r.Proof,
		r.Root,
	}
}

// Batch Header
type BatchEthHeaderWithProofByNumberParams struct {
	Number  uint64                               `json:"number"`
	Batch   int                                  `json:"batch"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}

// Proposal Header
type GetProposalEthHeadersParams struct {
	Numbers []uint64                             `json:"number"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}

type ProposalHeader struct {
	Header   eth.DarwiniaEthHeader           `json:"eth_header"`
	Proof    []eth.DoubleNodeWithMerkleProof `json:"ethash_proof"`
	Root     string                          `json:"mmr_root"`
	MMRProof []string                        `json:"mmr_proof"`
}

type ProposalResp struct {
	Headers []ProposalHeader `json:"headers"`
}

// Receipt
type GetReceiptResp struct {
	ReceiptProof eth.ProofRecord `json:"receipt_proof"`
	MMRProof     []string        `json:"mmr_proof"`
}
