package pdf

import (
	"fmt"
	"log"

	"github.com/johnfercher/maroto/v2"
	"github.com/johnfercher/maroto/v2/pkg/components/list"
	"github.com/johnfercher/maroto/v2/pkg/components/row"
	"github.com/johnfercher/maroto/v2/pkg/components/text"
	"github.com/johnfercher/maroto/v2/pkg/config"
	"github.com/johnfercher/maroto/v2/pkg/consts/align"
	"github.com/johnfercher/maroto/v2/pkg/consts/fontstyle"
	"github.com/johnfercher/maroto/v2/pkg/consts/pagesize"
	"github.com/johnfercher/maroto/v2/pkg/core"
	"github.com/johnfercher/maroto/v2/pkg/props"
	"karmanya.com/goservices/common"
	"karmanya.com/goservices/model"
)

func GenerateInvoice(pdfInvoiceData *model.PDFInvoiceData) ([]byte, error) {
	m, err := getMaroto(pdfInvoiceData)
	if err != nil {
		log.Print("Error while generating PDF", err)
		return nil, err
	}

	document, err := (*m).Generate()
	if err != nil {
		log.Print("Error while generating PDF", err)
		return nil, err
	}
	return document.GetBytes(), err
}

func getMaroto(invoiceData *model.PDFInvoiceData) (*core.Maroto, error) {
	cfgBuilder := config.NewBuilder().
		WithPageNumber("Page {current}/{total}", props.LeftBottom).
		WithPageSize(pagesize.A4).
		WithMargins(10, 10, 10)

	for fontName, customFont := range common.CustomFontsMap {
		common.Logger.Info(fmt.Sprintf("Added %v font in Maroto configuration", fontName))
		cfgBuilder.WithCustomFonts(customFont)
	}

	cfgBuilder.WithDefaultFont(&props.Font{
		Family: "Roboto",
	})
	cfg := cfgBuilder.Build()

	m := maroto.New(cfg)

	branchOwner := invoiceData.BranchOwner
	invoiceSummary := invoiceData.InvoiceSummary
	vendorData := invoiceData.Vendor
	transactions := invoiceData.Transactions
	total := invoiceData.Total
	otherCharges := invoiceData.OtherCharges
	taxableAmount := invoiceData.TaxableAmount
	gstCharges := invoiceData.GSTCharges
	allTotal := invoiceData.AllTotal

	transactionRows, err := list.Build[model.Transaction](transactions)
	if err != nil && err.Error() == "empty array" {
		common.Logger.Warn("Transactions are empty")
	}

	m.AddRows(
		getPageHeader("TAX INVOICE"),
		row.New(40).Add(
			branchOwner.GetOwnerLogo(),
			branchOwner.GetOwnerDetails(),
			invoiceSummary.GetInvoiceInfo(),
		),
		row.New(40).
			WithStyle(&common.CellFullBorderStyle).
			Add(
				vendorData.GetVendorDetails(),
			),
	)

	// transactions
	transactionRowLen := len(transactionRows)
	if transactionRowLen > 0 {
		m.AddRows(transactionRows...)

		if transactionRowLen <= 3 {
			m.AddRows(model.GetDummyRows(4 - transactionRowLen)...)
		}
	} else {
		m.AddRows(model.GetDummyRows(2)...)
	}
	// total
	m.AddRows(total.GetAmountRow())

	// other charges
	for _, otherCharge := range otherCharges {
		m.AddRows(otherCharge.GetChargesRow())
	}

	// taxable amount
	m.AddRows(taxableAmount.GetAmountRow())

	// adding gst
	for _, gstCharge := range gstCharges {
		m.AddRows(gstCharge.GetChargesRow())
	}

	// all total
	m.AddRows(allTotal.GetAmountRow())

	// qr and signature
	m.AddRows(
		branchOwner.GetPaymentAndSignature(allTotal.Amount, invoiceSummary.InvoiceNumber),
	)

	m.AddRow(40).Add(
		invoiceData.GetNotes(),
		invoiceData.GetTermsAndConditions(),
	)

	return &m, nil
}

func getPageHeader(heading string) core.Row {
	return text.NewRow(8, heading, props.Text{
		Top:   0.7,
		Style: fontstyle.Bold,
		Align: align.Center,
		Size:  15,
		Color: common.GetBlueColor(),
	}).WithStyle(&common.CellFullBorderStyle)
}
