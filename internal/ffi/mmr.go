package ffi

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lmmr -ldl
#include <inttypes.h>

extern int32_t run();
extern char* proof(uint64_t last_leaf, uint64_t *members, int32_t len);
*/
import "C"

func RunMMR() {
	C.run()
}

func ProofLeaves(last_leaf uint64, members []uint64, len int) string {
	return C.GoString(
		C.proof(
			(C.uint64_t)(last_leaf),
			(*C.uint64_t)(&members[0]),
			(C.int)(len)))
}
