package core

import (
	"github.com/darwinia-network/darwinia.go/util"
	"github.com/ethereum/go-ethereum/core/types"
)

// Dimmy shadow service
type Shadow int

/**
 * @method: GetEthHeaderByNumber
 */
type GetEthHeaderByNumberParams struct {
	Number uint64 `json:"number"`
}

type GetEthHeaderByNumberResp struct {
	Header types.Header `json:"header"`
}

func (s *Shadow) GetEthHeaderByNumber(
	params GetEthHeaderByNumberParams,
	resp GetEthHeaderByNumberResp,
) error {
	resp.Header = util.Header(params.Number)
	return nil
}

/**
 * @method: GetEthHeaderWithProofByNumber
 */
type GetEthHeaderWithProofByNumberParams struct {
	Number uint64 `json:"number"`
}

type GetEthHeaderWithProofByNumberResp struct {
	Header types.Header `json:"header"`
}

func (s *Shadow) GetEthHeaderWithProofByNumber(
	params GetEthHeaderWithProofByNumberParams,
	resp GetEthHeaderWithProofByNumberResp,
) error {
	resp.Header = util.Header(params.Number)
	return nil
}
