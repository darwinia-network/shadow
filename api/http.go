package api

import (
	"net/http"

	"github.com/darwinia-network/shadow/internal/core"
	"github.com/darwinia-network/shadow/internal/ffi"
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
	Shadow *core.Shadow
}

func NewShadowHTTP(shadow *core.Shadow) (ShadowHTTP, error) {
	return ShadowHTTP{
		shadow,
	}, nil
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

	header, err = c.Shadow.GetHeader(core.Ethereum, block)
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
// @Success 200 {object} core.GetEthHeaderWithProofJSONResp
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

	// format := ctx.DefaultQuery("format", "json")
	resp, err = c.Shadow.GetHeaderWithProof(
		core.Ethereum,
		block,
	)

	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, resp)
}

// Get receipt by hash
// @Summary Get receipt by tx hash
// @Description Get receipt by tx hash, used for cross-chain transfer
// @ID get-receipt-by-tx
// @Accept  json
// @Produce  json
// @Param tx path string true "tx hash"
// @Success 200 {array} core.GetReceiptResp
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /receipt/{tx} [get]
func (c *ShadowHTTP) GetReceipt(ctx *gin.Context) {
	receipt, err := c.Shadow.GetReceipt(ctx.Param("tx"))
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	ctx.JSON(http.StatusOK, receipt)
}

// Get headers by proposal
// @Summary Get headers by block numbers
// @Description Get headers by block numbers, used for relay proposal
// @ID get-headers-by-proposal
// @Accept  json
// @Produce  json
// @Param numbers query []uint64 true "Eth header numbers"
// @Success 200 {array} []core.GetEthHeaderWithProofJSONResp
// @Header 200 {string} Token "qwerty"
// @Failure 400 {object} HTTPError
// @Router /proposal [post]
func (c *ShadowHTTP) Proposal(ctx *gin.Context) {
	var (
		err             error
		params          ProposalParams
		proposalHeaders []interface{}
	)
	err = ctx.BindJSON(&params)
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	headers, err := c.Shadow.GetProposalHeaders(params.Members)
	if err != nil {
		NewError(ctx, http.StatusBadRequest, err)
		return
	}

	// Construct headers
	for _, h := range headers {
		mmrProof := ffi.ProofLeaves(params.LastLeaf, h.Header.Number)
		if params.Format == "codec" {
			proposalHeaders = append(
				proposalHeaders,
				h.IntoProposalCodec(params.LastLeaf, mmrProof),
			)
		} else {
			proposalHeaders = append(
				proposalHeaders,
				h.IntoProposal(params.LastLeaf, mmrProof),
			)
		}
	}

	ctx.JSON(http.StatusOK, core.ProposalResp{
		Headers: proposalHeaders,
	})
}
