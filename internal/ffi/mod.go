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
func Proof(number uint64) *C.char {
	header, err := eth.Header(number, CONFIG.Api)
	if err != nil {
		return C.CString("")
	}

	proof, err := eth.Proof(&header, &CONFIG)
	if err != nil {
		return C.CString("")
	}

	return C.CString(eth.EncodeProofArray(proof.Format()))
}

//export Receipt
func Receipt(tx string) (*C.char, *C.char) {
	proof, hash, err := eth.GetReceipt(tx)
	if err != nil {
		return C.CString(""), C.CString("")
	}

	return C.CString(proof.Proof), C.CString(hash)
}

func main() {}
