package eth

import (
	"encoding/json"
	"fmt"
	"net/http"
	"reflect"
	"strings"

	"github.com/darwinia-network/shadow/pkg/log"
	"github.com/ethereum/go-ethereum/core/types"
)

// Get uncle block by number
func UncleBlock(number uint64, api string) (types.Header, error) {
	// Get header from infura
	var (
		resp       *http.Response
		err        error
		infuraResp InfuraResponse
	)

	resp, err = http.Post(
		api,
		"application/json",
		strings.NewReader(fmt.Sprintf(GET_UNCLE_BLOCK, number)),
	)

	if err != nil {
		log.Error("get uncle block failed with rpc error %v", err)
		return infuraResp.Result, err
	}

	err = json.NewDecoder(resp.Body).Decode(&infuraResp)
	if err != nil {
		log.Error("get uncle block failed with decode err %v", err)
		return infuraResp.Result, err
	}

	// Empty result
	if reflect.DeepEqual(types.Header{}, infuraResp.Result) {
		log.Error("get uncle block failed with an empty result")
		return infuraResp.Result, fmt.Errorf("The requesting block does not exist")
	}

	// Return eth header
	return infuraResp.Result, nil
}
