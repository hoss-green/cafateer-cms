package main

import (
	//"fmt"
	"net/http"

  "github.com/labstack/echo/v4"
)

func main() {
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, Go World")
	})

	e.Logger.Fatal(e.Start(":4445"))
}

func server() {
}
