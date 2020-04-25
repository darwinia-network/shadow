package utils

import (
	"fmt"
	"os"
)

// assert error and exit process
func Assert(err error) {
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
