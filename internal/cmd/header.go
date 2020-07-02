package cmd

import (
	"encoding/json"
	"fmt"

	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/spf13/cobra"
)

var cmdHeader = &cobra.Command{
	Use:   "header [number]",
	Short: "Get eth block by number",
	Long:  "This command will use the config at `~/.darwinia/config.json`",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		shadow, err := core.NewShadow()
		util.Assert(err)

		header, err := shadow.GetHeader(
			core.Ethereum,
			args[0],
		)
		util.Assert(err)

		// Get the header string
		js, err := json.Marshal(header)
		util.Assert(err)
		fmt.Printf("%v\n", string(js))
	},
}
