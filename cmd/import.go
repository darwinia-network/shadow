package cmd

import (
	"fmt"
	"os"
	"runtime"
	"strings"
	"time"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/log"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func init() {
	cmdImport.PersistentFlags().BoolVarP(
		&VERBOSE,
		"verbose",
		"v",
		false,
		"Enable all shadow logs",
	)

	cmdImport.PersistentFlags().Uint64VarP(
		&PERTX,
		"pertx",
		"p",
		1000,
		"blocks per transaction",
	)

	cmdImport.PersistentFlags().Uint64VarP(
		&LIMITS,
		"limits",
		"l",
		1000000,
		"block limits",
	)

	cmdImport.PersistentFlags().IntVarP(
		&CHANNELS,
		"channels",
		"c",
		300,
		"go channels",
	)
}

var cmdImport = &cobra.Command{
	Use:   "import <path>",
	Short: "Import Shadow blocks",
	Long:  "Import Shadow blocks from leveldb",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		verboseCheck()
		runtime.GOMAXPROCS(runtime.NumCPU())
		ch := make(chan int, CHANNELS)

		// Set env
		os.Setenv(internal.GETH_DATADIR, args[0])
		shadow, err := core.NewShadow()
		util.Assert(err)

		shadow.DB.DB().SetMaxOpenConns(1)
		// Fetch headers
		for b := core.CountCache(shadow.DB); b < LIMITS; b++ {
			defer func() { _ = recover() }()
			ch <- 1
			go importBlock(&shadow, b, ch)
		}
	},
}

func importBlock(shadow *core.Shadow, block uint64, ch chan int) {
	header := shadow.Geth.Header(block)
	defer func() { _ = recover() }()
	if util.IsEmpty(header) {
		log.Warn("fetch block %v from leveldb faield, sleep 10s", block)
		time.Sleep(time.Second * 10)
	}

	err := core.CreateEthHeaderCache(shadow.DB, *header)
	if err != nil {
		log.Warn("save eth block %v to shadowdb failed, retry", block)
		importBlock(shadow, block, ch)
	}

	bs := fmt.Sprintf(
		"%s%v",
		strings.Repeat(
			" ",
			len(fmt.Sprintf("%v", LIMITS))-len(fmt.Sprintf("%v", block)),
		),
		block,
	)
	log.Info("Imported block %v/%v", bs, LIMITS)
	<-ch
}
