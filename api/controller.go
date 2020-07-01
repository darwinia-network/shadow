package api

import (
	"net/http"
	"strconv"

	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/util"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/gin-gonic/gin"
)

type Controller struct {
	Shadow core.Shadow
}

func NewController() (Controller, error) {
	shadow, err := core.NewShadow()
	if err != nil {
		return Controller{}, err
	}

	return Controller{
		shadow,
	}, nil
}

func (c *Controller) FromShadow(shadow core.Shadow) Controller {
	return Controller{
		shadow,
	}
}

// Get ETH Header by hash godoc
// @Summary Show a account
// @Description get string by ID
// @ID get-string-by-int
// @Accept  json
// @Produce  json
// @Param hash path string true "Eth header hash"
// @Success 200 {object} types.Header
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /header/{block} [get]
func (c *Controller) GetHeader(ctx *gin.Context) {
	var header types.Header
	block, err := util.NumberOrString(ctx.Param("block"))
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	header, err = c.Shadow.GetHeader(core.Ethereum, block)
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, header)
}

// Get ETH header with proof by number godoc
// @Summary Show a account
// @Description get string by ID
// @ID get-string-by-int
// @Accept  json
// @Produce  json
// @Param number path uint64 true "Eth header number"
// @Success 200 {object} core.GetEthHeaderByNumberParams
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /proof/number/{number} [get]
func (c *Controller) GetProof(ctx *gin.Context) {
	var resp interface{}
	block, err := util.NumberOrString(ctx.Param("block"))
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	format := ctx.DefaultQuery("format", "json")
	resp, err = c.Shadow.GetHeaderWithProof(
		core.Ethereum,
		block,
		new(core.ProofFormat).From(format),
	)

	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, resp)
}

// Get headers by proposal
// @Summary Show a account
// @Description get string by ID
// @ID get-string-by-int
// @Accept  json
// @Produce  json
// @Param numbers query []uint64 true "Eth header numbers"
// @Success 200 {array} core.GetEthHeaderWithProofByNumberJSONResp
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /proposal [post]
func (c *Controller) Proposal(ctx *gin.Context) {
	var (
		resp    interface{}
		numbers []uint64
		err     error
	)
	ns := ctx.Request.URL.Query()["numbers"]
	for _, n := range ns {
		num, _ := strconv.ParseUint(n, 10, 64)
		numbers = append(numbers, num)
	}

	format := ctx.DefaultQuery("format", "json")
	resp, err = c.Shadow.GetProposalHeaders(
		numbers,
		new(core.ProofFormat).From(format),
	)
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, resp)
}
