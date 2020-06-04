package tests

import (
	// "os"
	"fmt"
	"testing"

	"github.com/darwinia-network/darwinia.go/internal"
	"github.com/darwinia-network/darwinia.go/internal/util"
)

/**
 * Generate Shadow API
 */
func genShadow() internal.Shadow {
	conf := util.Config{}
	err := conf.Load()
	util.Assert(err)

	// Generate shadow rpc
	return internal.Shadow{Config: conf}
}

func TestGetBlockByNumber(t *testing.T) {
	t.Run("Test GetBlockByNumber", func(t *testing.T) {
		shadow := genShadow()
		params := internal.GetEthHeaderByNumberParams{Number: uint64(1)}
		resp := internal.GetEthHeaderByNumberResp{}
		err := shadow.GetEthHeaderByNumber(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)
	})
}

func TestGetBlockByHash(t *testing.T) {
	t.Run("Test GetBlockByHash", func(t *testing.T) {
		shadow := genShadow()
		params := internal.GetEthHeaderByHashParams{
			Hash: fmt.Sprintf(
				"%s%s",
				"0x88e96d4537bea4d9c05d12549907b32",
				"561d3bf31f45aae734cdc119f13406cb6",
			),
		}
		resp := internal.GetEthHeaderByHashResp{}
		err := shadow.GetEthHeaderByHash(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)
	})
}

func TestGetEthHeaderWithProofByNumber(t *testing.T) {
	t.Run("Test GetEthHeaderWithProofByNumber", func(t *testing.T) {
		shadow := genShadow()
		params := internal.GetEthHeaderWithProofByNumberParams{Number: 1}
		var resp interface{}
		err := shadow.GetEthHeaderWithProofByNumber(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)
	})
}

func TestGetEthHeaderWithProofByHash(t *testing.T) {
	t.Run("Test GetEthHeaderWithProofByHash", func(t *testing.T) {
		shadow := genShadow()
		params := internal.GetEthHeaderWithProofByHashParams{
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
		params := internal.BatchEthHeaderWithProofByNumberParams{Number: 1, Batch: 3}
		var resp interface{}
		err := shadow.BatchEthHeaderWithProofByNumber(params, &resp)

		util.Assert(err)
		util.AssertEmpty(resp)

		switch r := resp.(type) {
		case []interface{}:
			if len(r) != 3 {
				t.Errorf("Wrong length %v in batch header resp", r)
			}
		default:
			t.Errorf("Wrong type %v in batch header", r)
		}
	})
}
