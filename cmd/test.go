package cmd

import (
	"fmt"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/mock"
	"github.com/spf13/cobra"
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
		fmt.Println(h)
	},
}
