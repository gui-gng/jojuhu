package services

import (
	"errors"

	"gitlab.com/gng1/evaluatz/user-service/models"
	"golang.org/x/crypto/bcrypt"
)


func (data *Config) GetUserByEmail(email string) (models.User){
	var user models.User
	data.db.First(&user, models.User{Email: email })
	return user
}

func (data *Config) RegisterUser(email string, firstname string, lastname string, password string) (*models.User, error){
	
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(password), 12)
	if err != nil {
		return nil, err
	}
	
	user := models.User{
		Email: email,
		FirstName: firstname,
		LastName: lastname,
		Password: string(hashedPassword),
	}
	r := data.db.Create(&user)

	if r.Error != nil {
		return nil, r.Error
	}

	return &user, nil
}

func (data *Config) ValidateUser(email string, password string) (models.User){
	var user models.User
	data.db.First(&user, models.User{Email: email})

	err := bcrypt.CompareHashAndPassword([]byte(user.Password), []byte(password))
	if err != nil {
		switch {
		case errors.Is(err, bcrypt.ErrMismatchedHashAndPassword):
			// invalid password
			return models.User{}
		default:
			return models.User{}
		}
	}

	return user
}
