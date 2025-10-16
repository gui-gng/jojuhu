package handlers

import (
	"net/http"

	"jojuhu/models"
	"jojuhu/services"

	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

// CreatePost godoc
// @Summary Create a new post
// @Description Creates a new post for the authenticated user.
// @Tags posts
// @Accept json
// @Produce json
// @Param post body models.Post true "Post content"
// @Success 201 {object} models.Post
// @Failure 400 {object} map[string]string "error"
// @Failure 401 {object} map[string]string "error"
// @Failure 500 {object} map[string]string "error"
// @Router /posts [post]
func CreatePost(c *gin.Context) {
	var post models.Post
	if err := c.ShouldBindJSON(&post); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	userID, exists := c.Get("userID")
	if !exists {
		c.JSON(http.StatusUnauthorized, gin.H{"error": "User not authenticated"})
		return
	}

	post.UserID = userID.(primitive.ObjectID)

	createdPost, err := services.CreatePost(post)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to create post"})
		return
	}

	c.JSON(http.StatusCreated, createdPost)
}
