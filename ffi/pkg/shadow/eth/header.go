package eth

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"

	"github.com/ethereum/go-ethereum/core/types"
)

// Get ethereum header by block number
func Header(api string, block uint64) (types.Header, error) {
	// Get header from infura
	var (
		resp       *http.Response
		err        error
		infuraResp InfuraResponse
	)

	resp, err = http.Post(
		api,
		"application/json",
		strings.NewReader(fmt.Sprintf(GETBLOCK, block)),
	)

	if err != nil {
		return infuraResp.Result, err
	}

	// Decode resp to json
	defer resp.Body.Close()
	err = json.NewDecoder(resp.Body).Decode(&infuraResp)
	if err != nil {
		return infuraResp.Result, err
	}

	// Return eth header
	return infuraResp.Result, nil
}

