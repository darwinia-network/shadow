package eth

import (
	"fmt"
	"io/ioutil"
	"math/big"
	"os"
	"path/filepath"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/darwinia-network/shadow/pkg/ethashproof"
	"github.com/darwinia-network/shadow/pkg/ethashproof/ethash"
	"github.com/darwinia-network/shadow/pkg/ethashproof/mtree"
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

// Epoch in background
func bgEpoch(epoch uint64, config *internal.Config) {
	_, _ = ethashproof.CalculateDatasetMerkleRoot(epoch, true)
	_ = config.RemoveLock(internal.EPOCH_LOCK)
}

// Check if need epoch
func epochGently(epoch uint64, config *internal.Config) error {
	// Check if is epoching
	if config.CheckLock(internal.EPOCH_LOCK) {
		return nil
	}

	// Get home dir
	home, err := os.UserHomeDir()
	if err != nil {
		return err
	}

	// Find ethashproof cache
	cache := filepath.Join(home, ETHASHPROOF_CACHE)
	fs, err := ioutil.ReadDir(cache)
	if err != nil {
		return err
	}

	// Check if has epoched
	hasEpoched := false
	for _, f := range fs {
		if f.Name() == fmt.Sprintf("%v.json", epoch) {
			hasEpoched = true
		}
	}

	// Create epoch lock
	err = config.CreateLock(internal.EPOCH_LOCK)
	if err != nil {
		return err
	}

	// Run epoch
	if !hasEpoched {
		go bgEpoch(epoch, config)
	}

	return nil
}

// Proof eth blockheader
func Proof(header *types.Header, config *internal.Config) (ProofOutput, error) {
	blockno := header.Number.Uint64()
	epoch := blockno / 30000
	output := &ProofOutput{}

	// Get proof from cache
	cache, err := ethashproof.LoadCache(int(epoch))
	if err != nil {
		err = config.RemoveLock(internal.EPOCH_LOCK)
		if err != nil {
			return *output, err
		}

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

	// Check if need pre-epoch
	if blockno%30000 > 15000 {
		err := epochGently(epoch, config)
		if err != nil {
			return *output, err
		}
	}

	return *output, nil
}
