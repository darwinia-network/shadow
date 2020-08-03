package cmd

import (
	"fmt"
	"io/ioutil"
	"strings"

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

		var headers []string
		for _, b := range blocks {
			raw, err := b.IntoResp()
			util.Assert(err)

			headers = append(headers, raw.Header.Ser())
		}

		err = ioutil.WriteFile(
			fmt.Sprintf("%s/%s", PATH, NAME),
			[]byte(strings.Join(headers, ",")),
			0644,
		)
		util.Assert(err)

		fmt.Printf("Exported %v blocks!\n", len(headers))
	},
}
