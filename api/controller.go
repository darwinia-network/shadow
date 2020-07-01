package api

import (
	"net/http"
	"strconv"

	"github.com/darwinia-network/shadow/internal/core"
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
// @Router /header/hash/{hash} [get]
func (c *Controller) GetEthHeaderByHash(ctx *gin.Context) {
	var resp core.GetEthHeaderResp
	hash := ctx.Param("hash")
	err := c.Shadow.GetEthHeaderByHash(core.GetEthHeaderByHashParams{Hash: hash}, &resp)
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	var header types.Header = resp.Header
	ctx.JSON(http.StatusOK, header)
}

// Get ETH Header by number godoc
// @Summary Show a account
// @Description get string by ID
// @ID get-string-by-int
// @Accept  json
// @Produce  json
// @Param number path uint64 true "Eth header number"
// @Success 200 {object} types.Header
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /header/number/{number} [get]
func (c *Controller) GetEthHeaderByNumber(ctx *gin.Context) {
	var resp core.GetEthHeaderResp
	num, _ := strconv.ParseUint(ctx.Param("number"), 10, 64)
	err := c.Shadow.GetEthHeaderByNumber(core.GetEthHeaderByNumberParams{Number: num}, &resp)
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	var header types.Header = resp.Header
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
func (c *Controller) GetEthProofByNumber(ctx *gin.Context) {
	var resp interface{}
	num, _ := strconv.ParseUint(ctx.Query("number"), 10, 64)
	format := ctx.DefaultQuery("format", "json")
	err := c.Shadow.GetEthHeaderWithProofByNumber(
		core.GetEthHeaderWithProofByNumberParams{
			Number: num,
			Options: core.GetEthHeaderWithProofByNumberOptions{
				Format: format,
			},
		}, &resp)

	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, resp)
}

// Get ETH header with proof by hash godoc
// @Summary Show a account
// @Description get string by ID
// @ID get-string-by-int
// @Accept  json
// @Produce  json
// @Param hash query string true "Eth header hash"
// @Success 200 {object} core.GetEthHeaderWithProofByNumberJSONResp
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /proof/hash [post]
func (c *Controller) GetEthProofByHash(ctx *gin.Context) {
	var resp interface{}
	hash := ctx.Query("hash")
	format := ctx.DefaultQuery("format", "json")
	err := c.Shadow.GetEthHeaderWithProofByHash(
		core.GetEthHeaderWithProofByHashParams{
			Hash: hash,
			Options: core.GetEthHeaderWithProofByNumberOptions{
				Format: format,
			},
		}, &resp)

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
	var resp interface{}
	var numbers []uint64
	ns := ctx.Request.URL.Query()["numbers"]
	for _, n := range ns {
		num, _ := strconv.ParseUint(n, 10, 64)
		numbers = append(numbers, num)
	}

	format := ctx.DefaultQuery("format", "json")
	err := c.Shadow.GetProposalEthHeaders(
		core.GetProposalEthHeadersParams{
			Numbers: numbers,
			Options: core.GetEthHeaderWithProofByNumberOptions{
				Format: format,
			},
		}, &resp)

	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, resp)
}
