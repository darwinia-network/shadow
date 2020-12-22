package main

//#include <stdlib.h>
//typedef int (*get_header) (char*, void*);
//static int get_header_wrapper(get_header f, char* x, void* ctx) {
//    return f(x, ctx);
//}
import "C"

import (
    "github.com/darwinia-network/shadow/pkg/shadow"
    "github.com/darwinia-network/shadow/pkg/shadow/eth"
    "github.com/darwinia-network/shadow/pkg/log"
    "unsafe"
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
        log.Error("get ethashproof when get header failed %v", err)
        return C.CString("")
    }

    proof, err := eth.Proof(&header, &CONFIG)
    if err != nil {
        log.Error("get ethashproof when get proof failed %v", err)
        return C.CString("")
    }

    return C.CString(eth.EncodeProofArray(proof.Format()))
}

//export Import
func Import(datadir string, from int, to int, batch int, callback unsafe.Pointer, arg unsafe.Pointer) bool {
    f := C.get_header(callback)
    geth, err := eth.NewGeth(datadir)
    hashes := make([]string, 0)
    // the whole import process is split into several batches
    // each batch we process a number of `batch` blocks which saved in array `hashes`, and deliver it to callback
    // the hashes should be cleared for next batch
    for n := from; n < to; n++ {
        header := geth.Header(uint64(n))
        if header == nil || (header.Time == 0 && n != 0) {
            log.Error("Import hash of header %d failed err %v", n, err)
            return false
        }
        hashes = append(hashes, header.Hash().String())

        if (n - from + 1) % batch == 0 {
            log.Info("Imported hash %d/%d", n, to)
            hashstring := C.CString(strings.Join(hashes, ","));
            ret := C.get_header_wrapper(f, hashstring, arg)
            C.free(unsafe.Pointer(hashstring))
            if ret == 0 {
                return false
            }
            hashes = make([]string, 0)
        }
    }
    if len(hashes) > 0 {
        hashstring := C.CString(strings.Join(hashes, ","));
        defer C.free(unsafe.Pointer(hashstring))
        return C.get_header_wrapper(f, hashstring, arg) != 0
    }
    return true
}

//export Free
func Free(pointer unsafe.Pointer) {
    C.free(pointer);
}

func main() {}
