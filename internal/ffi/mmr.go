package ffi

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lmmr -ldl
#include <inttypes.h>

extern int32_t run();
extern char* proof(uint64_t *leaves, int32_t len);
*/
import "C"

func RunMMR() {
	go C.run()
}

func ProofLeaves(leaves []uint64, len int) string {
	return C.GoString(C.proof((*C.uint64_t)(&leaves[0]), (C.int)(len)))
}
