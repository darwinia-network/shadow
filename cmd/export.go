package cmd

import (
	"fmt"
	"github.com/spf13/cobra"

	"github.com/darwinia-network/shadow/internal/core"
)

// func init() {
// 	cmdExport.PersistentFlags().StringVarP(
// 		&PATH,
// 		"path",
// 		"p",
// 		".",
// 		"The export path",
// 	)
// }

var cmdExport = &cobra.Command{
	Use:   "export [path]",
	Short: "Export Shadow Database",
	Long:  "Export shadow database to path",
	Args:  cobra.MinimumNArgs(0),
	Run: func(cmd *cobra.Command, args []string) {
		shadow, _ := core.NewShadow()
		var blocks []core.EthHeaderWithProofCache
		shadow.DB.Find(&blocks)
		fmt.Println(blocks)
	},
}
