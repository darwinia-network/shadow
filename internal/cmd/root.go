package cmd

import (
	"github.com/spf13/cobra"
)

var (
	rootCmd = &cobra.Command{
		Use:   "shadow",
		Short: "Darwinia shadow service",
		Long:  `The way to Go`,
	}
	FETCH bool
	HTTP  string
	RPC   string
)

// Init commands to dargo
func init() {
	cmdRun.PersistentFlags().BoolVarP(
		&FETCH,
		"fetch",
		"f",
		false,
		"keep fetching blocks in background",
	)

	cmdRun.PersistentFlags().StringVar(
		&HTTP,
		"http",
		"3001",
		"set port of http api server",
	)

	cmdRun.PersistentFlags().StringVar(
		&RPC,
		"rpc",
		"3000",
		"set port of http rpc server",
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
