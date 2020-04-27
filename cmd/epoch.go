package cmd

import (
	"fmt"
	"strconv"

	"github.com/darwinia-network/darwinia.go/util"
	"github.com/spf13/cobra"
	"github.com/tranvictor/ethashproof"
)

var cmdEpoch = &cobra.Command{
	Use:   "epoch [number]",
	Short: "Calculate epoch cache",
	Long:  "This will take a long time",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		// parse epoch
		epoch, err := strconv.ParseUint(args[0], 10, 64)
		util.Assert(err)

		// calculating
		root, err := ethashproof.CalculateDatasetMerkleRoot(epoch, true)
		util.Assert(err)

		// output
		fmt.Printf("Root: %s\n", root.Hex())
	},
}
