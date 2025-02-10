package model

import (
	"fmt"

	"github.com/johnfercher/maroto/v2/pkg/components/code"
	"github.com/johnfercher/maroto/v2/pkg/components/col"
	"github.com/johnfercher/maroto/v2/pkg/components/image"
	"github.com/johnfercher/maroto/v2/pkg/components/row"
	"github.com/johnfercher/maroto/v2/pkg/components/signature"
	"github.com/johnfercher/maroto/v2/pkg/components/text"
	"github.com/johnfercher/maroto/v2/pkg/consts/align"
	"github.com/johnfercher/maroto/v2/pkg/consts/breakline"
	"github.com/johnfercher/maroto/v2/pkg/consts/fontstyle"
	"github.com/johnfercher/maroto/v2/pkg/core"
	"github.com/johnfercher/maroto/v2/pkg/props"
	"karmanya.com/goservices/common"
)

type PDFInvoiceData struct {
	BranchOwner       BranchOwner    `json:"branch_owner"`
	InvoiceSummary    InvoiceSummary `json:"invoice_summary"`
	Vendor            VendorData     `json:"vendor"`
	Transactions      []Transaction  `json:"transactions"`
	Total             Transaction    `json:"total"`
	OtherCharges      []Charges      `json:"other_charges"`
	TaxableAmount     Transaction    `json:"taxable_amount"`
	GSTCharges        []Charges      `json:"gst_charges"`
	AllTotal          Transaction    `json:"all_total"`
	Notes             []string       `json:"notes"`
	TermsAndCondition []string       `json:"terms_and_conditions"`
}

func (p *PDFInvoiceData) GetNotes() core.Col {
	initialTop := 2.0
	topCounter := common.GetCounter(2, 4.8)
	threshold := 44

	resCol := col.New(5).
		WithStyle(&common.CellFullBorderStyle).
		Add(
			text.New("Notes:", props.Text{
				Top:   initialTop,
				Left:  2,
				Style: fontstyle.Bold,
			}),
		)

	prevNote := ""
	for _, note := range p.Notes {
		resCol.Add(
			text.New(note, props.Text{
				Top:               topCounter(common.CalculateTimes(len(prevNote), threshold)),
				Left:              2,
				BreakLineStrategy: breakline.DashStrategy,
				VerticalPadding:   1,
			}),
		)

		prevNote = note
	}

	return resCol
}

func (p *PDFInvoiceData) GetTermsAndConditions() core.Col {
	initialTop := 2.0
	topCounter := common.GetCounter(2, 4.8)
	threshold := 59

	resCol := col.New(7).
		WithStyle(&common.CellFullBorderStyle).
		Add(
			text.New("Terms & Conditions", props.Text{
				Top:   initialTop,
				Left:  2,
				Style: fontstyle.Bold,
			}),
		)

	prevTC := ""
	for idx, tc := range p.TermsAndCondition {
		resCol.Add(
			text.New(fmt.Sprintf("%v) %v", idx+1, tc), props.Text{
				Top:               topCounter(common.CalculateTimes(len(prevTC), threshold)),
				Left:              2,
				BreakLineStrategy: breakline.DashStrategy,
				VerticalPadding:   1,
			}),
		)

		prevTC = tc
	}

	return resCol
}

type InvoiceSummary struct {
	Heading       string `json:"heading"`
	InvoiceNumber string `json:"invoice_number"`
	InvoiceDate   string `json:"invoice_date"`
	FinancialYear string `json:"financial_year"`
	PlaceOfSupply string `json:"place_of_supply"`
}

func (i *InvoiceSummary) GetInvoiceInfo() core.Col {
	return col.New(6).
		WithStyle(&common.CellFullBorderStyle).
		Add(
			text.New("Invoice #", props.Text{
				Top:  1,
				Left: 2,
			}),
			text.New(i.InvoiceNumber, props.Text{
				Top:   5.5,
				Left:  2,
				Style: fontstyle.Bold,
				Align: align.Left,
				Color: common.GetBlueColor(),
			}),
			text.New("Invoice Date", props.Text{
				Top:  1,
				Left: 50,
			}),
			text.New(i.InvoiceDate, props.Text{
				Left:  50,
				Top:   5.5,
				Style: fontstyle.Bold,
				Color: common.GetBlueColor(),
			}),
			text.New("Place Of Supply", props.Text{
				Top:  15,
				Left: 2,
			}),
			text.New(i.PlaceOfSupply, props.Text{
				Top:   20.5,
				Left:  2,
				Style: fontstyle.Bold,
				Color: common.GetBlueColor(),
			}),
			text.New("Financial Year", props.Text{
				Top:  15,
				Left: 50,
			}),
			text.New(i.FinancialYear, props.Text{
				Top:   20.5,
				Left:  50,
				Style: fontstyle.Bold,
				Color: common.GetBlueColor(),
			}),
		)
}

