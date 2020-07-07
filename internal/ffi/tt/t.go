package main

import (
	"fmt"
	"github.com/darwinia-network/shadow/internal/ffi"
)

func main() {
	fmt.Printf("%v", ffi.ProofLeaves([]uint64{1, 2}, 2))
}
