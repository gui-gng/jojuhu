package main

import (
	"fmt"

	"gitlab.com/gng1/evaluatz/user-service/router"
	"gitlab.com/gng1/evaluatz/user-service/services"
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
