package core

import (
	"encoding/json"
	"fmt"

	"github.com/darwinia-network/shadow/internal/eth"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/sqlite"
)

type EthHeaderWithProofCache struct {
	Hash   string `json:"hash"`
	Number uint64 `json:"number" gorm:"unique_index"`
	Header string `json:"eth_header"`
	Proof  string `json:"proof"`
	Root   string `json:"root" gorm:"DEFAULT:NULL"`
}

// Fetch Eth Header by number
func (c *EthHeaderWithProofCache) Fetch(shadow *Shadow) error {
	block, err := IndexHeaderFromDB(shadow.DB, c)

	if err != nil || util.IsEmpty(c.Header) || c.Header == "" {
		log.Trace("fetching block %v ...", block)
		_, *c, err = FetchHeader(shadow, block)
		if err != nil {
			return err
		}

		// Prints logs every 100 headers
		if c.Number > 0 && c.Number%100 == 0 {
			log.Info(
				"imported headers from #%v to #%v\n",
				c.Number-100,
				c.Number,
			)
		}
	}

	// Return resp
	return nil
}

/// The func should run after `Fetch`
func (c *EthHeaderWithProofCache) ApplyProof(shadow *Shadow) error {
	block, err := IndexHeaderFromDB(shadow.DB, c)

	// Check proof lock
	if err != nil || util.IsEmpty(c.Proof) || c.Proof == "" {
		log.Trace("Apply ethashproof to block %v...", c.Number)
		if shadow.Config.CheckLock(PROOF_LOCK) {
			return fmt.Errorf("Shadow service is busy now, please try again later")
		} else {
			err := shadow.Config.CreateLock(PROOF_LOCK, []byte(""))
			if err != nil {
				return err
			}
		}

		// Fetch EthHeader
		ethHeader, cache, err := FetchHeader(shadow, block)
		if err != nil {
			return err
		}

		*c = cache
		err = CreateProofCache(shadow, c, &ethHeader)
		if err != nil {
			return err
		}

	}
	return nil
}

// Convert EthHeader
func (c *EthHeaderWithProofCache) IntoResp() (GetEthHeaderWithProofRawResp, error) {
	var rResp GetEthHeaderWithProofRawResp
	header, proof := eth.DarwiniaEthHeader{}, []eth.DoubleNodeWithMerkleProof{}

	// Decode header
	err := json.Unmarshal([]byte(c.Header), &header)
	if err != nil {
		log.Error("Unmarshal eth header failed %v", c.Header)
		return rResp, err
	}

	// Decode proof
	if !util.IsEmpty(c.Proof) {
		err = json.Unmarshal([]byte(c.Proof), &proof)
		if err != nil {
			log.Error("Unmarshal eth header proof failed %v", c.Proof)
			return rResp, err
		}
	}

	// Construct resp
	return GetEthHeaderWithProofRawResp{
		Header: header,
		Proof:  proof,
		Root:   c.Root,
	}, nil
}

func (c *EthHeaderWithProofCache) FromResp(
	db *gorm.DB,
	resp GetEthHeaderWithProofRawResp,
) error {
	// Convert header to string
	header, err := json.Marshal(resp.Header)
	if err != nil {
		return err
	}

	proof, err := json.Marshal(resp.Proof)
	if err != nil {
		return err
	}

	db.Create(&EthHeaderWithProofCache{
		Hash:   resp.Header.Hash,
		Number: resp.Header.Number,
		Header: string(header),
		Proof:  string(proof),
	})

	return nil
}
