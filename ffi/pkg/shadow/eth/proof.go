package eth

import (
    "os"
    "fmt"
    "sync"
    "errors"
    "time"
    "math/big"
    "math"
    "path/filepath"
    "github.com/ethereum/go-ethereum/core/types"
    "github.com/ethereum/go-ethereum/common/hexutil"
    "github.com/darwinia-network/shadow/ffi/pkg/ethashproof"
    "github.com/darwinia-network/shadow/ffi/pkg/ethashproof/ethash"
    "github.com/darwinia-network/shadow/ffi/pkg/ethashproof/mtree"
    "github.com/darwinia-network/shadow/ffi/pkg/log"
    "github.com/darwinia-network/shadow/ffi/pkg/shadow/config"
)

type EthashProof struct {
    Epoching uint64
    // epoch=>timestamp
    SavedEpochs map[uint64]int64
    mu sync.Mutex
    ch chan interface{}

    rootDir string
    limitSize int
}

type NotifyWaitInfo struct {
    Epoch uint64
    Error chan error
}

type Output struct {
    HeaderRLP    string   `json:"header_rlp"`
    MerkleRoot   string   `json:"merkle_root"`
    Elements     []string `json:"elements"`
    MerkleProofs []string `json:"merkle_proofs"`
    ProofLength  uint64   `json:"proof_length"`
}

type DoubleNodeWithMerkleProof struct {
    DagNodes []string `json:"dag_nodes"`
    Proof    []string `json:"proof"`
}

func NewEthashProof(conf *config.EthProof) *EthashProof {
    normalizedPath := config.NormalizePath(conf.RootPath)
    log.Info("init ethash proof root path %v", normalizedPath)
    return &EthashProof {
        Epoching: math.MaxUint64,
        ch: make(chan interface{}),
        SavedEpochs: make(map[uint64]int64),
        rootDir: normalizedPath,
        limitSize: conf.Limitepochsize,
    }
}

// find existense of the epoch files
func (proof *EthashProof) Start(currentEpoch uint64) {
    // load old epoch
    cacheDir := proof.ethashProofDir()
    dagDir := proof.ethashDir()
    for ep := uint64(0); ep <= currentEpoch; ep++ {
        ethash.RemoveTempFile(dagDir, ep)
        if ep + uint64(proof.limitSize) < currentEpoch {
            ethashproof.RemoveEpochFile(cacheDir, dagDir, ep)
            continue
        }
        cacheFile := ethashproof.CacheFilePath(cacheDir, ep)
        dagFile := ethash.PathToDAG(ep, dagDir)
        info, err := os.Stat(cacheFile)
        if err == nil {
            _, err := os.Stat(dagFile)
            if err == nil {
                proof.SavedEpochs[ep] = info.ModTime().Unix()
                log.Info("find exist epoch file, epoch %v", ep)
            } else {
                ethashproof.RemoveEpochFile(cacheDir, dagDir, ep)
            }
        } else {
            ethashproof.RemoveEpochFile(cacheDir, dagDir, ep)
        }
    }
    go proof.epochLoop()
}

func (proof *EthashProof) Stop() {
    proof.ch <-errors.New("Stop epoch!")
}

func (proof *EthashProof) ethashDir() string {
    return filepath.Join(proof.rootDir, "ethash")
}

func (proof *EthashProof) ethashProofDir() string {
    return filepath.Join(proof.rootDir, "ethashproof")
}

func (proof *EthashProof) GenerateEpoch(epoch uint64) error {
    _, ok := proof.SavedEpochs[epoch]
    if ok {
        return nil
    }
    if proof.Epoching == epoch {
        return nil
    }
    proof.Epoching = epoch
    log.Info("start to create epoch cache epoch:%v", epoch)
    _, err := ethashproof.CalculateDatasetMerkleRoot(proof.ethashDir(), proof.ethashProofDir(), epoch, true)
    if err != nil {
        log.Error("create cache failed err %v", err)
        proof.Epoching = math.MaxUint64
        return err
    }
    proof.mu.Lock()
    defer proof.mu.Unlock()
    log.Info("successfully cache epoch %v", epoch)
    proof.SavedEpochs[epoch] = time.Now().Unix()
    return nil
}

func (proof *EthashProof) clearEpoch() {
    var clear_epoch = func(epoch uint64) {
        ethashproof.RemoveEpochFile(proof.ethashProofDir(), proof.ethashDir(), epoch)
        delete(proof.SavedEpochs, epoch)
    }
    proof.mu.Lock()
    defer proof.mu.Unlock()
    //todo use both epoch and timestamp
    if len(proof.SavedEpochs) > proof.limitSize {
        var minEpoch uint64 = math.MaxUint64
        var secondMinEpoch uint64 = minEpoch
        var minTime int64 = math.MaxInt64
        var secondMinTime int64 = math.MaxInt64
        for ep, timestamp := range proof.SavedEpochs {
            if ep < minEpoch {
                secondMinEpoch = minEpoch
                secondMinTime = minTime
                minEpoch = ep
                minTime = timestamp
            } else if ep < secondMinEpoch {
                secondMinEpoch = ep
                secondMinTime = timestamp
            }
        }
        log.Info("remove old epoch file, ep %v, timestamp %v, second ep %v, second time %v", minEpoch, minTime, secondMinEpoch, secondMinTime)
        if secondMinTime < minTime {
            clear_epoch(secondMinEpoch)
        } else {
            clear_epoch(minEpoch)
        }
    }
}

