package core

import (
	"encoding/binary"
	"encoding/hex"
	"strings"

	"github.com/darwinia-network/shadow/internal/eth"
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

func lenToHex(len int) string {
	b, _ := lenToBytes([]uint8{}, len*4)
	return hex.EncodeToString(b)
}

// Pack encode proof
func encodeProofArray(arr []eth.DoubleNodeWithMerkleProof) string {
	hex := "0x0101"
	for _, v := range arr {
		hex += encodeProof(v)
	}

	return hex
}

// Encode proof to hex with exist hex
func encodeProof(dnmp eth.DoubleNodeWithMerkleProof) string {
	hex := ""
	for _, v := range dnmp.DagNodes {
		hex += v[2:]
	}

	// pad the length
	hex += lenToHex(len(dnmp.Proof))
	for _, v := range dnmp.Proof {
		hex += v[2:]
	}

	return hex
}

// Encode Darwinia Eth Header
func encodeDarwiniaEthHeader(header eth.DarwiniaEthHeader) string {
	hb, _ := hex.DecodeString(header.ExtraData[2:])
	len := lenToHex(len(hb))

	hex := "0x"
	hex += header.ParentHash[2:]
	hex += encodeUint(header.TimeStamp, 64)
	hex += encodeUint(header.Number, 64)
	hex += strings.ToLower(header.Author[2:])
	hex += header.TransactionsRoot[2:]
	hex += header.UnclesHash[2:]
	hex += len
	hex += header.ExtraData[2:]
	hex += header.StateRoot[2:]
	hex += header.ReceiptsRoot[2:]
	hex += header.LogBloom[2:]
	hex += encodeUint(header.GasUsed, 256)
	hex += encodeUint(header.GasLimited, 256)
	hex += encodeUint(header.Difficulty, 256)
	hex += "0884"
	hex += header.Seal[0][2:]
	hex += "24"
	hex += header.Seal[1][2:]
	hex += "01"
	hex += header.Hash[2:]

	return hex
}

// Encode uint to hex
func encodeUint(n uint64, d int16) string {
	b := make([]byte, d/8)
	binary.LittleEndian.PutUint64(b, n)

	return hex.EncodeToString(b)
}
