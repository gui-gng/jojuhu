package models

import "github.com/google/uuid"


type User struct {
	ID  		uuid.UUID `gorm:"type:uuid;default:uuid_generate_v4()"`
	Email    	string    `gorm:"uniqueIndex"`
	FirstName 	string   
	LastName  	string
	Password 	string
}
