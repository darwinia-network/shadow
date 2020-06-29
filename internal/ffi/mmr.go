package ffi

//#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lmmr -ldl
//#include <stdint.h>
//extern int32_t run();
import "C"

func RunMMR() {
	go C.run()
}
