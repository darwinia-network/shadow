package ethashproof

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"os/user"
	"path/filepath"

	"github.com/darwinia-network/shadow/pkg/ethashproof/mtree"
	"github.com/darwinia-network/shadow/pkg/ethashproof/ethash"
	"github.com/darwinia-network/shadow/pkg/log"
)

const CACHE_LEVEL uint64 = 15
const MAX_CACHE_SIZE int = 5

var (
	DefaultDir = defaultDir()
)

type DatasetMerkleTreeCache struct {
	Epoch       uint64         `json:"epoch"`
	ProofLength uint64         `json:"proof_length"`
	CacheLength uint64         `json:"cache_length"`
	RootHash    mtree.Hash     `json:"root_hash"`
	Proofs      [][]mtree.Hash `json:"proofs"`
}

func (self *DatasetMerkleTreeCache) Print() {
	fmt.Printf("Epoch: %d\n", self.Epoch)
	fmt.Printf("Merkle root: %s\n", self.RootHash.Hex())
	fmt.Printf("Sub proofs:\n")
	for i, proof := range self.Proofs {
		fmt.Printf("%d. [", i)
		for _, node := range proof {
			fmt.Printf("%s, ", node.Hex())
		}
		fmt.Printf("]\n")
	}
}

func getHomeDir() string {
    usr, err := user.Current()
    log.Assert(err)
    return usr.HomeDir
}

func defaultDir() string {
    return filepath.Join(getHomeDir(), ".ethashproof")
}

func PersistCache(dirPath string, cache *DatasetMerkleTreeCache) error {
	content, err := json.Marshal(cache)
	if err != nil {
		return err
	}
	err = os.MkdirAll(dirPath, 0777)
	if err != nil {
		return err
	}
	path := filepath.Join(dirPath, fmt.Sprintf("%d.json", cache.Epoch))
	err = ioutil.WriteFile(path, content, 0644)
    if err == nil {
        removeCache(dirPath, ethash.DefaultDir, cache.Epoch)
    }
    return err
}

func removeCache(cachedir, dagdir string, epoch uint64) {
    for ep := uint64(0); ep + uint64(MAX_CACHE_SIZE) <= epoch; ep++ {
        path := filepath.Join(cachedir, fmt.Sprintf("%d.json", ep))
        _, err := os.Stat(path)
        if err == nil {
            err = os.Remove(path)
        }
        if err == nil || os.IsNotExist(err) {
            ethash.RemoveDatasetFile(dagdir, ep)
        }
    }
}

func LoadCache(dirPath string, epoch int) (*DatasetMerkleTreeCache, error) {
	path := filepath.Join(dirPath, fmt.Sprintf("%d.json", epoch))
	content, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, err
	}
	result := &DatasetMerkleTreeCache{}
	err = json.Unmarshal(content, &result)
	if err != nil {
		return nil, err
	}
	return result, nil
}
