package cmd

import (
	"runtime"

	"github.com/darwinia-network/shadow/api"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/ffi"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func init() {
	cmdRun.PersistentFlags().IntVarP(
		&CHANNELS,
		"channels",
		"c",
		1,
		"channel counts",
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
	_, err := shadow.FetchHeaderCache(ptr)
	if err != nil {
		log.Error("fetch header %v failed %v\n", ptr, err)
	}

	<-ch
}

func fetch(shadow *core.Shadow) {
	// set channel
	runtime.GOMAXPROCS(runtime.NumCPU())
	ch := make(chan int, CHANNELS)

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
