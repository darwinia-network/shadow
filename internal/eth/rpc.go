package eth

import (
	"github.com/regcostajr/go-web3/dto"
	"github.com/regcostajr/go-web3/providers"
)

func RPC(api string, method string, params interface{}) (*dto.RequestResult, error) {
	provider := providers.NewHTTPProvider(api, 10, false)
	pointer := &dto.RequestResult{}
	err := provider.SendRequest(pointer, method, params)
	if err != nil {
		return nil, err
	}
	return pointer, nil
}
