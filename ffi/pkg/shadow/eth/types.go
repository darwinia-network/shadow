package eth

import (
	"encoding/json"

	"github.com/ethereum/go-ethereum/core/types"
)

// The response of etherscan api
type InfuraResponse struct {
	JsonRPC string       `json:"jsonrpc"`
	Id      uint32       `json:"id"`
	Result  types.Header `json:"result"`
}

// Darwinia block
type DarwiniaEthHeader struct {
	ParentHash       string   `json:"parent_hash"`
	TimeStamp        uint64   `json:"timestamp"`
	Number           uint64   `json:"number"`
	Author           string   `json:"author"`
	TransactionsRoot string   `json:"transactions_root"`
	UnclesHash       string   `json:"uncles_hash"`
	ExtraData        string   `json:"extra_data"`
	StateRoot        string   `json:"state_root"`
	ReceiptsRoot     string   `json:"receipts_root"`
	LogBloom         string   `json:"log_bloom"`
	GasUsed          uint64   `json:"gas_used"`
	GasLimited       uint64   `json:"gas_limit"`
	Difficulty       uint64   `json:"difficulty"`
	Seal             []string `json:"seal"`
	Hash             string   `json:"hash"`
}

func (d *DarwiniaEthHeader) ToString() (string, error) {
	bytes, err := json.Marshal(d)
	if err != nil {
		return "", err
	}
	return string(bytes), nil
}

type DarwiniaEthHeaderHexFormat struct {
	ParentHash       string   `json:"parent_hash"`
	TimeStamp        string   `json:"timestamp"`
	Number           string   `json:"number"`
	Author           string   `json:"author"`
	TransactionsRoot string   `json:"transactions_root"`
	UnclesHash       string   `json:"uncles_hash"`
	ExtraData        string   `json:"extra_data"`
	StateRoot        string   `json:"state_root"`
	ReceiptsRoot     string   `json:"receipts_root"`
	LogBloom         string   `json:"log_bloom"`
	GasUsed          string   `json:"gas_used"`
	GasLimited       string   `json:"gas_limit"`
	Difficulty       string   `json:"difficulty"`
	Seal             []string `json:"seal"`
	Hash             string   `json:"hash"`
}

type Receipts struct {
	BlockNumber      string `json:"block_number"`
	Logs             []Log  `json:"logs"`
	Status           string `json:"status"`
	ChainSource      string `json:"chainSource"`
	GasUsed          string `json:"gasUsed"`
	LogsBloom        string `json:"logsBloom"`
	Solidity         bool   `json:"solidity"`
	TransactionIndex string `json:"transactionIndex"`
	BlockHash        string `json:"blockHash"`
}

type Log struct {
	Topics  []string `json:"topics"`
	Data    string   `json:"data"`
	Address string   `json:"address"`
}

type BlockResult struct {
	Difficulty       string        `json:"difficulty"`
	ExtraData        string        `json:"extraData"`
	GasLimit         string        `json:"gasLimit"`
	GasUsed          string        `json:"gasUsed"`
	Hash             string        `json:"hash"`
	LogsBloom        string        `json:"logs_bloom"`
	Miner            string        `json:"miner"`
	MixHash          string        `json:"mixHash"`
	Nonce            string        `json:"nonce"`
	Number           string        `json:"number"`
	ParentHash       string        `json:"parentHash"`
	ReceiptsRoot     string        `json:"receiptsRoot"`
	Sha3Uncles       string        `json:"sha3Uncles"`
	StateRoot        string        `json:"stateRoot"`
	Timestamp        string        `json:"timestamp"`
	TotalDifficulty  string        `json:"totalDifficulty"`
	Transactions     []Transaction `json:"transactions"`
	TransactionsRoot string        `json:"transactionsRoot"`
	BaseFee          string        `json:"baseFeePerGas" rlp:"optional"`
}

type Transaction struct {
	Hash  string `json:"hash"`
	From  string `json:"from"`
	Input string `json:"input"`
	To    string `json:"to"`
	Value string `json:"value"`
}