type Charges struct {
	Name    string  `json:"name"`
	Percent float32 `json:"percent"`
	Amount  float64 `json:"amount"`
}

func (c Charges) GetChargesRow() core.Row {
	desc := fmt.Sprintf("%v", c.Name)

	return row.New(5).
		WithStyle(&common.CellFullBorderStyle).
		Add(
			text.NewCol(4, desc, common.CellItalicTextStyle),
			text.NewCol(8, common.FormatAmount(c.Amount), common.NumHeaderTextStyle),
		)
}

type VendorData struct {
	Name        string  `json:"vendor_name"`
	Address     Address `json:"address"`
	PhoneNumber string  `json:"phone_number"`
	Email       string  `json:"email"`
	GSTIN       string  `json:"gstin"`
	FSSAI       string  `json:"fssai"`
}

func (v *VendorData) GetVendorDetails() core.Col {
	initialTop := 1.5
	topCounter := common.GetCounter(initialTop, 4.8)

	vendorCol := col.New(6).WithStyle(&common.CellFullBorderStyle).
		Add(
			text.New("Vendor Details:", props.Text{
				Left:  2,
				Top:   initialTop,
				Align: align.Left,
			}),
			text.New(v.Name, props.Text{
				Top:   topCounter(),
				Left:  2,
				Style: fontstyle.Bold,
				Align: align.Left,
			}),
			text.New(fmt.Sprintf("GSTIN: %v", v.GSTIN), props.Text{
				Top:   topCounter(),
				Left:  2,
				Align: align.Left,
			}),
		)

	vendorCol.Add(
		v.Address.GetAddressComponents(topCounter, 2)...,
	)

	vendorCol.Add(
		text.New(fmt.Sprintf("Mobile: %v", v.PhoneNumber), props.Text{
			Top:   topCounter(),
			Left:  2,
			Align: align.Left,
		}),
	)

	return vendorCol
}

type BranchOwner struct {
	FirstName         string  `json:"first_name"`
	MiddleName        string  `json:"middle_name"`
	LegalBusinessName string  `json:"legal_business_name"`
	LastName          string  `json:"last_name"`
	Address           Address `json:"address"`
	PhoneNumber       string  `json:"phone_number"`
	Email             string  `json:"email"`
	GSTIN             string  `json:"gstin"`
	FSSAI             string  `json:"fssai"`
	BankName          string  `json:"bank_name"`
	BranchName        string  `json:"branch_name"`
	AccountNumber     string  `json:"account_number"`
	IFSCCode          string  `json:"ifsc_code"`
	AccountHolderName string  `json:"account_holder_name"`
	UPIID             string  `json:"upi_id"`
	Signatory         string  `json:"signatory"`
	Icon              string  `json:"icon"`
}

func (b *BranchOwner) GetOwnerLogo() core.Col {
	imageBytes, err := common.Base64ToBytes(b.Icon)
	if err != nil {
		common.Logger.Error("Unable to load logo: %v", err)
	}

	return col.New(2).
		WithStyle(&common.CellLeftBorderStyle).
		Add(
			image.NewFromBytes(imageBytes, "jpg", props.Rect{
				Percent: 90,
				Left:    2,
				Top:     2,
			}),
		)
}

func (b *BranchOwner) GetOwnerDetails() core.Col {
	initialTop := 1.0
	topCounter := common.GetCounter(initialTop, 4.8)

	detailsCol := col.New(4).WithStyle(&common.CellRightBorderStyle)

	detailsCol.Add(
		text.New(b.LegalBusinessName, props.Text{
			Style: fontstyle.Bold,
			Align: align.Left,
			Top:   initialTop,
		}),
		text.New(fmt.Sprintf("GSTIN: %v", b.GSTIN), props.Text{
			Top:   topCounter(),
			Align: align.Left,
		}),
	)

	address := b.Address
	detailsCol.Add(
		address.GetAddressComponents(topCounter, 0)...,
	)

	detailsCol.Add(
		text.New(fmt.Sprintf("Mobile: %v", b.PhoneNumber), props.Text{
			Top:   topCounter(),
			Align: align.Left,
		}),
		text.New(fmt.Sprintf("Email: %v", b.Email), props.Text{
			Top:   topCounter(),
			Align: align.Left,
		}),
	)

	return detailsCol
}

