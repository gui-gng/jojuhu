package v1

import (
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
	"github.com/gui-gng/jojuhu/message-service/models"
	"github.com/gui-gng/jojuhu/message-service/services"
)

type RoutesV1 struct {
	router      *gin.Engine
	routerGroup *gin.RouterGroup
	service     *services.Config
}

type SendRequest struct {
	SenderUserId     uuid.UUID `json:"senderUserId" binding:"required"`
	ReceiverUserId uuid.UUID `json:"receiverUserId" binding:"required"`
	Msg  string `json:"msg" binding:"required"`
}

type SendResponse struct {
	SenderUserId     uuid.UUID `json:"senderUserId" binding:"required"`
	ReceiverUserId uuid.UUID `json:"receiverUserId" binding:"required"`
	Msg  string `json:"msg" binding:"required"`
}

func NewGroup(router *gin.Engine, service *services.Config) {
	routesv1 := &RoutesV1{}
	routesv1.router = router
	routesv1.service = service
	routesv1.routerGroup = router.Group("/v1")
	routesv1.routerGroup.POST("/send", routesv1.send)
	routesv1.routerGroup.POST("/list", routesv1.list)
}

func (r *RoutesV1) send(c *gin.Context) {
	var reqBody SendRequest
	if err := c.ShouldBindJSON(&reqBody); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	msg, err := r.service.Send(c.Request.Context(), reqBody.SenderUserId, reqBody.ReceiverUserId, reqBody.Msg)

	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}


	res := SendResponse{
		SenderUserId: msg.SenderUserId,
		ReceiverUserId: msg.ReceiverUserId,
		Msg: msg.Msg,
		}
	

	c.JSON(201, res)
}


type ListRequest struct {
	SenderUserId     uuid.UUID `json:"senderUserId" binding:"required"`
	ReceiverUserId uuid.UUID `json:"receiverUserId" binding:"required"`
}

type ListResponse struct {
	Messages    []models.Message `json:"messages" binding:"required"`
}

func (r *RoutesV1) list(c *gin.Context) {
	var reqBody ListRequest
	if err := c.ShouldBindJSON(&reqBody); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	msgs, err := r.service.List(c.Request.Context(), reqBody.SenderUserId, reqBody.ReceiverUserId)

	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}


	res := ListResponse{
		Messages: msgs,
		}
	

	c.JSON(201, res)
}