package util

import (
	"log"
	"reflect"
)

// Check if is empty
func IsEmpty(x interface{}) bool {
	return reflect.DeepEqual(x, reflect.Zero(reflect.TypeOf(x)).Interface())
}

// Assert error and exit process
func Assert(err error) {
	if err != nil {
		log.Fatalln(err)
	}
}

// Assert error and exit process
func AssertEmpty(v interface{}) {
	if IsEmpty(v) {
		log.Fatalln("Empty interface")
	}
}
