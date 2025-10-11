package main

import (
	"log"

	"jojuhu/database"
	"jojuhu/handlers"
	"jojuhu/models"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

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

	r := gin.Default()

	config := cors.DefaultConfig()
	config.AllowAllOrigins = true
	r.Use(cors.New(config))

	r.GET("/", handlers.HomeHandler)
	r.POST("/register", handlers.RegisterUser)
	r.POST("/login", handlers.Login)

	auth := r.Group("/")
	auth.Use(handlers.AuthMiddleware())
	auth.GET("/profile", handlers.ProfileHandler)
	auth.POST("/posts", handlers.CreatePost)
	auth.GET("/users/:userId/posts", handlers.GetUserPosts)
	auth.GET("/timeline", handlers.GetTimeline)

	log.Println("Starting on port 5001")
	r.Run(":5001")
}
