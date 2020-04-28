package core

import (
	"github.com/darwinia-network/darwinia.go/util"
)

func encodeProof(arr []util.DoubleNodeWithMerkleProof) string {
	hex := "0x0101"
	for _, v := range arr {
		hex += v.Encode()
	}

	return hex
}
