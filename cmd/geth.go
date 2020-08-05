package cmd

import (
	"fmt"

	"github.com/darwinia-network/shadow/internal/util"
	"github.com/ethereum/go-ethereum/core/rawdb"
	"github.com/ethereum/go-ethereum/ethdb"
	"github.com/spf13/cobra"
)

func init() {
	cmdGeth.PersistentFlags().BoolVarP(
		&VERBOSE,
		"verbose",
		"v",
		false,
		"Enable all shadow logs",
	)
}

var cmdGeth = &cobra.Command{
	Use:   "geth",
	Short: "Test Geth",
	Long:  "Try the blazling fast geth fast mode",
	Run: func(cmd *cobra.Command, args []string) {
		verboseCheck()
		var db ethdb.Database
		db, err := rawdb.NewLevelDBDatabaseWithFreezer(
			"/Volumes/Mercury/geth/geth/chaindata",
			768,
			16,
			"/Volumes/Mercury/geth/geth/chaindata/ancient",
			"",
		)
		util.Assert(err)

		fmt.Printf("%v\n", rawdb.ReadHeadHeaderHash(db))
		fmt.Printf("%v\n", rawdb.ReadCanonicalHash(db, 0))
		block, _ := rawdb.ReadBlock(
			db,
			rawdb.ReadCanonicalHash(db, 100),
			uint64(100),
		).Header().MarshalJSON()

		fmt.Printf("%v", string(block))
		// rawdb.ReadBlock(db ethdb.Reader, hash common.Hash, number uint64)
	},
}
