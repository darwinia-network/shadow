package util

import "log"

// Assert error and exit process
func Assert(err error) {
	if err != nil {
		log.Fatalln(err)
	}
}
