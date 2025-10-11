package handlers

import (
	"context"
	"net/http"

	"jojuhu/database"
	"jojuhu/models"

	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/bson"
)

func GetTimeline(c *gin.Context) {
	collection := database.Mongo.Database("jojuhu").Collection("posts")
	cursor, err := collection.Find(context.Background(), bson.M{})
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to get timeline"})
		return
	}
	defer cursor.Close(context.Background())

	var posts []models.Post
	if err = cursor.All(context.Background(), &posts); err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": "Failed to decode posts"})
		return
	}

	c.JSON(http.StatusOK, posts)
}