func (b *BranchOwner) GetPaymentAndSignature(totalAmount float64, invoiceNumber string) core.Row {
	resRow := row.New(45)
	upiIdPresent := len(b.UPIID) > 0

	if upiIdPresent {
		resRow.Add(
			col.New(3).
				WithStyle(&common.CellLeftBorderStyle).
				Add(b.GetPaymentQRCode(totalAmount, invoiceNumber)),
		)
	}

	colLength, bankDetailsCellStyle, bankDetailsLeft := 4, common.CellRightBorderStyle, 0
	if !upiIdPresent {
		colLength, bankDetailsCellStyle, bankDetailsLeft = 7, common.CellFullBorderStyle, 2
	}

	resRow.Add(
		col.New(colLength).
			WithStyle(&bankDetailsCellStyle).
			Add(
				b.GetBankDetails(float64(bankDetailsLeft))...,
			),
		col.New(5).
			WithStyle(&common.CellFullBorderStyle).
			Add(
				b.GetOwnerSignature()...,
			),
	)

	return resRow
}

func (b *BranchOwner) GetPaymentQRCode(totalAmount float64, invoiceNumber string) core.Component {
	qrCodeStr := fmt.Sprintf(
		"upi://pay?pa=%v&am=%.2f&tn=%v&cu=%v",
		b.UPIID,
		totalAmount,
		fmt.Sprintf("Payment for invoice %v", invoiceNumber),
		common.Currency,
	)
	common.Logger.Info(fmt.Sprintf("QR Code str: %v", qrCodeStr))

	return code.NewQr(qrCodeStr, props.Rect{
		Top:     2.5,
		Left:    2,
		Percent: 90,
	})
}

func (b *BranchOwner) GetBankDetails(left float64) []core.Component {
	initialTop := 2.5
	topCounter := common.GetCounter(initialTop, 4.8)

	return []core.Component{
		text.New("Bank Details:", props.Text{
			Top:   initialTop,
			Left:  left,
			Align: align.Left,
			Style: fontstyle.Bold,
		}),
		text.New(fmt.Sprintf("Name: %v", b.AccountHolderName), props.Text{
			Top:   topCounter(),
			Left:  left,
			Align: align.Left,
		}),
		text.New(fmt.Sprintf("Bank Name: %v", b.BankName), props.Text{
			Top:   topCounter(),
			Left:  left,
			Align: align.Left,
		}),
		text.New(fmt.Sprintf("Branch Name: %v", b.BranchName), props.Text{
			Top:   topCounter(),
			Left:  left,
			Align: align.Left,
		}),
		text.New(fmt.Sprintf("Account #: %v", b.AccountNumber), props.Text{
			Top:   topCounter(),
			Left:  left,
			Align: align.Left,
		}),
		text.New(fmt.Sprintf("IFSC Code: %v", b.IFSCCode), props.Text{
			Top:   topCounter(),
			Left:  left,
			Align: align.Left,
		}),
	}
}

func (b *BranchOwner) GetOwnerSignature() []core.Component {
	signComps := []core.Component{
		signature.New("Authorised Signatory", props.Signature{
			FontStyle:     fontstyle.Bold,
			FontSize:      11,
			FontColor:     common.GetBlueColor(),
			LineThickness: 0.3,
		}),
	}

	// add proprietor name
	var proprietorName string
	if len(b.MiddleName) > 0 {
		proprietorName = fmt.Sprintf("%v %v %v", b.FirstName, b.MiddleName, b.LastName)
	} else {
		proprietorName = fmt.Sprintf("%v %v", b.FirstName, b.LastName)
	}
	signComps = append(signComps, text.New(fmt.Sprintf("For %v", proprietorName), props.Text{
		Top:   2,
		Align: align.Middle,
		Style: fontstyle.Bold,
		Size:  10,
	}))

	if len(b.Signatory) == 0 {
		return signComps
	}

	signatureBytes, err := common.Base64ToBytes(b.Signatory)
	if err != nil {
		common.Logger.Error("Error while getting signature", err)
	} else {
		common.Logger.Info("Adding signature image")
		signComps = append(signComps, image.NewFromBytes(signatureBytes, "jpg", props.Rect{
			Center:  true,
			Percent: 80,
		}))
	}

	return signComps
}

type Address struct {
	AddressLines []string `json:"address_lines"`
	State        string   `json:"state"`
	Pincode      int      `json:"pin_code"`
}

func (a *Address) GetAddressComponents(topCounter common.Counter, left float64) []core.Component {
	var comps []core.Component

	var prevStr string
	for _, address := range a.AddressLines {
		comps = append(comps, text.New(address, props.Text{
			Top:               topCounter(common.CalculateTimes(len(prevStr), common.AddressThreshold)),
			Left:              left,
			Align:             align.Left,
			BreakLineStrategy: breakline.DashStrategy,
			VerticalPadding:   1,
		}))

		prevStr = address
	}

	comps = append(comps, text.New(fmt.Sprintf("%v, %v", a.State, a.Pincode), props.Text{
		Top:               topCounter(common.CalculateTimes(len(prevStr), common.AddressThreshold)),
		Left:              left,
		Align:             align.Left,
		BreakLineStrategy: breakline.DashStrategy,
		VerticalPadding:   1,
	}))

	return comps
}

