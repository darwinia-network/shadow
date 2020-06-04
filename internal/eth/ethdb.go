package eth

import (
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethdb"
)

type Geth struct {
	Cache    int64
	Handles  int64
	DataDir  string
	Database ethdb.Database
}

func NewGeth(datadir string) Geth {
	db, err := ethdb.NewLDBDatabase(datadir, 768, 16)
	if err != nil {
		return Geth{}
	}

	return Geth{Database: db, DataDir: datadir}
}

func (g *Geth) GetBlock(block interface{}) types.Header {
	switch b := block.(type) {
	case uint64:
		return g.GetBlockByNumber(b)
	case string:
		return g.GetBlockByHash(b)
	default:
		return types.Header{}
	}
}

func (g *Geth) GetBlockByNumber(number uint64) types.Header {
	hash := rawdb.ReadCanonicalHash(g.Database, number)
	block := rawdb.ReadBlock(g.Database, hash, number)
	return *block.Header()
}

func (g *Geth) GetBlockByHash(hash string) types.Header {
	ethash := common.HexToHash(hash)
	number := rawdb.ReadHeaderNumber(g.Database, ethash)
	block := rawdb.ReadBlock(g.Database, ethash, *number)
	return *block.Header()
}
