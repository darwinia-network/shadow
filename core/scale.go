package core

import (
	"github.com/darwinia-network/darwinia.go/util"
)

// Pack encode proof
func encodeProofArray(arr []util.DoubleNodeWithMerkleProof) string {
	hex := "0x0101"
	for _, v := range arr {
		hex += encodeProof(v)
	}

	return hex
}

// Encode proof to hex with exist hex
func encodeProof(dnmp util.DoubleNodeWithMerkleProof) string {
	hex := ""
	for _, v := range dnmp.DagNodes {
		hex += v[2:]
	}

	// pad the length
	hex += "64"
	for _, v := range dnmp.Proof {
		hex += v[2:]
	}

	return hex
}
