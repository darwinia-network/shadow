package util

import (
	"os"
	"reflect"

<<<<<<< HEAD:pkg/shadow/util/assert.go
	"github.com/darwinia-network/shadow/pkg/shadow/log"
=======
	"github.com/darwinia-network/shadow/pkg/internal/log"
>>>>>>> next:pkg/internal/util/assert.go
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
		log.Error("%v", err)
		os.Exit(1)
	}
}

// Assert error and exit process
func AssertEmpty(v interface{}) {
	if IsEmpty(v) {
		log.Error("Empty interface")
	}
}
