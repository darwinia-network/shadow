package core

import (
	"encoding/json"
	"fmt"
	"os"
	"os/user"
	"path"

	"github.com/darwinia-network/shadow/internal/eth"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/sqlite"
)

// Same directory as `darwinia.js`
const DB_PATH = ".darwinia/cache/shadow.db"

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
	db, err := gorm.Open("sqlite3",
		fmt.Sprintf(
			"file:%s?cache=shared&mode=rwc&_journal_mode=WAL",
			path.Join(usr.HomeDir, DB_PATH)),
	)

	if err != nil {
		return db, err
	}

	// Setings
	db.DB().SetMaxOpenConns(1)

	// bootstrap sqlite3
	db.AutoMigrate(&EthHeaderWithProofCache{})
	return db, err
}

func CountCache(db *gorm.DB) uint64 {
	var count uint64
	db.Model(&EthHeaderWithProofCache{}).Count(&count)
	return count
}

func FetchHeader(shadow *Shadow, block interface{}) (
	header types.Header,
	err error,
) {
	num, err := shadow.checkGenesis(shadow.Config.Genesis, block)
	if err != nil {
		return
	}

	if !util.IsEmpty(shadow.Geth) {
		log.Trace("Request block %v from leveldb...", num)
		dimHeader := shadow.Geth.Header(num)
		if !util.IsEmpty(dimHeader) {
			header = *dimHeader
		}
	}

	if util.IsEmpty(header) {
		log.Trace("Request block %v from infura api...", num)
		header, err = eth.Header(num, shadow.Config.Api)
		if err != nil {
			return
		}
	}

	_, err = CreateEthHeaderCache(shadow.DB, &header)
	return
}

func FetchHeaderCache(shadow *Shadow, block interface{}) (
	cache EthHeaderWithProofCache,
	err error,
) {
	var header types.Header
	num, err := shadow.checkGenesis(shadow.Config.Genesis, block)
	if err != nil {
		return
	}

	shadow.DB.Raw(SelectHeader(num)).Scan(&cache)
	if !util.IsEmpty(cache.Header) {
		log.Trace("Block %v exists in shadowdb...", block)
		return
	}

	if util.IsEmpty(cache.Header) && !util.IsEmpty(shadow.Geth) {
		log.Trace("Request block %v from leveldb...", block)
		dimHeader := shadow.Geth.Header(block)
		if !util.IsEmpty(dimHeader) {
			header = *dimHeader
		}
	}

	if util.IsEmpty(header) {
		log.Trace("Request block %v from infura api...", block)
		header, err = eth.Header(num, shadow.Config.Api)
		if err != nil {
			return
		}
	}

	cache, err = CreateEthHeaderCache(shadow.DB, &header)
	return
}

func CreateEthHeaderCache(
	db *gorm.DB,
	header *types.Header,
) (
	cache EthHeaderWithProofCache,
	err error,
) {
	if util.IsEmpty(header) {
		err = fmt.Errorf("empty header")
		return
	}

	dh, err := eth.IntoDarwiniaEthHeader(header)
	if err != nil {
		return
	}

	hstr, err := dh.ToString()
	if err != nil {
		return
	}

	cache = EthHeaderWithProofCache{
		Hash:   header.Hash().Hex(),
		Number: header.Number.Uint64(),
		Header: hstr,
	}

	if err = db.Exec(UpsertHeaderSQL(&cache)).Error; err != nil {
		return
	}

	return
}

func CreateProofCache(
	shadow *Shadow,
	cache *EthHeaderWithProofCache,
	header *types.Header,
) error {
	// Proof header
	proof, err := eth.Proof(header, shadow.Config)
	if err != nil {
		return err
	}

	proofBytes, err := json.Marshal(proof.Format())
	if err != nil {
		return err
	}

	cache.Proof = string(proofBytes)
	err = shadow.DB.Model(&cache).Where(
		"number = ?", cache.Number,
	).Updates(EthHeaderWithProofCache{
		Proof: cache.Proof,
		Root:  cache.Root,
	}).Error

	if err != nil {
		return err
	}

	// Remove proof lock
	err = shadow.Config.RemoveLock(PROOF_LOCK)
	if err != nil {
		return err
	}

	return nil
}

func IndexHeaderFromDB(
	db *gorm.DB,
	cache *EthHeaderWithProofCache,
) (interface{}, error) {
	var (
		err   error
		block interface{}
	)

	if !util.IsEmpty(cache.Number) || cache.Number == 0 {
		block = cache.Number
		err = db.Where("number = ?", cache.Number).Take(&cache).Error
	} else if !util.IsEmpty(cache.Hash) {
		block = cache.Hash
		err = db.Where("hash = ?", cache.Hash).Take(&cache).Error
	}

	return block, err
}
