package handlers

import (
	"net/http"

	"jojuhu/services"

	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// GetUserPosts godoc
// @Summary Get user posts
// @Description Retrieves all posts for a given user.
// @Tags posts
// @Produce json
// @Param userId path string true "User ID"
// @Success 200 {array} models.Post
// @Failure 400 {object} map[string]string "error"
// @Failure 500 {object} map[string]string "error"
// @Router /users/{userId}/posts [get]
func GetUserPosts(c *gin.Context) {
	userIDStr := c.Param("userId")
	userID, err := primitive.ObjectIDFromHex(userIDStr)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid user ID"})
		return
	}

	posts, err := services.GetUserPosts(userID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to get posts"})
		return
	}

	c.JSON(http.StatusOK, posts)
}
