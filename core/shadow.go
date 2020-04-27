package core

import (
	"github.com/darwinia-network/darwinia.go/util"
	"github.com/ethereum/go-ethereum/core/types"
)

// Dimmy shadow service
type Shadow int

/**
 * GetEthHeaderByNumber
 */
type GetEthHeaderByNumberParams struct {
	Number uint64 `json:"number"`
}

type GetEthHeaderByNumberResp struct {
	Header types.Header `json:"header"`
}

func (s *Shadow) GetEthHeaderByNumber(
	params GetEthHeaderByNumberParams,
	resp *GetEthHeaderByNumberResp,
) error {
	var err error
	resp.Header, err = util.Header(params.Number)
	return err
}

/**
 * GetEthHeaderWithProofByNumber
 */
type GetEthHeaderWithProofByNumberParams struct {
	Number uint64 `json:"number"`
}

type GetEthHeaderWithProofByNumberResp struct {
	Header types.Header                     `json:"header"`
	Proof  []util.DoubleNodeWithMerkleProof `json:"proof"`
}

func (s *Shadow) GetEthHeaderWithProofByNumber(
	params GetEthHeaderWithProofByNumberParams,
	resp *GetEthHeaderWithProofByNumberResp,
) error {
	header, err := util.Header(params.Number)
	resp.Header = header
	if err != nil {
		return err
	}

	// Proof header
	proof, err := util.Proof(&header)
	resp.Proof = proof.Format()
	return err
}
