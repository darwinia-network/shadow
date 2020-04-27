package main

import (
	"github.com/darwinia-network/darwinia.go/cmd"
	"github.com/darwinia-network/darwinia.go/util"
)

func main() {
	err := cmd.Execute()
	util.Assert(err)
}
