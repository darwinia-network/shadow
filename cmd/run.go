package cmd

import (
	"os"
	"sync"
	"time"

	"github.com/darwinia-network/shadow/api"
	"github.com/darwinia-network/shadow/internal"
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
		"r",
		10,
		"goroutine channel conunts",
	)

	cmdRun.PersistentFlags().BoolVarP(
		&FETCH,
		"fetch",
		"f",
		false,
		"keep fetching blocks in background",
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

	cmdRun.PersistentFlags().StringVar(
		&GETH_DATADIR,
		"geth-datadir",
		"",
		"The datadir of geth",
	)
}

const (
	GIN_MODE = "GIN_MODE"
)

func fetchRoutine(shadow *core.Shadow, ptr uint64, mutex *sync.Mutex) {
	mutex.Lock()
	_, err := core.FetchHeaderCache(shadow, ptr)
	if err != nil {
		time.Sleep(10 * time.Second)
		_, _ = core.FetchHeaderCache(shadow, ptr)
		log.Warn("fetch header %v failed: %v, refetching after 10s...", ptr, err)
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
		if count == uint64(0) {
			base = 0
		} else {
			base = count + 1
		}
		log.Info("current ethereum block height: %v", base)
	}

	// defer func() { recover() }()
	for ptr := base; ; ptr++ {
		channels <- struct{}{}
		fetchRoutine(shadow, ptr, mutex)
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

		// Check if has geth-datadir
		if len(GETH_DATADIR) > 0 {
			log.Info("Geth dir is: %v", GETH_DATADIR)
			os.Setenv(internal.GETH_DATADIR, GETH_DATADIR)
		}

		// Generate Shadow
		shadow, err := core.NewShadow()
		util.Assert(err)

		// if need fetch
		if FETCH {
			channels := make(chan struct{}, CHANNELS)
			go fetch(&shadow, channels)
		}

		// if trigger MMR
		if MMR {
			go ffi.RunMMR(1)
		}

		log.Info("Shadow HTTP service start at %s", HTTP)
		api.Swagger(&shadow, HTTP)
	},
}
