package main

import (
	"log"

	"github.com/darwinia-network/darwinia.go/core"
	"github.com/darwinia-network/darwinia.go/lib"
)

func main() {
	log.Fatal(lib.Serve(new(core.Shadow), ":3000"))
}
