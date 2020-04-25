package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/darwinia-network/darwinia.go/utils"
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
		utils.Assert(err)
	} else {
		num, err := strconv.ParseUint(os.Args[1], 10, 32)
		utils.Assert(err)

		header = utils.Header(num)
	}

	// proof header
	res, err := utils.Proof(&header)
	utils.Assert(err)

	// output string
	output, err := json.Marshal(res)
	utils.Assert(err)

	// have to use this printf because the ethash
	// has default stdout
	fmt.Printf("Json output:\n\n")
	fmt.Printf("%s\n", output)
}
