package cmd

import (
	"github.com/spf13/cobra"
	"os"
)

var (
	rootCmd = &cobra.Command{
		Use:   "shadow",
		Short: "Darwinia shadow service",
		Long:  `The way to Go`,
	}
	FETCH        bool
	VERBOSE      bool
	HTTP         string
	PROOF_FORMAT string
	PATH         string
	NAME         string
	MMR          bool
	CHANNELS     int64
	LIMITS       uint64
	PERTX        uint64
	CHECK        bool
	GETH_DATADIR string
)

const (
	// Rust log env
	GO_LOG = "GO_LOG"
	// Rust log env
	RUST_LOG = "RUST_LOG"
)

// Init commands to dargo
func init() {
	rootCmd.AddCommand(
		cmdEpoch,
		cmdHeader,
		cmdProof,
		cmdReceipt,
		cmdRun,
		cmdVersion,
		cmdTest,
		cmdImport,
		cmdGeth,
	)
}

/// Enable all logs
func verboseCheck() {
	if VERBOSE {
		os.Setenv(GO_LOG, "ALL")
		os.Setenv(RUST_LOG, "mmr")
	}
}

// Execute the command
func Execute() error {
	return rootCmd.Execute()
}
