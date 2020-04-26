package cmd

import (
	"fmt"

	"github.com/darwinia-network/darwinia.go/core"
	"github.com/darwinia-network/darwinia.go/lib"
	"github.com/darwinia-network/darwinia.go/util"
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

		fmt.Printf("Shadow service start at %s", args[0])
		err := lib.Serve(
			new(core.Shadow),
			fmt.Sprintf(":%s", args[0]),
		)
		util.Assert(err)
	},
}
