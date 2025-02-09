CREATE TABLE id_sequence (
    seq_name VARCHAR(20) NOT NULL,
    prefix VARCHAR(10) NOT NULL,
    sec_prefix VARCHAR(10) NOT NULL,
    seq_number INTEGER NOT NULL,
    created_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    modified_date TIMESTAMP,
    CONSTRAINT pk_sequence PRIMARY KEY (seq_name, prefix, sec_prefix)
)