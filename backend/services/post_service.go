package services

import (
	"context"

	"jojuhu/database"
	"jojuhu/models"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func CreatePost(post models.Post) (models.Post, error) {
	collection := database.Mongo.Database("jojuhu").Collection("posts")
	_, err := collection.InsertOne(context.Background(), post)
	if err != nil {
		return models.Post{}, err
	}

	return post, nil
}

func GetUserPosts(userID primitive.ObjectID) ([]models.Post, error) {
	collection := database.Mongo.Database("jojuhu").Collection("posts")
	cursor, err := collection.Find(context.Background(), bson.M{"userid": userID})
	if err != nil {
		return nil, err
	}
	defer cursor.Close(context.Background())

	var posts []models.Post
	if err = cursor.All(context.Background(), &posts); err != nil {
		return nil, err
	}

	return posts, nil
}

func GetTimeline() ([]models.Post, error) {
	collection := database.Mongo.Database("jojuhu").Collection("posts")
	cursor, err := collection.Find(context.Background(), bson.M{})
	if err != nil {
		return nil, err
	}
	defer cursor.Close(context.Background())

	var posts []models.Post
	if err = cursor.All(context.Background(), &posts); err != nil {
		return nil, err
	}

	return posts, nil
}
