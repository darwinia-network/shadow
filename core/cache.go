package core

import (
	"encoding/json"
	"log"
	"os/user"
	"path"

	"github.com/darwinia-network/darwinia.go/util"
	"github.com/jinzhu/gorm"
	_ "github.com/jinzhu/gorm/dialects/sqlite"
)

// Same directory as `darwinia.js`
const DB_PATH = ".darwinia/cache/shadow.db"

// EthHeaderWithProof Cache
type EthHeaderWithProofCache struct {
	gorm.Model
	Number uint64 `json:"number" gorm:"unique_index"`
	Header string `json:"eth_header"`
	Proof  string `json:"proof"`
}

// Save header to cache
func (c *EthHeaderWithProofCache) FromResp(resp GetEthHeaderWithProofByNumberRawResp) error {
	db, err := ConnectDb()
	if err != nil {
		return err
	}

	// Convert header to string
	header, err := json.Marshal(resp.Header)
	if err != nil {
		return err
	}

	proof, err := json.Marshal(resp.Proof)
	if err != nil {
		return err
	}

	defer db.Close()
	db.Create(&EthHeaderWithProofCache{
		Number: resp.Header.Number,
		Header: string(header),
		Proof:  string(proof),
	})

	// Return nil
	return nil
}

// Convert EthHeader
func (c *EthHeaderWithProofCache) IntoResp() (GetEthHeaderWithProofByNumberRawResp, error) {
	var rResp GetEthHeaderWithProofByNumberRawResp
	header, proof := util.DarwiniaEthHeader{}, []util.DoubleNodeWithMerkleProof{}

	// Decode header
	err := json.Unmarshal([]byte(c.Header), header)
	if err != nil {
		return rResp, err
	}

	// Decode proof
	err = json.Unmarshal([]byte(c.Proof), proof)
	if err != nil {
		return rResp, err
	}

	// Construct resp
	return GetEthHeaderWithProofByNumberRawResp{
		header,
		proof,
	}, nil
}

// Fetch Eth Header by number
func (c *EthHeaderWithProofCache) Fetch() (GetEthHeaderWithProofByNumberRawResp, error) {
	var resp GetEthHeaderWithProofByNumberRawResp
	db, err := ConnectDb()
	if err != nil {
		return resp, err
	}

	// Get header from sqlite3
	defer db.Close()
	err = db.Where("number = ?", c.Number).Take(&c).Error
	if err != nil {
		return resp, err
	}

	// Return resp
	return c.IntoResp()
}

// Connect to cache
func ConnectDb() (*gorm.DB, error) {
	usr, err := user.Current()
	if err != nil {
		log.Fatal("Can not find current os user")
	}

	db, err := gorm.Open("sqlite3", path.Join(usr.HomeDir, DB_PATH))
	if err != nil {
		return db, err
	}

	// bootstrap sqlite3
	db.AutoMigrate(&EthHeaderWithProofCache{})
	return db, err
}
