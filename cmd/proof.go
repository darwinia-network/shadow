package cmd

import (
	"encoding/json"
	"fmt"
	"strconv"

	"github.com/darwinia-network/darwinia.go/util"
	"github.com/spf13/cobra"
)

var cmdProof = &cobra.Command{
	Use:   "proof [number]",
	Short: "Proof the block by number",
	Long:  "DANGEROUS! This cmd will fill up your screen!",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		// parse block number
		block, err := strconv.ParseUint(args[0], 10, 64)
		util.Assert(err)

		// get header
		header, err := util.Header(block)
		util.Assert(err)

		// get proof
		proof, err := util.Proof(&header)
		util.Assert(err)

		// output string
		output, err := json.Marshal(proof)
		util.Assert(err)

		// have to use this printf because the ethash
		// has default stdout
		fmt.Printf("Json output:\n\n")
		fmt.Printf("%s\n", output)
	},
}
