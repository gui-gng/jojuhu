package main

import (
	"log"

	"jojuhu/database"
	"jojuhu/models"
	"jojuhu/router"
)

// @title Jojuhu API
// @version 1.0
// @description This is a sample server for a social media application.
// @termsOfService http://swagger.io/terms/

// @contact.name API Support
// @contact.url http://www.swagger.io/support
// @contact.email support@swagger.io

// @license.name Apache 2.0
// @license.url http://www.apache.org/licenses/LICENSE-2.0.html

// @host localhost:5001
// @BasePath /
func main() {
	log.Println("Starting")
	db, err := database.InitDB()
	if err != nil {
		log.Fatal(err)
	}

	_, err = database.InitMongo()
	if err != nil {
		log.Fatal(err)
	}

	db.AutoMigrate(&models.User{})

	//write a log

	r := router.SetupRouter()

	log.Println("Starting on port 5001")
	r.Run(":5001")
}
