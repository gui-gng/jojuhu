package main

import (
	"log"

	"jojuhu/database"
	"jojuhu/handlers"
	"jojuhu/models"

	"github.com/gin-gonic/gin"
)

func main() {
	log.Println("Starting")
	db, err := database.InitDB()
	if err != nil {
		log.Fatal(err)
	}

	db.AutoMigrate(&models.User{})

	//write a log

	r := gin.Default()
	r.GET("/", handlers.HomeHandler)
	r.POST("/register", handlers.RegisterUser)
	r.POST("/login", handlers.Login)

	auth := r.Group("/")
	auth.Use(handlers.AuthMiddleware())
	auth.GET("/profile", handlers.ProfileHandler)

	r.Run(":8080")
}
