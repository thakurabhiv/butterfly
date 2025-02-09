CREATE TABLE invoice_summary (
    invoice_id INTEGER AUTO_INCREMENT,
    invoice_number VARCHAR(100) NOT NULL,
    tax_id INTEGER NOT NULL,
    invoice_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    financial_year VARCHAR(100) NOT NULL,
    vendor_id INTEGER NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    pkg_charges DECIMAL(15, 2),
    taxable_amount DECIMAL(15, 2),
    tax1 DECIMAL(15, 2), -- IGST
    tax2 DECIMAL(15, 2), -- SGST
    tax3 DECIMAL(15, 2), -- CGST
    total_amount DECIMAL(15, 2),
    comments VARCHAR(300),
    created_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    modified_date TIMESTAMP,
    is_deleted BOOL DEFAULT FALSE,
    CONSTRAINT pk_invoice_id PRIMARY KEY (invoice_id),
    CONSTRAINT uk_invoice_number UNIQUE (invoice_number),
    CONSTRAINT fk_invoice_summary_tax_details FOREIGN KEY (tax_id) REFERENCES tax_details(tax_id),
    CONSTRAINT fk_invoice_summary_vendor_details FOREIGN KEY (vendor_id) REFERENCES vendor_details(vendor_id)
)