CREATE TABLE product_details (
    id INTEGER AUTO_INCREMENT,
    short_name VARCHAR(100) NOT NULL,
    descr VARCHAR(200),
    hsn_sac VARCHAR(100) NOT NULL,
    price DECIMAL(18, 2) NOT NULL,
    created_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    modified_date TIMESTAMP,
    is_deleted BOOL DEFAULT FALSE,
    CONSTRAINT pk_id PRIMARY KEY (id)
)
