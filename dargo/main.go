package main

import (
	"github.com/darwinia-network/darwinia.go/internal/cmd"
	"github.com/darwinia-network/darwinia.go/internal/util"
)

func main() {
	util.Assert(cmd.Execute())
}
