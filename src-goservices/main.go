package main

import (
	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
)

func main() {
	gin.SetMode(gin.ReleaseMode)
	r := gin.Default()

	setCORS(r)
	setHandles(r)

	// TODO either get ip and port from config or supply
	// white starting this service
	r.Run("localhost:8080")
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
