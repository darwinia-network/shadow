package cmd

import (
	// "os"

	"github.com/darwinia-network/shadow/internal/core"
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

	cmdImport.PersistentFlags().BoolVarP(
		&VERBOSE,
		"verbose",
		"v",
		false,
		"Enable all shadow logs",
	)
}

var cmdImport = &cobra.Command{
	Use:   "import",
	Short: "Import Shadow blocks",
	Long:  "Import Shadow blocks from file",
	Args:  cobra.MinimumNArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		verboseCheck()

		// Set env
		// os.Setenv("GETH_DATADIR", PATH)
		shadow, err := core.NewShadow()
		util.Assert(err)

		// Fetch headers
		for b := uint64(0); ; b++ {
			_, err := shadow.FetchHeaderCache(b)
			if err != nil {
				util.Assert(err)
			}
		}
	},
}
