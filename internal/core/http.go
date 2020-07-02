package core

import (
	"net/http"
	"strconv"

	"github.com/darwinia-network/shadow/internal/util"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/gin-gonic/gin"
)

// NewError example
func NewError(ctx *gin.Context, status int, err error) {
	er := HTTPError{
		Code:    status,
		Message: err.Error(),
	}
	ctx.JSON(status, er)
}

// HTTPError example
type HTTPError struct {
	Code    int    `json:"code" example:"400"`
	Message string `json:"message" example:"status bad request"`
}

type ShadowHTTP struct {
	Shadow Shadow
}

func NewShadowHTTP() (ShadowHTTP, error) {
	shadow, err := NewShadow()
	if err != nil {
		return ShadowHTTP{}, err
	}

	return ShadowHTTP{
		shadow,
	}, nil
}

func (c *ShadowHTTP) FromShadow(shadow Shadow) ShadowHTTP {
	return ShadowHTTP{
		shadow,
	}
}

// Get Header by hash godoc
// @Summary Get ETH Header by block
// @Description Get ETH Header by block number or hash
// @ID get-header-by-block
// @Accept  json
// @Produce  json
// @Param block path string true "Eth header number or hash"
// @Success 200 {object} types.Header
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /header/{block} [get]
func (c *ShadowHTTP) GetHeader(ctx *gin.Context) {
	var header types.Header
	block, err := util.NumberOrString(ctx.Param("block"))
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	header, err = c.Shadow.GetHeader(Ethereum, block)
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, header)
}

// Get header with proof godoc
// @Summary Get header with proof
// @Description Get header with hash proof and mmr roothash
// @ID get-header-with-proof
// @Accept  json
// @Produce  json
// @Param block path string true "Eth header number or hash"
// @Success 200 {object} GetEthHeaderWithProofByNumberJSONResp
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /proof/{block} [get]
func (c *ShadowHTTP) GetProof(ctx *gin.Context) {
	var resp interface{}
	block, err := util.NumberOrString(ctx.Param("block"))
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	format := ctx.DefaultQuery("format", "json")
	resp, err = c.Shadow.GetHeaderWithProof(
		Ethereum,
		block,
		new(ProofFormat).From(format),
	)

	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, resp)
}

// Get headers by proposal
// @Summary Get headers by block numbers
// @Description Get headers by block numbers, used for relay proposal
// @ID get-headers-by-proposal
// @Accept  json
// @Produce  json
// @Param numbers query []uint64 true "Eth header numbers"
// @Success 200 {array} []GetEthHeaderWithProofByNumberJSONResp
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /proposal [post]
func (c *ShadowHTTP) Proposal(ctx *gin.Context) {
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
		new(ProofFormat).From(format),
	)
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, resp)
}
