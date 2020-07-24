package cmd

import (
	"fmt"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/mock"
	"github.com/spf13/cobra"
)

var (
	ROOT   = "54d123c9f298ae54a9af7309fbc264499f70637cf6947d2390626f2d73b6de74"
	PROOFS = []string{
		"7a0bf9a39dc552cf4ce072783c571a2a76b675fb96caf14254fca2c59f66dc3b",
		"480ff3f8a495b764e4361a6c2e296f34e8721cf1ec54fe5c46827937353bf118",
		"c0b665675897534496760f178673f8c563f68f7813f858ba00739933b528ca73",
	}
)

func init() {
	cmdTest.PersistentFlags().BoolVarP(
		&VERBOSE,
		"verbose",
		"v",
		false,
		"Enable all shadow logs",
	)
}

var cmdTest = &cobra.Command{
	Use:   "test",
	Short: "Test command",
	Long:  "This command is for tests, will not port in production",
	Run: func(cmd *cobra.Command, args []string) {
		verboseCheck()
		conf := internal.Config{}
		_ = conf.Load()

		h, _ := mock.Proposal(uint64(21), conf)
		h.Root = ROOT

		h.IntoProposalCodecWithExProof(PROOFS)
		fmt.Println(h.IntoProposalCodecWithExProof(PROOFS))
	},
}
