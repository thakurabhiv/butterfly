package main

import (
	"flag"
	"fmt"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

var port int64

func main() {
	flag.Int64Var(&port, "port", 8080, "Port to start server on")
	flag.Parse()

	gin.SetMode(gin.ReleaseMode)
	r := gin.Default()

	setCORS(r)
	setHandles(r)

	fmt.Printf("Starting server on port %v\n", port)
	r.Run(fmt.Sprintf(":%v", port))
}

func setHandles(e *gin.Engine) {
	e.NoRoute(handleNoRoute)
	e.POST("/invoicePdf", handleInvoicePDF)
	e.GET("/loadFont/:name/:type", handleLoadFontResoures)
}

func setCORS(e *gin.Engine) {
	e.Use(cors.New(cors.Config{
		AllowOrigins:  []string{"*"},
		AllowMethods:  []string{"GET", "POST"},
		AllowHeaders:  []string{"Origin"},
		ExposeHeaders: []string{"Content-Length"},
	}))
}
