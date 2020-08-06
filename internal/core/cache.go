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
	db, err := gorm.Open("sqlite3", fmt.Sprintf(
		"file:%s?cache=shared&mode=rwc&_busy_timeout=9999999&_journal_mode=WAL",
		path.Join(usr.HomeDir, DB_PATH)),
	)
	if err != nil {
		return db, err
	}

	// bootstrap sqlite3
	db.AutoMigrate(&EthHeaderWithProofCache{})
	return db, err
}

func CountCache(db *gorm.DB) uint64 {
	var count uint64
	db.Model(&EthHeaderWithProofCache{}).Count(&count)
	return count
}

func FetchHeader(shadow *Shadow, block interface{}) (types.Header, error) {
	num, err := shadow.checkGenesis(shadow.Config.Genesis, block)
	if err != nil {
		return types.Header{}, err
	}

	if !util.IsEmpty(shadow.Geth) {
		block := *shadow.Geth.Header(block)
		if !util.IsEmpty(block) {
			return block, nil
		}
	}

	return eth.Header(num, shadow.Config.Api)
}

func CreateEthHeaderCache(db *gorm.DB, header types.Header) error {
	if util.IsEmpty(header) {
		return fmt.Errorf("empty header")
	}

	dh, err := eth.IntoDarwiniaEthHeader(header)
	if err != nil {
		return err
	}

	hstr, err := dh.ToString()
	if err != nil {
		return err
	}

	cache := EthHeaderWithProofCache{
		Hash:   header.Hash().Hex(),
		Number: header.Number.Uint64(),
		Header: hstr,
	}

	if err := db.Exec(UpsertHeaderSQL(&cache)).Error; err != nil {
		return err
	}

	return nil
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
	).Update(
		"proof", cache.Proof,
	).Error

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

func IndexHeaderFromDB(db *gorm.DB, cache *EthHeaderWithProofCache) (interface{}, error) {
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
