package core

import (
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
	Number uint64                           `json:"number" gorm:"unique_index"`
	Header util.DarwiniaEthHeader           `json:"eth_header"`
	Proof  []util.DoubleNodeWithMerkleProof `json:"proof"`
}

func (c *EthHeaderWithProofCache) FromResp(resp GetEthHeaderWithProofByNumberRawResp) error {
	db, err := ConnectDb()
	if err != nil {
		return err
	}

	// Insert cache into sqlite3
	defer db.Close()
	db.Create(&EthHeaderWithProofCache{
		Number: resp.Header.Number,
		Header: resp.Header,
		Proof:  resp.Proof,
	})

	// Return nil
	return nil
}

func (c *EthHeaderWithProofCache) IntoResp() GetEthHeaderWithProofByNumberRawResp {
	return GetEthHeaderWithProofByNumberRawResp{
		c.Header,
		c.Proof,
	}
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
	err = db.Take(&c).Error
	if err != nil {
		return c.IntoResp(), err
	}

	// Return resp
	return c.IntoResp(), nil
}

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
