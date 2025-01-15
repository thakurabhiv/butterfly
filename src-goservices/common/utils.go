package common

import (
	"encoding/base64"
	"fmt"
	"log/slog"

	"github.com/johnfercher/maroto/v2/pkg/props"
	"github.com/johnfercher/maroto/v2/pkg/repository"
)

func LoadCustomFont(fontPath, extension, fontName string) (bool, error) {
	if _, ok := CustomFontsMap[fontName]; ok {
		slog.Debug(fmt.Sprintf("Font %v alread loaded", fontName))
		return false, nil
	}

	fontRepo := repository.New()
	for name, style := range fontStyleMap {
		path := fmt.Sprintf("%v/%v-%v.%v", fontPath, fontName, name, extension)
		fontRepo.AddUTF8Font(fontName, style, path)
	}

	customFonts, err := fontRepo.Load()
	if err != nil {
		slog.Error(fmt.Sprintf("Unable to load font %v", fontName))
		return false, err
	}

	CustomFontsMap[fontName] = customFonts
	return true, nil
}

func Base64ToBytes(base64Str string) ([]byte, error) {
	return base64.StdEncoding.DecodeString(base64Str)
}

func FormatAmount(amount float64) string {
	return INRPrinter.Sprintf("%s %.2f", "₹", amount)
}

func CalculateTimes(strLen, threshold int) int {
	if strLen > threshold {
		return 2 + ((strLen / threshold) - 1)
	}

	return 1
}

type Counter func(...int) float64

func GetCounter(initialValue, increment float64) Counter {
	iv := initialValue

	return func(times ...int) float64 {
		if len(times) == 0 {
			iv += increment
		} else {
			iv += (increment * float64(times[0]))
		}

		return iv
	}
}

func GetLogger() *slog.Logger {
	return slog.Default()
}

func GetGrayColor() *props.Color {
	return &props.Color{
		Red:   120,
		Green: 120,
		Blue:  120,
	}
}

func GetBlueColor() *props.Color {
	return &props.Color{
		Red:   44,
		Green: 45,
		Blue:  103,
	}
}
