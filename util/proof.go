package util

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/tranvictor/ethashproof"
	"github.com/tranvictor/ethashproof/ethash"
	"github.com/tranvictor/ethashproof/mtree"
)

// This struct is used for process interaction
type ProofOutput struct {
	HeaderRLP    string   `json:"header_rlp"`
	MerkleRoot   string   `json:"merkle_root"`
	Elements     []string `json:"elements"`
	MerkleProofs []string `json:"merkle_proofs"`
	ProofLength  uint64   `json:"proof_length"`
}

// Proof eth blockheader
func Proof(header *types.Header) (ProofOutput, error) {
	blockno := header.Number.Uint64()
	epoch := blockno / 30000
	output := &ProofOutput{}

	// get proof from cache
	cache, err := ethashproof.LoadCache(int(epoch))
	if err != nil {
		_, err = ethashproof.CalculateDatasetMerkleRoot(epoch, true)
		if err != nil {
			return *output, err
		}

		cache, err = ethashproof.LoadCache(int(epoch))
		if err != nil {
			return *output, err
		}
	}

	rlpheader, err := ethashproof.RLPHeader(header)
	if err != nil {
		return *output, err
	}

	// init output
	output = &ProofOutput{
		HeaderRLP:    hexutil.Encode(rlpheader),
		MerkleRoot:   cache.RootHash.Hex(),
		Elements:     []string{},
		MerkleProofs: []string{},
		ProofLength:  cache.ProofLength,
	}

	// get verification indices
	indices := ethash.Instance.GetVerificationIndices(
		blockno,
		ethash.Instance.SealHash(header),
		header.Nonce.Uint64(),
	)

	// fill output
	for _, index := range indices {
		// calculate proofs
		element, proof, err := ethashproof.CalculateProof(blockno, index, cache)
		if err != nil {
			return *output, err
		}

		es := element.ToUint256Array()
		for _, e := range es {
			output.Elements = append(output.Elements, hexutil.EncodeBig(e))
		}

		// append proofs
		allProofs := []*big.Int{}
		for _, be := range mtree.HashesToBranchesArray(proof) {
			allProofs = append(allProofs, be.Big())
		}

		for _, pr := range allProofs {
			output.MerkleProofs = append(output.MerkleProofs, hexutil.EncodeBig(pr))
		}
	}

	return *output, nil
}
