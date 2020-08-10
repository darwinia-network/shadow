package core

import (
	"fmt"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/eth"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/jinzhu/gorm"
)

// Shadow genesis block error message
const GENESIS_ERROR = "The requested block number is too low, only support blocks heigher than %v"
const PROOF_LOCK = "proof.lock"

// Dimmy shadow service
type Shadow struct {
	Config internal.Config
	DB     *gorm.DB
	Geth   eth.Geth
}

func NewShadow() (Shadow, error) {
	conf := new(internal.Config)
	err := conf.Load()
	if err != nil {
		return Shadow{}, err
	}

	db, err := ConnectDb()
	if err != nil {
		return Shadow{}, err
	}

	geth, err := eth.NewGeth(conf.Geth)
	if err != nil {
		log.Warn("Geth path doen't confirgured")
	}

	return Shadow{
		*conf,
		db,
		geth,
	}, err
}

// Genesis block checker
func (s *Shadow) checkGenesis(genesis uint64, block interface{}) (uint64, error) {
	block, err := util.NumberOrString(block)
	if err != nil {
		return genesis, err
	}

	switch b := block.(type) {
	case uint64:
		if b < genesis {
			return genesis, fmt.Errorf(GENESIS_ERROR, genesis)
		}

		return b, nil
	case string:
		if !util.IsEmpty(s.Geth) {
			return s.Geth.HashToNumber(b), nil
		}

		// from infura
		eH, err := eth.Header(b, s.Config.Api)
		if err != nil {
			return genesis, err
		}

		// convert ethHeader to darwinia header
		dH, err := eth.IntoDarwiniaEthHeader(&eH)
		if err != nil {
			return dH.Number, err
		}

		// Check hash empty response
		if util.IsEmpty(dH) {
			return genesis, fmt.Errorf("Empty block: %s", b)
		}

		// Check genesis by number
		if dH.Number < genesis {
			log.Error("Requesting block %v", dH.Number)
			return genesis, fmt.Errorf(GENESIS_ERROR, genesis)
		}

		return dH.Number, nil
	default:
		return genesis, fmt.Errorf("Invaild block param: %v", block)
	}
}

/**
 * GetEthHeaderByNumber
 */
func (s *Shadow) GetHeader(
	chain Chain,
	block interface{},
) (types.Header, error) {
	switch chain {
	default:
		num, err := s.checkGenesis(s.Config.Genesis, block)
		if err != nil {
			return types.Header{}, err
		}

		if !util.IsEmpty(s.Geth) {
			block := *s.Geth.Header(block)
			if !util.IsEmpty(block) {
				return block, nil
			}
		}

		return eth.Header(num, s.Config.Api)
	}

}

func (s *Shadow) GetHeaderWithProof(
	chain Chain,
	block interface{},
) (GetEthHeaderWithProofRawResp, error) {
	switch chain {
	default:
		num, err := s.checkGenesis(s.Config.Genesis, block)
		if err != nil {
			return GetEthHeaderWithProofRawResp{}, err
		}

		// Log the event
		log.Trace("Request block %v with proof...", num)

		// Fetch header from cache
		cache, err := FetchHeaderCache(s, num)
		if err != nil {
			return GetEthHeaderWithProofRawResp{}, err
		}

		err = cache.ApplyProof(s)
		if err != nil {
			return GetEthHeaderWithProofRawResp{}, err
		}

		rawResp, err := cache.IntoResp()
		if err != nil {
			return GetEthHeaderWithProofRawResp{}, err
		}

		return rawResp, nil
	}
}

/**
 * BatchEthHeaderWithProofByNumber
 */
func (s *Shadow) BatchHeaderWithProof(
	block uint64,
	batch int,
) ([]GetEthHeaderWithProofRawResp, error) {
	log.Trace("Batching blocks %v - %v...", block, block+uint64(batch))

	// Batch headers
	var nps []GetEthHeaderWithProofRawResp
	for i := 0; i < batch; i++ {
		np, err := s.GetHeaderWithProof(
			Ethereum,
			block+uint64(i),
		)

		if err != nil {
			return nps, err
		}

		nps = append(nps, np)
	}

	return nps, nil
}

/**
 * Get proposal headers
 */
func (s *Shadow) GetProposalHeaders(numbers []uint64) ([]GetEthHeaderWithProofRawResp, error) {
	var (
		phs []GetEthHeaderWithProofRawResp
	)

	log.Trace("Geting proposal block %v...", numbers)
	for _, i := range numbers {
		rawp, err := s.GetHeaderWithProof(
			Ethereum,
			uint64(i),
		)
		if err != nil {
			return phs, err
		}

		phs = append(phs, rawp)
	}

	return phs, nil
}

/**
 * Get proposal headers
 */
func (s *Shadow) GetReceipt(
	tx string,
) (resp GetReceiptResp, err error) {
	log.Trace("Geting ethereum receipt of tx %v...", tx)
	proof, hash, err := eth.GetReceipt(tx)
	if err != nil {
		return
	}

	resp.ReceiptProof = proof.Proof
	cache := EthHeaderWithProofCache{Hash: hash}
	err = cache.Fetch(s)
	if err != nil {
		return
	}

	cr, err := cache.IntoResp()
	resp.Header = cr.Header
	return
}
