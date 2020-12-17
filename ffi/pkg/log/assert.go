package log

import (
	"os"
	"reflect"
<<<<<<< HEAD:ffi/pkg/shadow/util/assert.go

	"github.com/darwinia-network/shadow/ffi/pkg/shadow/log"
=======
>>>>>>> master:ffi/pkg/log/assert.go
)

// Check if is empty
func IsEmpty(x interface{}) bool {
	if x == nil {
		return true
	}
	return reflect.DeepEqual(x, reflect.Zero(reflect.TypeOf(x)).Interface())
}

// Assert error and exit process
func Assert(err error) {
	if err != nil {
		Error("assert %v", err)
		os.Exit(1)
	}
}

// Assert error and exit process
func AssertEmpty(v interface{}) {
	if IsEmpty(v) {
		Error("Empty interface")
	}
}
