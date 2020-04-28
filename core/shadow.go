package core

import (
	// "encoding/hex"
	// "fmt"

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
type GetEthHeaderWithProofByNumberOptions struct {
	Format string `json:"format"`
}

type GetEthHeaderWithProofByNumberParams struct {
	Number  uint64                               `json:"block_num"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}

type GetEthHeaderWithProofByNumberJSONResp struct {
	Header types.Header                     `json:"header"`
	Proof  []util.DoubleNodeWithMerkleProof `json:"proof"`
}

type GetEthHeaderWithProofByNumberCodecResp struct {
	Header string `json:"header"`
	Proof  string `json:"proof"`
}

func (s *Shadow) GetEthHeaderWithProofByNumber(
	params GetEthHeaderWithProofByNumberParams,
	resp *interface{},
) error {
	header, err := util.Header(params.Number)
	jsonResp := GetEthHeaderWithProofByNumberJSONResp{}
	jsonResp.Header = header
	if err != nil {
		return err
	}

	// Proof header
	proof, err := util.Proof(&header)
	jsonResp.Proof = proof.Format()

	// Set response
	*resp = jsonResp

	// Check if need codec
	if params.Options.Format == "scale" {
		*resp = GetEthHeaderWithProofByNumberCodecResp{
			"",
			encodeProof(jsonResp.Proof),
		}
	}

	return err
}
