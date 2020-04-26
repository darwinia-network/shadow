package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/darwinia-network/darwinia.go/util"
	"github.com/ethereum/go-ethereum/core/types"
)

func main() {
	// exit when args not pair
	if len(os.Args) != 2 {
		fmt.Printf("usage: proof <blockNumber>/<blockJSON>\n")
		os.Exit(0)
	}

	// get etherum header from os.Args
	header := types.Header{}

	// check should use args or fetch new block
	if strings.HasPrefix("{", os.Args[1]) {
		err := json.Unmarshal([]byte(os.Args[1]), &header)
		util.Assert(err)
	} else {
		num, err := strconv.ParseUint(os.Args[1], 10, 32)
		util.Assert(err)

		header, err = util.Header(num)
		util.Assert(err)
	}

	// proof header
	res, err := util.Proof(&header)
	util.Assert(err)

	// output string
	output, err := json.Marshal(res)
	util.Assert(err)

	// have to use this printf because the ethash
	// has default stdout
	fmt.Printf("Json output:\n\n")
	fmt.Printf("%s\n", output)
}
