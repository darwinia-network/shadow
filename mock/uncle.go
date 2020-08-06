package mock

import (
	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/eth"
)

func Proposal(number uint64, conf internal.Config) (core.GetEthHeaderWithProofRawResp, error) {
	header, _ := eth.UncleBlock(number, conf.Api)
	proof, _ := eth.Proof(&header, conf)
	dh, _ := eth.IntoDarwiniaEthHeader(&header)

	return core.GetEthHeaderWithProofRawResp{
		Header: dh,
		Proof:  proof.Format(),
	}, nil
}
