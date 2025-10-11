package models

import "go.mongodb.org/mongo-driver/bson/primitive"

type Post struct {
	ID     primitive.ObjectID `bson:"_id,omitempty"`
	Text   string             `bson:"text,omitempty"`
	Image  string             `bson:"image,omitempty"`
	UserID primitive.ObjectID `bson:"userid,omitempty"`
}
