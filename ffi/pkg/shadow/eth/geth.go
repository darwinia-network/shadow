package eth

import (
	"path"

	"github.com/darwinia-network/shadow/ffi/pkg/shadow/util"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethdb"
)

type Geth struct {
	db ethdb.Database
}

func NewGeth(datadir string) (Geth, error) {
	if util.IsEmpty(datadir) {
		return Geth{}, nil
	}

	db, err := rawdb.NewLevelDBDatabaseWithFreezer(
		datadir,
		768,
		16,
		path.Join(datadir, "chaindata/ancient"),
		"",
	)

	return Geth{db}, err
}

func (g *Geth) HashToNumber(h string) uint64 {
	return *rawdb.ReadHeaderNumber(g.db, common.BytesToHash(common.FromHex(h)))
}

func (g *Geth) Header(block interface{}) *types.Header {
	block, err := util.NumberOrString(block)
	if err != nil || g.db == nil {
		return &types.Header{}
	}

	switch b := block.(type) {
	case string:
		hash := common.BytesToHash(common.FromHex(b))
		return g.Header(rawdb.ReadHeaderNumber(g.db, hash))
	case uint64:
		block := rawdb.ReadBlock(
			g.db,
			rawdb.ReadCanonicalHash(g.db, b),
			b,
		)
		if util.IsEmpty(block) {
			return &types.Header{}
		}
		return block.Header()
	default:
		return &types.Header{}
	}
}
