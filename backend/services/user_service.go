package services

import (
	"jojuhu/database"
	"jojuhu/models"
)

func GetProfile(email string) (models.User, error) {
	var user models.User
	if err := database.DB.Where("email = ?", email).First(&user).Error; err != nil {
		return models.User{}, err
	}
	return user, nil
}
