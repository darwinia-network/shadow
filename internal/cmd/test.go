package cmd

import (
	"github.com/darwinia-network/shadow/internal/log"
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
		log.Info("hello, world")
	},
}
