package util

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/ethereum/go-ethereum/core/types"
)

// The api of fetching eth header from etherscan XDD
const ETHERSCAN_API = "https://api.etherscan.io/api?module=proxy&action=eth_getBlockByNumber&tag=%d&boolean=true"

// The response of etherscan api
type EtherScanResponse struct {
	JsonRPC string       `json:"jsonrpc"`
	Id      uint32       `json:"id"`
	Result  types.Header `json:"result"`
}

// Get ethereum header by block number
func Header(blockNum uint64) types.Header {
	esResp := EtherScanResponse{}
	resp, err := http.Get(fmt.Sprintf(ETHERSCAN_API, blockNum))
	Assert(err)

	err = json.NewDecoder(resp.Body).Decode(&esResp)
	Assert(err)

	return esResp.Result
}
