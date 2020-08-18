package cmd

import (
	"encoding/json"
	"fmt"

	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func init() {
	cmdProof.PersistentFlags().StringVarP(
		&PROOF_FORMAT,
		"format",
		"f",
		"json",
		"set port of http rpc server",
	)

	cmdProof.PersistentFlags().BoolVarP(
		&VERBOSE,
		"verbose",
		"v",
		false,
		"Enable all shadow logs",
	)
}

var cmdProof = &cobra.Command{
	Use:   "proof [number]",
	Short: "Proof the block by number",
	Long:  "DANGEROUS! This cmd will fill up your screen!",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		verboseCheck()

		// new shadow service
		shadow, err := core.NewShadow()
		util.Assert(err)

		proof, err := shadow.GetHeaderWithProof(
			core.Ethereum,
			args[0],
		)
		util.Assert(err)

		var ret interface{} = proof
		if PROOF_FORMAT == "codec" {
			ret = proof.IntoCodec()
		}

		// toJSON
		js, err := json.Marshal(ret)
		util.Assert(err)
		fmt.Printf("%s\n", js)
	},
}
