package tests

import (
	"fmt"
	"testing"

	"github.com/darwinia-network/shadow/internal"
	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/util"
)

/**
 * Generate Shadow API
 */
func genShadow() core.ShadowRPC {
	conf := internal.Config{}
	err := conf.Load()
	util.Assert(err)

	// Generate Shadow
	shadow, err := core.NewShadowRPC()
	util.Assert(err)

	// Generate shadow rpc
	return shadow
}

func TestGetBlockByNumber(t *testing.T) {
	t.Run("Test GetBlockByNumber", func(t *testing.T) {
		shadow := genShadow()
		params := core.GetEthHeaderByNumberParams{Number: uint64(1)}
		resp := core.GetEthHeaderResp{}
		err := shadow.GetEthHeaderByNumber(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)
	})
}

func TestGetBlockByHash(t *testing.T) {
	t.Run("Test GetBlockByHash", func(t *testing.T) {
		shadow := genShadow()
		params := core.GetEthHeaderByHashParams{
			Hash: fmt.Sprintf(
				"%s%s",
				"0x88e96d4537bea4d9c05d12549907b32",
				"561d3bf31f45aae734cdc119f13406cb6",
			),
		}
		resp := core.GetEthHeaderResp{}
		err := shadow.GetEthHeaderByHash(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)
	})
}

func TestGetEthHeaderWithProofByNumber(t *testing.T) {
	t.Run("Test GetEthHeaderWithProofByNumber", func(t *testing.T) {
		shadow := genShadow()
		params := core.GetEthHeaderWithProofByNumberParams{Number: 1}
		var resp interface{}
		err := shadow.GetEthHeaderWithProofByNumber(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)
	})
}

func TestGetEthHeaderWithProofByHash(t *testing.T) {
	t.Run("Test GetEthHeaderWithProofByHash", func(t *testing.T) {
		shadow := genShadow()
		params := core.GetEthHeaderWithProofByHashParams{
			Hash: fmt.Sprintf(
				"%s%s",
				"0x88e96d4537bea4d9c05d12549907b32",
				"561d3bf31f45aae734cdc119f13406cb6",
			),
		}
		var resp interface{}
		err := shadow.GetEthHeaderWithProofByHash(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)
	})
}

func TestBatchEthHeaderWithProofByNumber(t *testing.T) {
	t.Run("Test BatchEthHeaderWithProofByNumber", func(t *testing.T) {
		shadow := genShadow()
		params := core.BatchEthHeaderWithProofByNumberParams{Number: 1, Batch: 3}
		var resp interface{}
		err := shadow.BatchEthHeaderWithProofByNumber(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)

		switch r := resp.(type) {
		case []core.GetEthHeaderWithProofRawResp:
			if len(r) != 3 {
				t.Errorf("Wrong length %v in batch header resp", r)
			}
		default:
			t.Errorf("Wrong type %v in batch header", r)
		}
	})
}

func TestGetReceipt(t *testing.T) {
	t.Run("Test GetReceipt", func(t *testing.T) {
		shadow, err := core.NewShadow()
		util.Assert(err)

		resp, err := shadow.GetReceipt("/0x663cffc56aece411d9dc8096a162b65089d720d69e06f953bd58804cedebb06f")
		util.Assert(err)

		util.AssertEmpty(resp.Header)
		util.AssertEmpty(resp.ReceiptProof)
	})
}
