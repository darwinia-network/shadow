package cmd

import (
	"github.com/spf13/cobra"
)

var (
	rootCmd = &cobra.Command{
		Use:   "dargo",
		Short: "Darwinia.go cmd-tool",
		Long:  `The way to Go`,
	}
)

// Init commands to dargo
func init() {
	rootCmd.AddCommand(
		cmdEpoch,
		cmdHeader,
		cmdProof,
		cmdShadow,
		cmdVersion,
	)
}

// Execute the command
func Execute() error {
	return rootCmd.Execute()
}
