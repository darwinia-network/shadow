package cmd

import (
	"encoding/json"
	"fmt"
	"io/ioutil"

	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func init() {
	cmdExport.PersistentFlags().StringVarP(
		&PATH,
		"path",
		"p",
		".",
		"The export path",
	)

	cmdExport.PersistentFlags().StringVarP(
		&NAME,
		"name",
		"n",
		"shadow.blocks",
		"The database export name",
	)
}

type Blocks struct {
	Blocks []core.EthHeaderWithProofCache
}

var cmdExport = &cobra.Command{
	Use:   "export",
	Short: "Export Shadow Database",
	Long:  "Export shadow database to path",
	Args:  cobra.MinimumNArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		shadow, err := core.NewShadow()
		util.Assert(err)

		var blocks []core.EthHeaderWithProofCache
		shadow.DB.Find(&blocks)

		for _, b := range blocks {
			b.Proof = ""
		}

		// Convert blocks to header
		bytes, err := json.Marshal(blocks)
		util.Assert(err)

		err = ioutil.WriteFile(fmt.Sprintf("%s/%s", PATH, NAME), bytes, 0644)
		util.Assert(err)
	},
}
