CREATE TABLE tax_details (
    tax_id INTEGER AUTO_INCREMENT,
    tax_name1 VARCHAR(100) NOT NULL,
    tax_name2 VARCHAR(100),
    tax_name3 VARCHAR(100),
    tax_rate1 DECIMAL(16, 2) NOT NULL,
    tax_rate2 DECIMAL(16, 2),
    tax_rate3 DECIMAL(16, 2),
    financial_year VARCHAR(100) NOT NULL,
    created_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    modified_date TIMESTAMP,
    is_deleted BOOL DEFAULT FALSE,
    CONSTRAINT pk_tax_id PRIMARY KEY (tax_id)
)