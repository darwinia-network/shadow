package cmd

import (
	"encoding/json"
	"fmt"

	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

var cmdProof = &cobra.Command{
	Use:   "proof [number]",
	Short: "Proof the block by number",
	Long:  "DANGEROUS! This cmd will fill up your screen!",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		shadow, err := core.NewShadow()
		util.Assert(err)

		proof, err := shadow.GetHeaderWithProof(
			core.Ethereum,
			args[0],
		)
		util.Assert(err)

		var ret interface{} = proof
		if PROOF_FORMAT == "codec" {
			ret = proof.IntoCoedc()
		}

		// toJSON
		js, err := json.Marshal(ret)
		util.Assert(err)
		fmt.Printf("%s\n", js)
	},
}
