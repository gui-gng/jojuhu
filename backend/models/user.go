package models

import (
	"gorm.io/gorm"
)

// User represents a user in the social network
type User struct {
	gorm.Model
	Username string `json:"username"`
	Email    string `json:"email" gorm:"unique"`
	Password string `json:"password"`
}
