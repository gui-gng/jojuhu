package main

import (
	"fmt"

	"gitlab.com/gng1/evaluatz/user-service/router"
)

func main() {
	fmt.Printf("Starting auth service")

	router := router.New()
	router.Run(":5001")
}
