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
	Fetch bool
)

// Init commands to dargo
func init() {
	cmdRun.PersistentFlags().BoolVarP(
		&Fetch,
		"fetch",
		"f",
		false,
		"keep fetching blocks in background",
	)

	rootCmd.AddCommand(
		cmdEpoch,
		cmdHeader,
		cmdProof,
		cmdReceipt,
		cmdRun,
		cmdVersion,
	)
}

// Execute the command
func Execute() error {
	return rootCmd.Execute()
}