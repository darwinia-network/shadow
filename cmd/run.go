package cmd

import (
	"runtime"
	"sync"
	"time"

	"github.com/darwinia-network/shadow/api"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/ffi"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func init() {
	cmdRun.PersistentFlags().Uint32VarP(
		&LIMITS,
		"limits",
		"l",
		300,
		"handle blocks per second",
	)

	cmdRun.PersistentFlags().Int64VarP(
		&CHANNELS,
		"channels",
		"r",
		10,
		"goroutine channel conunts",
	)

	cmdRun.PersistentFlags().BoolVarP(
		&NOFETCH,
		"no-fetch",
		"",
		false,
		"doesn't fetch blocks",
	)

	cmdRun.PersistentFlags().BoolVarP(
		&NOAPI,
		"no-api",
		"",
		false,
		"doesn't start api server",
	)

	cmdRun.PersistentFlags().BoolVarP(
		&CHECK,
		"check",
		"c",
		false,
		"fetch headers from block 0, check all blocks exists",
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

func fetchRoutine(shadow *core.Shadow, ptr uint64, mutex *sync.Mutex) {
	mutex.Lock()
	_, err := core.FetchHeaderCache(shadow, ptr)
	for err != nil {
		log.Warn("fetch header %v failed: %v, refetching after 10s...", ptr, err)
		time.Sleep(10 * time.Second)
		_, err = core.FetchHeaderCache(shadow, ptr)
	}
	mutex.Unlock()
}

func fetch(shadow *core.Shadow, channels chan struct{}) {
	var (
		base  uint64      = shadow.Config.Genesis
		mutex *sync.Mutex = &sync.Mutex{}
	)
	if !CHECK {
		count := core.CountCache(shadow.DB)
		if count == 0 {
			base = count
		} else {
			base = count - 1
		}
		log.Info("current ethereum block height: %v", base)
	}

	gap := 1 * time.Second / time.Duration(int64(LIMITS))
	for ptr := base; ; ptr++ {
		channels <- struct{}{}
		fetchRoutine(shadow, ptr, mutex)
		time.Sleep(gap)
		<-channels
	}
}

var cmdRun = &cobra.Command{
	Use:   "run",
	Short: "Start shadow service",
	Long:  "The main command of shadow service, lots of available flags",
	Args:  cobra.MinimumNArgs(0),
	Run: func(cmd *cobra.Command, _ []string) {
		verboseCheck()
		runtime.GOMAXPROCS(3)

		// Generate Shadow
		shadow, err := core.NewShadow()
		util.Assert(err)

		// Remove all locks
		err = shadow.Config.RemoveAllLocks()
		util.Assert(err)

		funcs := []func(){}

		// append swagger
		if !NOAPI {
			funcs = append(funcs, func() {
				log.Info("Shadow HTTP service start at %s", HTTP)
				api.Swagger(&shadow, HTTP)
			})
		}

		// if need fetch
		if !NOFETCH {
			channels := make(chan struct{}, CHANNELS)
			funcs = append(funcs, func() { fetch(&shadow, channels) })
		}

		// if trigger MMR
		if MMR {
			funcs = append(funcs, func() { ffi.RunMMR(CHANNELS, LIMITS) })
		}

		// run parallelize
		util.Parallelize(funcs)
	},
}
