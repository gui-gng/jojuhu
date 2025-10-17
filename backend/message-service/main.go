package main

import (
	"fmt"

	"github.com/gui-gng/jojuhu/message-service/router"
	"github.com/gui-gng/jojuhu/message-service/services"
)

func main() {
	fmt.Printf("Starting auth service")

	_, err := services.InitTracer("message-service")
	if err != nil {
		fmt.Println(err)
	}

	router := router.New()
	router.Run(":5002")
}
