package core

import (
	"github.com/darwinia-network/darwinia.go/internal"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/ethdb"
)

type Block struct {
	Number uint64
	Hash   string
}

type Geth struct {
	Cache    int64
	Handles  int64
	DataDir  string
	Database ethdb.Database
}

func New(datadir string) (Geth, error) {
	db, err := ethdb.NewLDBDatabase(datadir, 768, 16)
	if err != nil {
		return Geth{}, err
	}

	return Geth{Database: db, DataDir: datadir}, nil
}

func (g *Geth) GetBlock(number uint64) (internal.DarwiniaEthHeader, error) {
	hash := rawdb.ReadCanonicalHash(g.Database, number)
	block := rawdb.ReadBlock(g.Database, hash, number)
	return internal.IntoDarwiniaEthHeader(*block.Header())
}
