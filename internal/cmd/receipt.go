package cmd

import (
	"fmt"

	"github.com/darwinia-network/shadow/internal/eth"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

var cmdReceipt = &cobra.Command{
	Use:   "receipt [tx]",
	Short: "Get receipt by tx hash",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		r, err := eth.GetReceiptLog(args[0])
		util.Assert(err)

		proof, err := eth.BuildProofRecord(r)
		util.Assert(err)

		// output receipt
		fmt.Printf("%s\n", proof)
	},
}
