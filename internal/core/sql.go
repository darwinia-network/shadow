package core

import (
	"fmt"
)

func UpsertHeaderSQL(cache *EthHeaderWithProofCache) string {
	return fmt.Sprintf(
		"%s %s %s %s ('%s', %v, '%s', '%s', '%s');",
		"INSERT or REPLACE INTO",              // insert or replace
		"eth_header_with_proof_caches",        // table
		"(hash, number, header, proof, root)", // columns
		"values",                              // values
		cache.Hash,
		cache.Number,
		cache.Header,
		cache.Proof,
		cache.Root,
	)
}

func SelectHeader(number uint64) string {
	return fmt.Sprintf(
		"%s %s %s %s %v;",
		"SELECT * FROM",
		"eth_header_with_proof_caches",
		"WHERE",
		"number =",
		number,
	)
}
