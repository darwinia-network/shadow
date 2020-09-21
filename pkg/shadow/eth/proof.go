package eth

import (
	"errors"
	"fmt"
	"math/big"

	"github.com/darwinia-network/shadow/pkg/ethashproof"
	"github.com/darwinia-network/shadow/pkg/ethashproof/ethash"
	"github.com/darwinia-network/shadow/pkg/ethashproof/mtree"
	"github.com/darwinia-network/shadow/pkg/shadow"
	"github.com/darwinia-network/shadow/pkg/shadow/util"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
)

// Constants
const ETHASHPROOF_CACHE = ".ethashproof"

// Final outputs of ethashproof
type DoubleNodeWithMerkleProof struct {
	DagNodes []string `json:"dag_nodes"`
	Proof    []string `json:"proof"`
}

// This struct is used for process interaction
type ProofOutput struct {
	HeaderRLP    string   `json:"header_rlp"`
	MerkleRoot   string   `json:"merkle_root"`
	Elements     []string `json:"elements"`
	MerkleProofs []string `json:"merkle_proofs"`
	ProofLength  uint64   `json:"proof_length"`
}

// Format ProofOutput to double node with merkle proofs
func (o *ProofOutput) Format() []DoubleNodeWithMerkleProof {
	h512s := util.Filter(o.Elements, func(i int, _ string) bool {
		return i%2 == 0
	})

	h512s = util.Map(h512s, func(i int, v string) string {
		return fmt.Sprintf("0x%064s%064s", v[2:], o.Elements[(i*2)+1][2:])
	})

	dnmps := []DoubleNodeWithMerkleProof{}
	sh512s := util.Filter(h512s, func(i int, _ string) bool {
		return i%2 == 0
	})
	util.Map(sh512s, func(i int, v string) string {
		dnmps = append(dnmps, DoubleNodeWithMerkleProof{
			[]string{v, h512s[i*2+1]},
			util.Map(
				o.MerkleProofs[uint64(i)*o.ProofLength:(uint64(i)+1)*o.ProofLength],
				func(_ int, v string) string {
					return fmt.Sprintf("0x%032s", v[2:])
				},
			),
		})
		return v
	})

	return dnmps
}

func Epoch(block uint64, config *shadow.Config) (*ethashproof.DatasetMerkleTreeCache, error) {
	epoch := block / 30000
	cache, err := ethashproof.LoadCache(int(epoch))
	if err != nil {
		err = config.CreateLock(shadow.PROOF_LOCK, epoch)
		if err != nil {
			return nil, errors.New("Create epoch lock failed")
		}

		_, err = ethashproof.CalculateDatasetMerkleRoot(epoch, true)
		if err != nil {
			return nil, errors.New("Calculate merkle root failed")
		}

		cache, err = ethashproof.LoadCache(int(epoch))
		if err != nil {
			return nil, errors.New("Get ethash proof failed again, please retry")
		}

		err = config.RemoveLock(shadow.PROOF_LOCK, epoch)
		if err != nil {
			return nil, errors.New("Remove lock failed")
		}
	}

	return cache, nil
}

// Proof eth blockheader
func Proof(header *types.Header, config *shadow.Config) (ProofOutput, error) {
	blockno := header.Number.Uint64()
	output := &ProofOutput{}

	// Get proof from cache
	cache, err := Epoch(blockno, config)
	if err != nil {
		return *output, err
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
