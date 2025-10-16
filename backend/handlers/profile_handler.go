package handlers

import (
	"net/http"

	"jojuhu/services"

	"github.com/gin-gonic/gin"
)

// ProfileHandler godoc
// @Summary Get user profile
// @Description Retrieves the profile of the authenticated user.
// @Tags profile
// @Produce json
// @Success 200 {object} models.User
// @Failure 401 {object} map[string]string "error"
// @Router /profile [get]
func ProfileHandler(c *gin.Context) {
	email, _ := c.Get("email")

	user, err := services.GetProfile(email.(string))
	if err != nil {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "User not found"})
		return
	}

	c.JSON(http.StatusOK, user)
}
