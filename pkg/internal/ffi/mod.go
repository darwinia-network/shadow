package main

import "C"
import (
	"github.com/darwinia-network/shadow/pkg/internal"
	"github.com/darwinia-network/shadow/pkg/internal/eth"
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
	tx = "0x" + tx[2:]
	proof, hash, err := eth.GetReceipt(tx)
	if err != nil {
		return C.CString(""), C.CString("")
	}

	return C.CString(proof.Proof), C.CString(hash)
}

func main() {}
