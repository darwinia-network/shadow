package core

import (
	// "github.com/ethereum/go-ethereum/common"
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

func (g *Geth) GetBlock(number uint64) Block {
	hash := rawdb.ReadCanonicalHash(g.Database, number)

	return Block{
		Number: number,
		Hash:   hash.Hex(),
	}
}
