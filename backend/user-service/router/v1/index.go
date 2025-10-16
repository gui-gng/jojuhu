package v1

import (
	"github.com/gin-gonic/gin"
	"gitlab.com/gng1/evaluatz/user-service/services"
)

type RoutesV1 struct {
	router      *gin.Engine
	routerGroup *gin.RouterGroup
}

func NewGroup(router *gin.Engine, service *services.Config) {
	routesv1 := &RoutesV1{}
	routesv1.router = router
	routesv1.routerGroup = router.Group("/v1")
	routesv1.CreateAuthGroup(service)
}

