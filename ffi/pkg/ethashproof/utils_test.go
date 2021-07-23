package ethashproof

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/darwinia-network/shadow/ffi/pkg/ethashproof"
	"fmt"
)

// the block 12866795 on ethereum mainnet
func TestHeaderBaseFee(t *testing.T) {
    difficulty, _ := new(big.Int).SetString("0x17ec5383558159", 16)
    header := &types.Header {
	ParentHash: common.HexToHash("0x81b307590f1d8479b6a67c2413e295ee6aa0240c8282df7e2e7328db46e54690"),
	UncleHash: common.HexToHash("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
	Coinbase: common.HexToAddress("0x52bc44d5378309ee2abf1539bf71de1b7d7be3b5"),
	Root: common.HexToHash("0x317b2b0b81108163a13fc660aeaf91e00f6a3360c8b201fa28f20ad3669d9a11"),
	TxHash: common.HexToHash("0x1c0c9e39e6fba04bd3dea16f934fc4601eba1958d36081b0b7ef42d4f4c17966"),
	ReceiptHash: common.HexToHash("0x27c03e834ffe798cc42f3d8ca86d4cb86fb89f465311b25c8cf31c6755d47a91"),
	Bloom: types.BytesToBloom(common.FromHex("0x35a35287c1f619c55b80445ca6514f50558d00863c41d09596a78056bdfd8987120d1f15108212ea1c827b742c202105cabdc01569143d6c8b018d0c62766a91c3418ac5c1838d39eb9a025b2548e7e4a2e3060f0c6dfc2d76433e0982873cab932083052282e8581d60598a26680afa50024f71f09d1e282b41cdba210a066d26352326e7109f585acd42642d8884072420905319119c182fafc15458d8884aa3b79c2b0aa2ff624ec343859514a410c0140e0c7462287a0fa7062e1aa9830d7d1972aa0d57c9f37c1419f366cecd047222230c1509f814245c331a7243e2007c3d3833992410689007f62295aec64044313760e84398d1226acbb80e01b972")),
	Difficulty: difficulty,
	Number: big.NewInt(12866795),
	GasLimit: 14992656,
	GasUsed: 14702384,
	Time: 1626829800,
	Extra: common.FromHex("0x6e616e6f706f6f6c2e6f7267"),
	MixDigest: common.HexToHash("0x1a5a7251e8d7ee9a70b5c11e0aad87990133f762bffe7bd1eb2f542e8da5beb8"),
	Nonce: types.EncodeNonce(16623017644316803142),
	BaseFee: nil,
    }

    bytes, err := ethashproof.RLPHeader(header)
    fmt.Println(bytes, err)
}
