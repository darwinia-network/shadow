package util

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"

	"github.com/ethereum/go-ethereum/core/types"
)

// The post api of fetching eth header
const GETBLOCK = "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByNumber\",\"params\": [\"0x%x\", false],\"id\":1}"

// The response of etherscan api
type InfuraResponse struct {
	JsonRPC string       `json:"jsonrpc"`
	Id      uint32       `json:"id"`
	Result  types.Header `json:"result"`
}

// Get ethereum header by block number
func Header(blockNum uint64) (types.Header, error) {
	infuraResp := InfuraResponse{}
	conf, err := LoadConfig()
	if err != nil {
		return infuraResp.Result, err
	}

	// Request infura
	resp, err := http.Post(
		conf.Api,
		"application/json",
		strings.NewReader(fmt.Sprintf(GETBLOCK, blockNum)),
	)

	if err != nil {
		return infuraResp.Result, err
	}

	defer resp.Body.Close()

	// Decode resp to json
	err = json.NewDecoder(resp.Body).Decode(&infuraResp)
	if err != nil {
		return infuraResp.Result, err
	}

	// Return eth header
	return infuraResp.Result, nil
}

// Darwinia block
type DarwiniaEthBlock struct {
	ParentHash       string
	TimeStamp        int64
	Number           int64
	Author           string
	TransactionsRoot string
	UnclesHash       string
	ExtraData        []uint8
	StateRoot        string
	ReceiptsRoot     string
	LogBloom         string
	GasUsed          string
	GasLimited       string
	Difficulty       string
	Seal             [][]uint8
	Hash             string
}
