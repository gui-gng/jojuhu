package v1

import (
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	"gitlab.com/gng1/evaluatz/user-service/services"
)

type RoutesUserV1 struct {
	routerGroup *gin.RouterGroup
	service     *services.Config
}

func (r *RoutesV1) CreateAuthGroup(service *services.Config) {
	routes := &RoutesUserV1{}
	routes.service = service
	routes.routerGroup = r.routerGroup.Group("/auth")

	routes.routerGroup.POST("/signup", routes.signup)
	routes.routerGroup.POST("/signin", routes.signin)

}

type Account struct {
	Email     string `json:"email" binding:"required"`
	FirstName string `json:"firstname" binding:"required"`
	LastName  string `json:"lastname" binding:"required"`
}

type SignUpRequest struct {
	Email     string `json:"email" binding:"required"`
	FirstName string `json:"firstname" binding:"required"`
	LastName  string `json:"lastname" binding:"required"`
	Password  string `json:"password" binding:"required"`
}

type SignUpResponse struct {
	Account Account `json:"account" binding:"required"`
}

func (r *RoutesUserV1) signup(c *gin.Context) {

	var reqBody SignUpRequest
	if err := c.ShouldBindJSON(&reqBody); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	log.Printf("Signing up user with email: %s", reqBody.Email)

	usr, err := r.service.RegisterUser(c.Request.Context(), reqBody.Email, reqBody.FirstName, reqBody.LastName, reqBody.Password)

	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}


	res := SignUpResponse{
		Account: Account{
		Email:     usr.Email,
		FirstName: usr.FirstName,
		LastName:  usr.LastName,
		},
	}

	c.JSON(201, res)
}

type SignInRequest struct {
	Email    string `json:"email" binding:"required"`
	Password string `json:"password" binding:"required"`
}

type SignInResponse struct {
	Account Account `json:"account" binding:"required"`
}

func (r *RoutesUserV1) signin(c *gin.Context) {

	var reqBody SignInRequest
	if err := c.ShouldBindJSON(&reqBody); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	log.Printf("Signing in user with email: %s", reqBody.Email)

	usr := r.service.ValidateUser(c.Request.Context(), reqBody.Email, reqBody.Password)

	res := SignInResponse{
		Account: Account{
		Email:     usr.Email,
		FirstName: usr.FirstName,
		LastName:  usr.LastName,
		},
	}

	c.JSON(200, res)
}
