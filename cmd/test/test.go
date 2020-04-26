package main

import (
	"fmt"
	"github.com/darwinia-network/darwinia.go/util"
)

func main() {
	conf, err := util.LoadConfig()
	if err != nil {
		fmt.Printf("%v", err)
	}
	fmt.Printf("%v\n", conf.Api)
}