type Transaction struct {
	ProductName   string    `json:"product_name"`
	HSN           string    `json:"hsn_sac"`
	Unit          string    `json:"unit"`
	Quantity      int       `json:"quantity"`
	PricePerUnit  float64   `json:"price_per_unit"`
	NoOfBags      int       `json:"no_of_bags"`
	Amount        float64   `json:"amount"`
	ApplicabelGST []Charges `json:"applicable_gst"`
	TotalAmount   string    `json:"total_amount"`
}

func (t Transaction) GetHeader() core.Row {
	return row.New(8).Add(
		text.NewCol(1, "Sr No", common.HeaderTextStyle).WithStyle(&common.CellFullBorderStyle),
		text.NewCol(3, "Item Name", common.HeaderTextStyle).WithStyle(&common.CellFullBorderStyle),
		text.NewCol(1, "HSN", common.HeaderTextStyle).WithStyle(&common.CellFullBorderStyle),
		text.NewCol(1, "No. Of Bags", common.NumHeaderTextStyle).WithStyle(&common.CellFullBorderStyle),
		text.NewCol(1, "Quantity", common.NumHeaderTextStyle).WithStyle(&common.CellFullBorderStyle),
		text.NewCol(1, "Unit", common.HeaderTextStyle).WithStyle(&common.CellFullBorderStyle),
		text.NewCol(2, "Price/Unit", common.NumHeaderTextStyle).WithStyle(&common.CellFullBorderStyle),
		text.NewCol(2, "Amount", common.NumHeaderTextStyle).WithStyle(&common.CellFullBorderStyle),
	)
}

func (t Transaction) GetContent(i int) core.Row {
	row := row.New(10).Add(
		text.NewCol(1, fmt.Sprintf("%v", i+1), common.HeaderTextStyle).
			WithStyle(&common.CellFullBorderStyle),
		text.NewCol(3, t.ProductName, common.HeaderTextStyle).
			WithStyle(&common.CellFullBorderStyle),
		text.NewCol(1, t.HSN, common.CellTextStyle).
			WithStyle(&common.CellFullBorderStyle),
		text.NewCol(1, fmt.Sprintf("%v", t.NoOfBags), common.NumCellTextStyle).
			WithStyle(&common.CellFullBorderStyle),
		text.NewCol(1, fmt.Sprintf("%v", t.Quantity), common.NumCellTextStyle).
			WithStyle(&common.CellFullBorderStyle),
		text.NewCol(1, t.Unit, common.CellTextStyle).
			WithStyle(&common.CellFullBorderStyle),
		text.NewCol(2, common.FormatAmount(t.PricePerUnit), common.NumCellTextStyle).
			WithStyle(&common.CellFullBorderStyle),
		text.NewCol(2, common.FormatAmount(t.Amount), common.NumCellTextStyle).
			WithStyle(&common.CellFullBorderStyle),
	)

	return row
}

func GetDummyRows(numRows int) []core.Row {
	rows := make([]core.Row, 0, numRows)

	for i := 0; i < numRows; i++ {
		rows = append(rows, row.New(10).
			Add(
				col.New(1).WithStyle(&common.CellFullBorderStyle),
				col.New(3).WithStyle(&common.CellFullBorderStyle),
				col.New(1).WithStyle(&common.CellFullBorderStyle),
				col.New(1).WithStyle(&common.CellFullBorderStyle),
				col.New(1).WithStyle(&common.CellFullBorderStyle),
				col.New(1).WithStyle(&common.CellFullBorderStyle),
				col.New(2).WithStyle(&common.CellFullBorderStyle),
				col.New(2).WithStyle(&common.CellFullBorderStyle),
			))
	}

	return rows
}

func (t Transaction) GetAmountRow() core.Row {
	resRow := row.New(5).
		WithStyle(&common.CellFullBorderStyle).
		Add(
			text.NewCol(4, t.ProductName, props.Text{
				Style: fontstyle.BoldItalic,
				Align: align.Right,
				Size:  8.5,
				Top:   0.5,
			}),
		)

	if t.NoOfBags != 0 && t.Quantity != 0 {
		resRow.Add(
			text.NewCol(2, fmt.Sprintf("%v", t.NoOfBags), common.NumHeaderTextStyle),
			text.NewCol(1, fmt.Sprintf("%v", t.Quantity), common.NumHeaderTextStyle),
			text.NewCol(5, common.FormatAmount(t.Amount), common.NumHeaderTextStyle),
		)
	} else {
		resRow.Add(
			text.NewCol(8, common.FormatAmount(t.Amount), common.NumHeaderTextStyle),
		)
	}

	return resRow
}
