package core

import (
	"fmt"
	"log"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/eth"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/jinzhu/gorm"
)

// Shadow genesis block error message
const GENESIS_ERROR = "The requested block number is too low, only support blocks heigher than %v"
const PROOF_LOCK = "proof.lock"

// Dimmy shadow service
type Shadow struct {
	Config internal.Config
	Geth   eth.Geth
	DB     *gorm.DB
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
func (s *Shadow) GetEthHeaderByNumber(
	params GetEthHeaderByNumberParams,
	resp *GetEthHeaderResp,
) error {
	log.Println("Request /GetEthHeaderByNumber")
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
func (s *Shadow) GetEthHeaderByHash(
	params GetEthHeaderByHashParams,
	resp *GetEthHeaderResp,
) error {
	log.Println("Request /GetEthHeaderByHash")
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
func (s *Shadow) GetEthHeaderWithProofByNumber(
	params GetEthHeaderWithProofByNumberParams,
	resp *interface{},
) error {
	log.Println("Request /GetEthHeaderWithProofByNumber")
	err := s.checkGenesis(s.Config.Genesis, params.Number, s.Config.Api)
	if err != nil {
		return err
	}

	// Fetch header from cache
	cache := EthHeaderWithProofCache{Number: params.Number}
	err = cache.Fetch(s.Config, s.DB, s.Geth)
	if err != nil {
		return err
	}

	err = cache.ApplyProof(s.Config, s.DB, s.Geth)
	if err != nil {
		return err
	}

	rawResp, err := cache.IntoResp()
	if err != nil {
		return err
	}

	// Set response
	*resp = rawResp

	// Check if need codec
	if params.Options.Format == "scale" {
		*resp = GetEthHeaderWithProofByNumberCodecResp{
			encodeDarwiniaEthHeader(rawResp.Header),
			encodeProofArray(rawResp.Proof),
			rawResp.Root,
		}
	} else if params.Options.Format == "json" {
		*resp = GetEthHeaderWithProofByNumberJSONResp{
			rawResp.Header.HexFormat(),
			rawResp.Proof,
			rawResp.Root,
		}
	}

	return nil
}

/**
 * GetEthHeaderWithProofByNumber
 */
func (s *Shadow) GetEthHeaderWithProofByHash(
	params GetEthHeaderWithProofByHashParams,
	resp *interface{},
) error {
	log.Println("Request /GetEthHeaderWithProofByHash")
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
func (s *Shadow) BatchEthHeaderWithProofByNumber(
	params BatchEthHeaderWithProofByNumberParams,
	resp *interface{},
) error {
	log.Println("Request /BatchEthHeaderWithProofByNumber")
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

/**
 * BatchEthHeaderWithProofByNumber
 */
func (s *Shadow) GetProposalEthHeaders(
	params GetProposalEthHeadersParams,
	resp *interface{},
) error {
	log.Println("Request /GetProposalEthHeaders")
	var nps []interface{}
	for _, i := range params.Numbers {
		var np interface{}
		err := s.GetEthHeaderWithProofByNumber(GetEthHeaderWithProofByNumberParams{
			Number:  i,
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