func (proof *EthashProof) epochLoop() {
    clearInterval := time.Second* time.Duration(10)
    checkTimer := time.NewTimer(clearInterval)
    for {
        select {
        case v := <-proof.ch:
            switch v:= v.(type) {
            case error:
                log.Info("epoch loop ended with error %v", v)
                return
            case uint64:
                err := proof.GenerateEpoch(v)
                if err != nil {
                    log.Info("generate epoch failed err %v", err)
                }
            case *NotifyWaitInfo:
                err := proof.GenerateEpoch(v.Epoch)
                v.Error <-err
            }
        case <-checkTimer.C:
            proof.clearEpoch()
            checkTimer.Reset(clearInterval)
        }
    }
}

func (proof *EthashProof) NotifyEpoch(blockno uint64) bool {
    epoch := blockno / 30000
    select {
    case proof.ch <-epoch:
        return true
    default:
        return false
    }
}

func (proof *EthashProof) NotifyEpochWait(epoch uint64) error {
    waitInfo := &NotifyWaitInfo {
        Epoch: epoch,
        Error: make(chan error),
    }

    proof.ch <- waitInfo
    return <-waitInfo.Error
}

func (proof *EthashProof) Proof(header *types.Header, wait bool) (*Output, []DoubleNodeWithMerkleProof, error) {
    blockno := header.Number.Uint64()
    epoch := blockno / 30000
    log.Info("start to get ethashproof, blockno %v", blockno)
    proof.mu.Lock()
    defer proof.mu.Unlock()
    _, ok := proof.SavedEpochs[epoch]
    if !ok {
        log.Warn("Cache is missing, notify to generate epoch %v", epoch)
        if wait {
            proof.mu.Unlock()
            err := proof.NotifyEpochWait(epoch)
            proof.mu.Lock()
            if err != nil {
                return nil, nil, err
            }
        } else {
            proof.NotifyEpoch(blockno)
            return nil, nil, errors.New("cache is not found, wait")
        }
    }
    //ethash.NeedYield = true
    //defer func(){ethash.NeedYield = false}()
    cache, err := ethashproof.LoadCache(proof.ethashProofDir(), epoch)
    if err != nil {
        log.Info("Cache is missing, create cache number %v, epoch %v err %v", blockno, epoch, err)
        return nil, nil, err
    }

    indices := ethash.Instance.GetVerificationIndices(
        blockno,
        ethash.Instance.SealHash(header),
        header.Nonce.Uint64(),
    )

    log.Info("Proof length %v, number %v", cache.ProofLength, blockno)

    rlpheader, err := ethashproof.RLPHeader(header)
    if err != nil {
        log.Error("Can't rlp encode the header err %v", err)
        return nil, nil, err
    }

    output := &Output{
        HeaderRLP:    hexutil.Encode(rlpheader),
        MerkleRoot:   cache.RootHash.Hex(),
        Elements:     []string{},
        MerkleProofs: []string{},
        ProofLength:  cache.ProofLength,
    }
    dumps := []DoubleNodeWithMerkleProof{}
    for _, index := range indices {
        element, proof, err := ethashproof.CalculateProof(proof.ethashDir(), blockno, index, cache)
        if err != nil {
            log.Error("calculating the proofs failed index %v err %v", index, err)
            return nil, nil, err
        }
        es := element.ToUint256Array()
        dump := DoubleNodeWithMerkleProof{
            DagNodes: make([]string, 2),
        }
        for index, e := range es {
            output.Elements = append(output.Elements, hexutil.EncodeBig(e))
            dump.DagNodes[index / 2] = dump.DagNodes[index / 2] + fmt.Sprintf("%s%064s", "0x"[(index&1<<1):], hexutil.EncodeBig(e)[2:])
        }
        allProofs := []*big.Int{}
        for _, be := range mtree.HashesToBranchesArray(proof) {
            allProofs = append(allProofs, be.Big())
        }
        for _, pr := range allProofs {
            output.MerkleProofs = append(output.MerkleProofs, hexutil.EncodeBig(pr))
            dump.Proof = append(dump.Proof, fmt.Sprintf("0x%032s", hexutil.EncodeBig(pr)[2:]))
        }
        dumps = append(dumps, dump)
    }
    return output, dumps, nil
}

