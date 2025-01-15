package common

import (
	"github.com/johnfercher/maroto/v2/pkg/consts/align"
	"github.com/johnfercher/maroto/v2/pkg/consts/border"
	"github.com/johnfercher/maroto/v2/pkg/consts/fontstyle"
	"github.com/johnfercher/maroto/v2/pkg/core/entity"
	"github.com/johnfercher/maroto/v2/pkg/props"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
)

var (
	Logger = GetLogger()
)

var (
	FontFolder   = "resources/fonts"
	fontStyleMap = map[string]fontstyle.Type{
		"Regular":    fontstyle.Normal,
		"Italic":     fontstyle.Italic,
		"Bold":       fontstyle.Bold,
		"BoldItalic": fontstyle.BoldItalic,
	}
	CustomFontsMap = make(map[string][]*entity.CustomFont)
)

var (
	CellFullBorderStyle = props.Cell{
		BorderColor:     GetGrayColor(),
		BorderType:      border.Full,
		BorderThickness: 0.3,
	}
	CellLeftBorderStyle = props.Cell{
		BorderColor:     GetGrayColor(),
		BorderType:      border.Left,
		BorderThickness: 0.3,
	}
	CellRightBorderStyle = props.Cell{
		BorderColor:     GetGrayColor(),
		BorderType:      border.Right,
		BorderThickness: 0.3,
	}
	HeaderTextStyle = props.Text{
		Style: fontstyle.Bold,
		Align: align.Left,
		Size:  8.5,
		Top:   0.5,
		Left:  2,
	}
	NumHeaderTextStyle = props.Text{
		Style: fontstyle.Bold,
		Align: align.Right,
		Size:  8.5,
		Top:   0.5,
		Right: 2,
	}
	CellTextStyle = props.Text{
		Align: align.Left,
		Size:  8.5,
		Top:   0.5,
		Left:  2,
	}
	CellItalicTextStyle = props.Text{
		Style: fontstyle.BoldItalic,
		Align: align.Right,
		Size:  8.5,
		Top:   0.5,
		Left:  2,
	}
	NumCellTextStyle = props.Text{
		Align: align.Right,
		Size:  8.5,
		Top:   0.5,
		Right: 2,
	}
	AddressThreshold = 37
	Currency         = "INR"
	INRPrinter       = message.NewPrinter(language.Hindi)
)
