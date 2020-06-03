package main

import (
	"github.com/darwinia-network/darwinia.go/internal/cmd"
	"github.com/darwinia-network/darwinia.go/internal/util"
)

func main() {
	err := cmd.Execute()
	util.Assert(err)
}
