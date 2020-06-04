package cmd

import (
	"encoding/json"
	"fmt"
	"strconv"

	"github.com/darwinia-network/darwinia.go/internal"
	"github.com/darwinia-network/darwinia.go/internal/eth"
	"github.com/darwinia-network/darwinia.go/internal/util"
	"github.com/spf13/cobra"
)

var cmdHeader = &cobra.Command{
	Use:   "header [number]",
	Short: "Get eth block by number",
	Long:  "This command will use the config at `~/.darwinia/config.json`",
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		conf := new(internal.Config)
		err := conf.Load()
		util.Assert(err)

		// parse block number
		block, err := strconv.ParseUint(args[0], 10, 64)
		util.Assert(err)

		// get header
		header, err := eth.Header(block, conf.Api)
		util.Assert(err)

		// get the header string
		js, err := json.Marshal(header)
		util.Assert(err)
		fmt.Printf("%v\n", string(js))
	},
}
