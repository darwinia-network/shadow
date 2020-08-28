package eth

import (
	"encoding/hex"
)

func lenToBytes(b []uint8, len int) ([]uint8, int) {
	if len < 255 {
		b = append(b, uint8(len))
		len = 0
		return b, len
	}

	b = append(b, uint8(len/0xff))
	len = len % 0xff
	return lenToBytes(b, len)
}

func LenToHex(len int) string {
	b, _ := lenToBytes([]uint8{}, len*4)
	return hex.EncodeToString(b)
}

// Pack encode proof
func EncodeProofArray(arr []DoubleNodeWithMerkleProof) string {
	hex := "0x" + LenToHex(len(arr))
	for _, v := range arr {
		hex += EncodeProof(v)
	}

	return hex
}

// Encode proof to hex with exist hex
func EncodeProof(dnmp DoubleNodeWithMerkleProof) string {
	hex := ""
	for _, v := range dnmp.DagNodes {
		hex += v[2:]
	}

	// pad the length
	hex += LenToHex(len(dnmp.Proof))
	for _, v := range dnmp.Proof {
		hex += v[2:]
	}

	return hex
}
