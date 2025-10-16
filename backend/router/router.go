package router

import (
	"jojuhu/handlers"
	"jojuhu/docs"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	swaggerFiles "github.com/swaggo/files"
	ginSwagger "github.com/swaggo/gin-swagger"
)

func SetupRouter() *gin.Engine {
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

	docs.SwaggerInfo.BasePath = "/"
	r.GET("/swagger/*any", ginSwagger.WrapHandler(swaggerFiles.Handler))

	return r
}
