package main

import (
	"fmt"
	"log"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
	"karmanya.com/goservices/common"
	"karmanya.com/goservices/model"
	"karmanya.com/goservices/pdf"
)

func handleNoRoute(c *gin.Context) {
	fmt.Printf("No route found for url: %v", c.Request.URL)

	c.JSON(http.StatusNotFound, gin.H{
		"staus": 0,
		"error": fmt.Sprintf("No route found for url: %v", c.Request.URL),
	})
}

func handleInvoicePDF(c *gin.Context) {
	var pdfInvoiceData model.PDFInvoiceData

	err := c.BindJSON(&pdfInvoiceData)
	if err != nil {
		log.Println("Error while binding invoice json", err)
		c.JSON(http.StatusInternalServerError, gin.H{
			"staus": 0,
			"error": fmt.Sprintf("Error while binding invoice json: %v", err.Error()),
		})

		return
	}

	invoicePdf, err := pdf.GenerateInvoice(&pdfInvoiceData)
	if err != nil {
		log.Println("Error while generating pdf ", err)
		c.JSON(http.StatusInternalServerError, gin.H{
			"staus": 0,
			"error": fmt.Sprintf("Error while generating pdf: %v", err.Error()),
		})

		return
	}

	c.Data(http.StatusOK, "application/pdf", invoicePdf)
}

func handleLoadFontResoures(c *gin.Context) {
	fontName := strings.TrimSpace(c.Param("name"))
	extension := strings.TrimSpace(c.Param("type"))

	if len(fontName) == 0 {
		c.JSON(http.StatusInternalServerError, gin.H{
			"success": 0,
			"error":   "Font name required",
		})
		return
	}

	if len(extension) == 0 {
		c.JSON(http.StatusInternalServerError, gin.H{
			"success": 0,
			"error":   "Font type required",
		})
		return
	}

	fontPath := fmt.Sprintf("%v/%v", common.FontFolder, fontName)
	ok, err := common.LoadCustomFont(fontPath, extension, fontName)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"success": 0,
			"error":   err.Error(),
		})
		return
	}

	if !ok {
		c.JSON(http.StatusNotModified, gin.H{
			"success": 0,
			"message": fmt.Sprintf("%v font already loaded", fontName),
		})
		return
	}

	c.JSON(http.StatusOK, gin.H{
		"status":  1,
		"message": fmt.Sprintf("%v font loaded successfully", fontName),
	})
}
