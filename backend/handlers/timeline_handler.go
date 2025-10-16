package handlers

import (
	"net/http"

	"jojuhu/services"

	"github.com/gin-gonic/gin"
)

// GetTimeline godoc
// @Summary Get timeline
// @Description Retrieves the timeline with all posts.
// @Tags posts
// @Produce json
// @Success 200 {array} models.Post
// @Failure 500 {object} map[string]string "error"
// @Router /timeline [get]
func GetTimeline(c *gin.Context) {
	posts, err := services.GetTimeline()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to get timeline"})
		return
	}

	c.JSON(http.StatusOK, posts)
}
