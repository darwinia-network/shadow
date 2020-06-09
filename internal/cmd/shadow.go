package cmd

import (
	"fmt"
	"time"

	"github.com/darwinia-network/darwinia.go/internal"
	"github.com/darwinia-network/darwinia.go/internal/core"
	"github.com/darwinia-network/darwinia.go/internal/rpc"
	"github.com/darwinia-network/darwinia.go/internal/util"
	"github.com/spf13/cobra"
)

func fetch(shadow *core.Shadow, genesis uint64) {
	var n uint64 = genesis
	for n > genesis {
		var _dimmy core.GetEthHeaderByNumberResp
		err := shadow.GetEthHeaderByNumber(core.GetEthHeaderByNumberParams{
			Number: n,
		}, &_dimmy)

		if err != nil {
			fmt.Println(fmt.Errorf("Get block failed, sleep for 1 min"))
			time.Sleep(60 * time.Second)
		}
		n++
	}
}

var cmdShadow = &cobra.Command{
	Use:   "shadow [port]",
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
