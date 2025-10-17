package router

import (
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	v1 "github.com/gui-gng/jojuhu/message-service/router/v1"
	"github.com/gui-gng/jojuhu/message-service/services"
	"github.com/prometheus/client_golang/prometheus/promhttp"
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

	router.GET("/metrics", gin.WrapH(promhttp.Handler()))
	
	v1.NewGroup(router, &service)
	return router
}

