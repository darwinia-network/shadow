package ffi

/*
#cgo LDFLAGS: -lmmr -lsqlite3
#include <inttypes.h>

extern int32_t run();
extern char* proof(uint64_t last_leaf, uint64_t member);
*/
import "C"

func RunMMR() {
	C.run()
}

func ProofLeaves(last_leaf uint64, member uint64) string {
	return C.GoString(C.proof((C.uint64_t)(last_leaf), (C.uint64_t)(member)))
}
