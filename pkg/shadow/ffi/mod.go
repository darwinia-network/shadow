package main

import "C"
import (
	"github.com/darwinia-network/shadow/pkg/shadow"
	"github.com/darwinia-network/shadow/pkg/shadow/eth"
	"strings"
)

var (
	CONFIG shadow.Config = shadow.Config{}
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
func Receipt(tx string) (*C.char, *C.char, *C.char) {
	tx = "0x" + tx[2:]
	proof, _, err := eth.GetReceipt(tx)
	if err != nil {
		return C.CString(""), C.CString(""), C.CString("")
	}

	return C.CString(proof.Index), C.CString(proof.Proof), C.CString(proof.HeaderHash)
}

//export Import
func Import(datadir string, limit int) string {
	geth, _ := eth.NewGeth(datadir)
	hashes := []string{}
	for n := 0; n < limit; n++ {
		header := geth.Header(n)
		if header == nil {
			return strings.Join(hashes, ",")
		}
		hashes = append(hashes, header.Hash().String())
	}
	return strings.Join(hashes, ",")
}

func main() {}
