package cmd

import (
	"encoding/json"
	"fmt"
	"strconv"

	"github.com/darwinia-network/darwinia.go/util"
	"github.com/spf13/cobra"
)

var cmdHeader = &cobra.Command{
	Use:   "header [number]",
	Short: "Get eth block by number",
	Long:  "This command will use the config at `~/.darwinia/config.json`",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		// parse block number
		block, err := strconv.ParseUint(args[0], 10, 64)
		util.Assert(err)

		// get header
		header, err := util.Header(block)
		util.Assert(err)

		// get the header string
		js, err := json.Marshal(header)
		util.Assert(err)
		fmt.Printf("%v\n", string(js))
	},
}
