package cmd

import (
	"fmt"
	"strings"

	"github.com/darwinia-network/shadow/api"
	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/ffi"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func init() {
	cmdRun.PersistentFlags().StringArrayVarP(
		&INFURA_KEYS,
		"infura_keys",
		"k",
		[]string{},
		"mutiple infura keys",
	)

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

func fetch(shadow *core.Shadow) {
	api := 0
	ptr := core.EthHeaderWithProofCache{Number: shadow.Config.Genesis}
	for ptr.Number >= shadow.Config.Genesis {
		err := ptr.Fetch(shadow.Config, shadow.DB)
		if err != nil {
			log.Error("fetch header %v failed\n", ptr.Number)
			if strings.Contains(
				strings.ToLower(fmt.Sprintf("%v", err)),
				// TODO: The real error string
				"infura",
			) && api < len(INFURA_KEYS)-1 {
				api += 1
				shadow.Config.Api = internal.ParseKey(INFURA_KEYS[api])
			}
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
			go fetch(&shadow)
		}

		// if trigger MMR
		if MMR {
			go ffi.RunMMR()
		}

		log.Info("Shadow HTTP service start at %s", HTTP)
		api.Swagger(&shadow, HTTP)
	},
}
