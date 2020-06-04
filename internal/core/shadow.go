package core

import (
	"fmt"

	"github.com/darwinia-network/darwinia.go/internal"
	"github.com/darwinia-network/darwinia.go/internal/eth"
	"github.com/darwinia-network/darwinia.go/internal/util"
	"github.com/ethereum/go-ethereum/core/types"
)

// Shadow genesis block error message
const GENESIS_ERROR = "The requested block number is too low, only support blocks heigher than %v"
const PROOF_LOCK = "proof.lock"

// Dimmy shadow service
type Shadow struct {
	Config internal.Config
	Geth   eth.Geth
}

/**
 * Genesis block checker
 */
func (s *Shadow) checkGenesis(genesis uint64, block interface{}, api string) error {
	switch b := block.(type) {
	case uint64:
		if b < genesis {
			return fmt.Errorf(GENESIS_ERROR, genesis)
		}
	case string:
		eH, err := eth.Header(b, api, s.Geth)
		if err != nil {
			return err
		}

		// convert ethHeader to darwinia header
		dH, err := eth.IntoDarwiniaEthHeader(eH)
		if err != nil {
			return err
		}

		// Check hash empty response
		if util.IsEmpty(dH) {
			return fmt.Errorf("Empty block: %s", b)
		}

		// Check genesis by number
		if dH.Number <= genesis {
			return fmt.Errorf(GENESIS_ERROR, genesis)
		}
	default:
		return fmt.Errorf("Invaild block param: %v", block)
	}

	return nil
}

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
	err := s.checkGenesis(s.Config.Genesis, params.Number, s.Config.Api)
	if err != nil {
		return err
	}

	// Return raw eth header
	resp.Header, err = eth.Header(params.Number, s.Config.Api, s.Geth)
	return err
}

/**
 * GetEthHeaderByHash
 */
type GetEthHeaderByHashParams struct {
	Hash string `json:"hash"`
}

type GetEthHeaderByHashResp struct {
	Header types.Header `json:"header"`
}

func (s *Shadow) GetEthHeaderByHash(
	params GetEthHeaderByHashParams,
	resp *GetEthHeaderByHashResp,
) error {
	err := s.checkGenesis(s.Config.Genesis, params.Hash, s.Config.Api)
	if err != nil {
		return err
	}

	// Return raw eth header
	resp.Header, err = eth.Header(params.Hash, s.Config.Api, s.Geth)
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

type GetEthHeaderWithProofByNumberRawResp struct {
	Header eth.DarwiniaEthHeader           `json:"eth_header"`
	Proof  []eth.DoubleNodeWithMerkleProof `json:"proof"`
}

type GetEthHeaderWithProofByNumberJSONResp struct {
	Header eth.DarwiniaEthHeaderHexFormat  `json:"eth_header"`
	Proof  []eth.DoubleNodeWithMerkleProof `json:"proof"`
}

type GetEthHeaderWithProofByNumberCodecResp struct {
	Header string `json:"eth_header"`
	Proof  string `json:"proof"`
}

func (s *Shadow) GetEthHeaderWithProofByNumber(
	params GetEthHeaderWithProofByNumberParams,
	resp *interface{},
) error {
	err := s.checkGenesis(s.Config.Genesis, params.Number, s.Config.Api)
	if err != nil {
		return err
	}

	// Fetch header from cache
	cache := EthHeaderWithProofCache{Number: params.Number}
	rawResp, err := cache.Fetch()

	// Fetch header from infura
	if err != nil {
		// Fetch eth header
		ethHeader, err := eth.Header(params.Number, s.Config.Api, s.Geth)
		if err != nil {
			return err
		}

		rawResp.Header, err = eth.IntoDarwiniaEthHeader(ethHeader)
		if err != nil {
			return err
		}

		// Check proof lock
		if s.Config.CheckLock(PROOF_LOCK) {
			return fmt.Errorf("Shadow service is busy now, please try again later")
		} else {
			err := s.Config.CreateLock(PROOF_LOCK, []byte(""))
			if err != nil {
				return err
			}
		}

		// Proof header
		proof, err := eth.Proof(&ethHeader, s.Config)
		rawResp.Proof = proof.Format()
		if err != nil {
			return err
		}

		// Remove proof lock
		err = s.Config.RemoveLock(PROOF_LOCK)
		if err != nil {
			return err
		}

		// Create cache
		err = cache.FromResp(rawResp)
		if err != nil {
			return err
		}
	}

	// Set response
	*resp = rawResp

	// Check if need codec
	if params.Options.Format == "scale" {
		*resp = GetEthHeaderWithProofByNumberCodecResp{
			encodeDarwiniaEthHeader(rawResp.Header),
			encodeProofArray(rawResp.Proof),
		}
	} else if params.Options.Format == "json" {
		*resp = GetEthHeaderWithProofByNumberJSONResp{
			rawResp.Header.HexFormat(),
			rawResp.Proof,
		}
	}

	return nil
}

/**
 * GetEthHeaderWithProofByNumber
 */
type GetEthHeaderWithProofByHashParams struct {
	Hash    string                               `json:"hash"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}

func (s *Shadow) GetEthHeaderWithProofByHash(
	params GetEthHeaderWithProofByHashParams,
	resp *interface{},
) error {
	eH, err := eth.Header(params.Hash, s.Config.Api, s.Geth)
	if err != nil {
		return err
	}

	// convert ethHeader to darwinia header
	dH, err := eth.IntoDarwiniaEthHeader(eH)
	if err != nil {
		return err
	}

	// construct number req
	p := GetEthHeaderWithProofByNumberParams{
		dH.Number,
		params.Options,
	}

	return s.GetEthHeaderWithProofByNumber(p, resp)
}

/**
 * BatchEthHeaderWithProofByNumber
 */
type BatchEthHeaderWithProofByNumberParams struct {
	Number  uint64                               `json:"number"`
	Batch   int                                  `json:"batch"`
	Options GetEthHeaderWithProofByNumberOptions `json:"options"`
}

func (s *Shadow) BatchEthHeaderWithProofByNumber(
	params BatchEthHeaderWithProofByNumberParams,
	resp *interface{},
) error {
	var nps []interface{}
	for i := 0; i < params.Batch; i++ {
		var np interface{}
		err := s.GetEthHeaderWithProofByNumber(GetEthHeaderWithProofByNumberParams{
			Number:  params.Number + uint64(i),
			Options: params.Options,
		}, &np)

		if err != nil {
			return err
		}

		nps = append(nps, np)
	}

	*resp = nps
	return nil
}
