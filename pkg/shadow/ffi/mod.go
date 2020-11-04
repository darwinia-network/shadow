package main

import "C"
import (
	"github.com/darwinia-network/shadow/pkg/shadow"
	"github.com/darwinia-network/shadow/pkg/shadow/eth"
	"github.com/darwinia-network/shadow/pkg/shadow/log"
	"strings"
)

var (
	CONFIG shadow.Config = shadow.Config{}
)

func init() {
	_ = CONFIG.Load()
}

//export Epoch
func Epoch(block uint64) bool {
	_, err := eth.Epoch(block, &CONFIG)
	return err == nil
}

//export Proof
func Proof(api string, number uint64) *C.char {
	header, err := eth.Header(api, number)
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
func Receipt(api string, tx string) (*C.char, *C.char, *C.char) {
	tx = "0x" + tx[2:]
	proof, _, err := eth.GetReceipt(api, tx)
	if err != nil {
		log.Error("%v", err)
		return C.CString(""), C.CString(""), C.CString("")
	}

	return C.CString(proof.Index), C.CString(proof.Proof), C.CString(proof.HeaderHash)
}

//export Import
func Import(datadir string, from int, to int) *C.char {
	geth, _ := eth.NewGeth(datadir)
	hashes := []string{}
	for n := from; n < to; n++ {
		header := geth.Header(uint64(n))
		if header == nil || (header.Time == 0 && n != 0) {
			log.Error("Import hash of header %d failed", n)
			return C.CString(strings.Join(hashes, ","))
		}
		if n&1000 == 0 {
			log.Info("Imported hash %d/%d", n, to)
		}
		hashes = append(hashes, header.Hash().String())
	}
	return C.CString(strings.Join(hashes, ","))
}

func main() {}
