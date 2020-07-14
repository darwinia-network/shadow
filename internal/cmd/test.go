package cmd

import (
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/spf13/cobra"
)

var cmdTest = &cobra.Command{
	Use:   "test",
	Short: "Test command",
	Long:  "This command is for tests, will not port in production",
	Run: func(cmd *cobra.Command, args []string) {
		log.Info("hello, world")
	},
}
