package cmd

import (
	"fmt"
	"runtime"
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

func fetchRoutine(api *int, shadow *core.Shadow, ptr uint64, ch chan int) {
	defer func() { _ = recover() }()
	cache := core.EthHeaderWithProofCache{Number: ptr}
	err := cache.Fetch(shadow.Config, shadow.DB)
	if err != nil {
		log.Error("fetch header %v failed\n", ptr)
		if strings.Contains(
			strings.ToLower(fmt.Sprintf("%v", err)),
			"infura",
		) {
			if *api < len(INFURA_KEYS)-1 {
				*api += 1
			} else {
				*api = 0
			}

			shadow.Config.Api = internal.ParseKey(INFURA_KEYS[*api])
		}

		fetchRoutine(api, shadow, ptr, ch)
	}

	<-ch
}

func fetch(shadow *core.Shadow) {
	// set channel
	runtime.GOMAXPROCS(runtime.NumCPU())
	ch := make(chan int, 300)

	api := 0
	for ptr := shadow.Config.Genesis; ; ptr++ {
		ch <- 1
		go fetchRoutine(&api, shadow, ptr, ch)
	}
}

var cmdRun = &cobra.Command{
	Use:   "run [port]",
	Short: "Start shadow service",
	Long:  "The main command of shadow service, lots of avaiable flags",
	Args:  cobra.MinimumNArgs(0),
	Run: func(cmd *cobra.Command, _ []string) {
		verboseCheck()

		// Generate Shadow
		shadow, err := core.NewShadow()
		util.Assert(err)
		shadow.DB.DB().SetMaxOpenConns(1)

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
