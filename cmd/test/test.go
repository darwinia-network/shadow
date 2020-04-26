package main

import (
	"fmt"
	"github.com/darwinia-network/darwinia.go/util"
)

func main() {
	header, err := util.Header(666666)
	if err != nil {
		fmt.Printf("%v\n", err)
	}
	fmt.Printf("%v\n", header)
}
