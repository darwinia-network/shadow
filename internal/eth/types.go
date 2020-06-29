package eth

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
}

type Transaction struct {
	Hash  string `json:"hash"`
	From  string `json:"from"`
	Input string `json:"input"`
	To    string `json:"to"`
	Value string `json:"value"`
}
