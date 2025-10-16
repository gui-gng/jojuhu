package router

import (
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	v1 "gitlab.com/gng1/evaluatz/user-service/router/v1"
	"gitlab.com/gng1/evaluatz/user-service/services"
)

func New() *gin.Engine {

	service := services.Init()

	router := gin.Default()
	router.Use(cors.Default())
	router.Use(gin.Logger())
	router.Use(gin.Recovery())
	router.Use(TraceMiddleware())
	router.GET("/health", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "ok",
		})
	})
	
	v1.NewGroup(router, &service)
	return router
}

