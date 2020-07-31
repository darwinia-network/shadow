package cmd

import (
	"encoding/json"
	"fmt"

	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

var cmdReceipt = &cobra.Command{
	Use:   "receipt [tx]",
	Short: "Get receipt by tx hash",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		shadow, err := core.NewShadow()
		util.Assert(err)

		receipt, err := shadow.GetReceipt(args[0])
		util.Assert(err)

		// output receipt
		js, err := json.Marshal(receipt)
		util.Assert(err)

		fmt.Printf("%s\n", js)
	},
}
