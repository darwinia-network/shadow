package main

import "C"
import (
	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/eth"
)

var (
	CONFIG internal.Config = internal.Config{}
)

func init() {
	_ = CONFIG.Load()
}

//export Proof
func Proof(number uint64) string {
	header, _ := eth.Header(number, CONFIG.Api)
	proof, _ := eth.Proof(&header, &CONFIG)
	return eth.EncodeProofArray(proof.Format())
}

//export Receipt
func Receipt(tx string) (string, string) {
	proof, hash, _ := eth.GetReceipt(tx)
	return proof.Proof, hash
}

func main() {}
