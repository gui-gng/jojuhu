package main

import (
	"fmt"

	"github.com/gui-gng/jojuhu/user-service/router"
	"github.com/gui-gng/jojuhu/user-service/services"
)

func main() {
	fmt.Printf("Starting auth service")

	_, err := services.InitTracer("user-service")
	if err != nil {
		fmt.Println(err)
	}

	router := router.New()
	router.Run(":5001")
}
