package cmd

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/eth"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func init() {
	cmdImport.PersistentFlags().StringVarP(
		&PATH,
		"path",
		"p",
		".",
		"The export path",
	)

	cmdImport.PersistentFlags().StringVarP(
		&NAME,
		"name",
		"n",
		"shadow.blocks",
		"The database export name",
	)
}

var cmdImport = &cobra.Command{
	Use:   "import",
	Short: "Import Shadow blocks",
	Long:  "Import Shadow blocks from file",
	Args:  cobra.MinimumNArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		shadow, err := core.NewShadow()
		util.Assert(err)

		hb, err := ioutil.ReadFile(
			fmt.Sprintf("%s/%s", PATH, NAME),
		)
		util.Assert(err)

		headers := strings.Split(string(hb), ",")

		var blocks []core.EthHeaderWithProofCache
		for _, h := range headers {
			dh := eth.DarwiniaEthHeader{}
			err = dh.De(h)
			util.Assert(err)

			header, err := json.Marshal(dh)
			util.Assert(err)

			blocks = append(blocks, core.EthHeaderWithProofCache{
				Number: dh.Number,
				Hash:   dh.Hash,
				Header: string(header),
			})
			util.Assert(err)
		}

		for _, b := range blocks {
			if shadow.DB.Model(&b).Where(
				"number = ?", b.Number,
			).Updates(&b).RowsAffected == 0 {
				shadow.DB.Create(&b)
			}
		}

		fmt.Printf("Imported %v blocks!\n", len(blocks))
	},
}
