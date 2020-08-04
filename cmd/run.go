package cmd

import (
	"github.com/darwinia-network/shadow/api"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/ffi"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func init() {
	cmdRun.PersistentFlags().BoolVarP(
		&FETCH,
		"fetch",
		"f",
		false,
		"keep fetching blocks in background",
	)

	cmdRun.PersistentFlags().BoolVarP(
		&MMR,
		"mmr",
		"m",
		false,
		"trigger mmr service",
	)

	cmdRun.PersistentFlags().BoolVarP(
		&VERBOSE,
		"verbose",
		"v",
		false,
		"Enable all shadow logs",
	)

	cmdRun.PersistentFlags().StringVar(
		&HTTP,
		"http",
		"3000",
		"set port of http api server",
	)
}

const (
	GIN_MODE = "GIN_MODE"
)

func fetch(shadow *core.Shadow, genesis uint64) {
	ptr := core.EthHeaderWithProofCache{Number: genesis}
	for ptr.Number >= genesis {
		err := ptr.Fetch(shadow.Config, shadow.DB)
		if err != nil {
			log.Error("fetch header %v failed\n", ptr.Number)
			continue
		}

		ptr = core.EthHeaderWithProofCache{
			Number: ptr.Number + 1,
			Header: "",
		}
	}
}

var cmdRun = &cobra.Command{
	Use:   "run [port]",
	Short: "Start shadow service",
	Long:  "This command will use the config at `~/.darwinia/config.json`",
	Args:  cobra.MinimumNArgs(0),
	Run: func(cmd *cobra.Command, _ []string) {
		verboseCheck()

		// Generate Shadow
		shadow, err := core.NewShadow()
		util.Assert(err)

		// if need fetch
		if FETCH {
			go fetch(&shadow, shadow.Config.Genesis)
		}

		// if trigger MMR
		if MMR {
			go ffi.RunMMR()
		}

		log.Info("Shadow HTTP service start at %s", HTTP)
		api.Swagger(&shadow, HTTP)
	},
}
