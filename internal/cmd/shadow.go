package cmd

//#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lmmr -ldl
//#include <stdint.h>
//extern int32_t run();
import "C"
import (
	"fmt"
	"log"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/rpc"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

func fetch(shadow *core.Shadow, genesis uint64) {
	// run mmr service
	go C.run()

	// fetcher
	ptr := core.EthHeaderWithProofCache{Number: genesis}
	for ptr.Number >= genesis {
		err := ptr.Fetch(shadow.Config, shadow.DB)
		if err != nil {
			log.Printf(
				"fetch header %v failed\n",
				ptr.Number,
			)
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
	Run: func(cmd *cobra.Command, args []string) {
		if len(args) == 0 {
			args = []string{"3000"}
		}

		// Generate config
		conf := new(internal.Config)
		err := conf.Load()
		util.Assert(err)

		// Generate Shadow
		shadow := new(core.Shadow)
		shadow.Config = *conf
		shadow.DB, err = core.ConnectDb()
		util.Assert(err)

		// if need fetch
		if Fetch {
			go fetch(shadow, conf.Genesis)
		}

		// Start service
		fmt.Printf("Shadow service start at %s\n", args[0])
		err = rpc.ServeHTTP(
			shadow,
			fmt.Sprintf(":%s", args[0]),
		)
		util.Assert(err)
	},
}
