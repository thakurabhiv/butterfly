CREATE TABLE branch_owner_details (
    id INTEGER AUTO_INCREMENT,
    first_name VARCHAR(100) NOT NULL,
    middle_name VARCHAR(100),
    last_name VARCHAR(100) NOT NULL,
    legal_business_name VARCHAR(100) NOT NULL,
    birth_date DATETIME,
    address1 VARCHAR(100),
    address2 VARCHAR(100),
    address3 VARCHAR(100),
    address4 VARCHAR(100),
    state_id INTEGER NOT NULL,
    pin_code VARCHAR(100),
    phone_number VARCHAR(100),
    email_id VARCHAR(100),
    gstin VARCHAR(100),
    fssai VARCHAR(100),
    bank_name VARCHAR(100),
    branch_name VARCHAR(100),
    account_number VARCHAR(100),
    ifsc_code VARCHAR(100),
    account_holder_name VARCHAR(100),
    upi_id VARCHAR(100),
    signatory BLOB,
    icon BLOB,
    created_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    modified_date TIMESTAMP,
    is_deleted BOOL DEFAULT FALSE,
    CONSTRAINT pk_id PRIMARY KEY (id),
    CONSTRAINT fk_branch_owner_details_state_list FOREIGN KEY (state_id) REFERENCES state_list(id)
)