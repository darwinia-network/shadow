package main

import (
	"github.com/darwinia-network/shadow/internal/cmd"
	"github.com/darwinia-network/shadow/internal/util"
)

func main() {
	util.Assert(cmd.Execute())
}
