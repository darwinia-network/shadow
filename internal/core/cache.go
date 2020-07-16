package core

import (
	"encoding/json"
	"fmt"
	"os"
	"os/user"
	"path"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/eth"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/sqlite"
)

// Same directory as `darwinia.js`
const DB_PATH = ".darwinia/cache/shadow.db"

// EthHeaderWithProof Cache
type EthHeaderWithProofCache struct {
	gorm.Model
	Hash   string `json:"hash"`
	Number uint64 `json:"number" gorm:"unique_index"`
	Header string `json:"eth_header"`
	Proof  string `json:"proof"`
	// MMR
	Pos  string `json:"pos"`
	Root string `json:"root" gorm:"DEFAULT:NULL"`
}

func (c *EthHeaderWithProofCache) Parse(block interface{}) error {
	switch b := block.(type) {
	case uint64:
		c.Number = b
	case string:
		c.Hash = b
	default:
		return fmt.Errorf("Invaild block param: %v", block)
	}

	return nil
}

// Save header to cache
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

/// The func should run after `Fetch`
func (c *EthHeaderWithProofCache) ApplyProof(
	config internal.Config,
	db *gorm.DB,
) error {
	var (
		ethHeader types.Header
		err       error
	)

	if util.IsEmpty(c.Number) && c.Number != 0 {
		log.Warn("Apply ethashproof to block %v failed", c.Number)
		return fmt.Errorf("Apply ethashproof to block %v failed", c.Number)
	} else if util.IsEmpty(c.Header) || c.Header == "" {
		log.Warn("Can not analyse the block %v", c)
		return fmt.Errorf("Can not analyse the block %v", c)
	}

	log.Trace("Apply ethashproof to block %v...", c.Number)
	// Check proof lock
	if util.IsEmpty(c.Proof) || c.Proof == "" {
		if config.CheckLock(PROOF_LOCK) {
			return fmt.Errorf("Shadow service is busy now, please try again later")
		} else {
			err := config.CreateLock(PROOF_LOCK, []byte(""))
			if err != nil {
				return err
			}
		}

		// Fetch EthHeader
		ethHeader, err = eth.Header(c.Number, config.Api)
		if err != nil {
			return err
		}

		// Proof header
		proof, err := eth.Proof(&ethHeader, config)
		if err != nil {
			return err
		}

		proofBytes, err := json.Marshal(proof.Format())
		if err != nil {
			return err
		}

		c.Proof = string(proofBytes)
		err = db.Model(&c).Where("number = ?", c.Number).Update("proof", c.Proof).Error
		if err != nil {
			return err
		}

		// Remove proof lock
		err = config.RemoveLock(PROOF_LOCK)
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

// Fetch Eth Header by number
func (c *EthHeaderWithProofCache) Fetch(
	config internal.Config,
	db *gorm.DB,
) error {
	// Get header from sqlite3
	var (
		err   error
		block interface{}
	)

	if !util.IsEmpty(c.Number) || c.Number == 0 {
		block = c.Number
		err = db.Where("number = ?", c.Number).Take(&c).Error
	} else if !util.IsEmpty(c.Hash) {
		block = c.Hash
		err = db.Where("hash = ?", c.Hash).Take(&c).Error
	}

	if err != nil || util.IsEmpty(c.Header) || c.Header == "" {
		log.Trace("Fetching block %v ...", block)
		ethHeader, err := eth.Header(block, config.Api)
		if err != nil {
			return err
		}

		header, err := eth.IntoDarwiniaEthHeader(ethHeader)
		if err != nil {
			return err
		}

		bytes, err := json.Marshal(header)
		if err != nil {
			return err
		}

		c.Header = string(bytes)
		c.Hash = ethHeader.Hash().Hex()
		c.Number = header.Number
		db.Create(&c)

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

// Connect to cache
func ConnectDb() (*gorm.DB, error) {
	usr, err := user.Current()
	if err != nil {
		log.Error("Can not find current os user")
	}

	// Check path exists
	cachePath := path.Join(usr.HomeDir, path.Dir(DB_PATH))
	if _, err = os.Stat(cachePath); os.IsNotExist(err) {
		err = os.MkdirAll(cachePath, 0700)
		if err != nil {
			log.Error("Can not create cache folder at %s", cachePath)
		}
	}

	log.Info("Connecting database ~/%v...", DB_PATH)
	db, err := gorm.Open("sqlite3", path.Join(usr.HomeDir, DB_PATH))
	if err != nil {
		return db, err
	}

	// bootstrap sqlite3
	db.AutoMigrate(&EthHeaderWithProofCache{})
	return db, err
}
