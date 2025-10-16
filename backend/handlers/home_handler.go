package handlers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

// HomeHandler godoc
// @Summary Home
// @Description Returns a welcome message.
// @Tags general
// @Produce plain
// @Success 200 {string} string "Welcome to Jojuhu!"
// @Router / [get]
func HomeHandler(c *gin.Context) {
	c.String(http.StatusOK, "Welcome to Jojuhu!")
}
