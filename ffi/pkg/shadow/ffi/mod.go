package main

//#include <stdlib.h>
//typedef int (*get_header) (const void*, int len, void*);
//static int get_header_wrapper(get_header f, const void* x, int len, void* ctx) {
//    return f(x, len, ctx);
//}
import "C"

import (
    "github.com/darwinia-network/shadow/ffi/pkg/shadow/config"
    "github.com/darwinia-network/shadow/ffi/pkg/shadow/eth"
    "github.com/darwinia-network/shadow/ffi/pkg/log"
    "unsafe"
    "runtime"
    "path/filepath"
)

const (
    reorgDistance = 0x40
)

var (
    ethproof *eth.EthashProof
)

func init() {
    conf := &config.EthProof {
        RootPath: "$HOME/.shadow",
        Limitepochsize: 5,
        LimitCPU: 3,
    }
    ethproof = eth.NewEthashProof(conf)
    runtime.GOMAXPROCS(conf.LimitCPU)
}

//export Start
func Start(epoch uint64) {
    ethproof.Start(epoch)
}

//export Stop
func Stop() {
    ethproof.Stop()
}

//export Epoch
func Epoch(blockno uint64) bool {
    return ethproof.NotifyEpoch(blockno)
}

//export EpochWait
func EpochWait(blockno uint64) bool {
    return ethproof.NotifyEpochWait(blockno) == nil
}

//export Proof
func Proof(api string, number uint64) *C.char {
    header, err := eth.Header(api, number)
    if err != nil {
        log.Error("get ethashproof when get header failed %v", err)
        return C.CString("")
    }

    _, proof, err := ethproof.Proof(&header, true)
    if err != nil {
        log.Error("get ethashproof when get proof failed %v", err)
        return C.CString("")
    }

    return C.CString(eth.EncodeProofArray(proof))
}

//export Receipt
func Receipt(api string, tx string) (*C.char, *C.char, *C.char) {
    tx = "0x" + tx[2:]
    proof, _, err := eth.GetReceipt(api, tx)
    if err != nil {
        log.Error("get receipt failed api %v, %v", api, err)
        return C.CString(""), C.CString(""), C.CString("")
    }

    return C.CString(proof.Index), C.CString(proof.Proof), C.CString(proof.HeaderHash)
}

//export Import
func Import(datadir string, genesis string, from int, to int, batch int, callback unsafe.Pointer, arg unsafe.Pointer) bool {
    f := C.get_header(callback)
    ar := eth.NewAncientReader(filepath.Join(datadir, "ancient"), "hashes", false)
    blockReader := eth.NewBlockHashReader(ar, datadir)
    if blockReader == nil {
        return false
    }
    if err := blockReader.CheckGenesis(genesis); err != nil {
        return false
    }
    hashes := make([]byte, 0)
    // the whole import process is split into several batches
    // each batch we process a number of `batch` blocks which saved in array `hashes`, and deliver it to callback
    // the hashes should be cleared for next batch
    if uint64(to) > blockReader.Head {
        log.Info("import to %v is too large, change to max number %v - %v", to, blockReader.Head, reorgDistance)
        to = int(blockReader.Head)
        if to > reorgDistance {
            to -= reorgDistance
        }
    }
    for n := from; n < to; n++ {
        hash, err := blockReader.Read(uint64(n))
        if err != nil {
            log.Error("Import hash of header %d failed err %v", n, err)
            return false
        }
        hashes = append(hashes, hash...)

        if (n - from + 1) % batch == 0 {
            log.Info("Imported hash %d/%d", n, to)
            ret := C.get_header_wrapper(f, unsafe.Pointer(&hashes[0]), C.int(len(hashes)), arg)
            if ret == 0 {
                return false
            }
            hashes = make([]byte, 0)
        }
    }
    if len(hashes) > 0 {
        return C.get_header_wrapper(f, unsafe.Pointer(&hashes[0]), C.int(len(hashes)), arg) != 0
    }
    return true
}

//export Free
func Free(pointer unsafe.Pointer) {
    C.free(pointer);
}

func main() {}
