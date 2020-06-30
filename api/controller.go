package api

import (
	"net/http"

	"github.com/darwinia-network/shadow/internal/core"
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

// ShowAccount godoc
// @Summary Show a account
// @Description get string by ID
// @ID get-string-by-int
// @Accept  json
// @Produce  json
// @Param id path int true "Account ID"
// @Success 200 {object} string
// @Header 200 {string} Token "qwerty"
// @Router /accounts/{id} [get]
func (c *Controller) ShowAccount(ctx *gin.Context) {
	id := ctx.Param("id")
	ctx.JSON(http.StatusOK, id)
}

// ListAccounts godoc
// @Summary List accounts
// @Description get accounts
// @Accept  json
// @Produce  json
// @Param q query string false "name search by q"
// @Success 200 {array} string
// @Header 200 {string} Token "qwerty"
// @Router /accounts [get]
func (c *Controller) ListAccounts(ctx *gin.Context) {
	ctx.JSON(http.StatusOK, "abc")
}
