package cmd

import (
	"fmt"

	"github.com/darwinia-network/darwinia.go/internal"
	"github.com/darwinia-network/darwinia.go/internal/core"
	"github.com/darwinia-network/darwinia.go/internal/rpc"
	"github.com/darwinia-network/darwinia.go/internal/util"
	"github.com/spf13/cobra"
)

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

		// Start service
		fmt.Printf("Shadow service start at %s\n", args[0])
		err = rpc.ServeHTTP(
			shadow,
			fmt.Sprintf(":%s", args[0]),
		)
		util.Assert(err)
	},
}
