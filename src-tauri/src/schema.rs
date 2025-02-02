// @generated automatically by Diesel CLI.

diesel::table! {
    branch_owner_details (id) {
        id -> Integer,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        middle_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 100]
        legal_business_name -> Varchar,
        birth_date -> Nullable<Datetime>,
        #[max_length = 100]
        address1 -> Nullable<Varchar>,
        #[max_length = 100]
        address2 -> Nullable<Varchar>,
        #[max_length = 100]
        address3 -> Nullable<Varchar>,
        #[max_length = 100]
        address4 -> Nullable<Varchar>,
        state_id -> Integer,
        #[max_length = 100]
        pin_code -> Nullable<Varchar>,
        #[max_length = 100]
        phone_number -> Nullable<Varchar>,
        #[max_length = 100]
        email_id -> Nullable<Varchar>,
        #[max_length = 100]
        gstin -> Nullable<Varchar>,
        #[max_length = 100]
        fssai -> Nullable<Varchar>,
        #[max_length = 100]
        bank_name -> Nullable<Varchar>,
        #[max_length = 100]
        branch_name -> Nullable<Varchar>,
        #[max_length = 100]
        account_number -> Nullable<Varchar>,
        #[max_length = 100]
        ifsc_code -> Nullable<Varchar>,
        #[max_length = 100]
        account_holder_name -> Nullable<Varchar>,
        #[max_length = 100]
        upi_id -> Nullable<Varchar>,
        signatory -> Nullable<Blob>,
        icon -> Nullable<Blob>,
        created_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    financial_years (id) {
        id -> Integer,
        #[max_length = 10]
        financial_year -> Varchar,
    }
}

diesel::table! {
    id_sequence (seq_name, prefix, sec_prefix) {
        #[max_length = 20]
        seq_name -> Varchar,
        #[max_length = 10]
        prefix -> Varchar,
        #[max_length = 10]
        sec_prefix -> Varchar,
        seq_number -> Integer,
        created_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    invoice_details (id) {
        id -> Integer,
        invoice_id -> Integer,
        product_id -> Integer,
        quantity -> Integer,
        #[max_length = 100]
        unit -> Nullable<Varchar>,
        price_per_unit -> Decimal,
        no_of_bags -> Nullable<Integer>,
        amount -> Decimal,
        tax1 -> Nullable<Decimal>,
        tax2 -> Nullable<Decimal>,
        tax3 -> Nullable<Decimal>,
        total_amount -> Nullable<Decimal>,
        created_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    invoice_summary (invoice_id) {
        invoice_id -> Integer,
        #[max_length = 100]
        invoice_number -> Varchar,
        tax_id -> Integer,
        invoice_date -> Nullable<Timestamp>,
        #[max_length = 100]
        financial_year -> Varchar,
        vendor_id -> Integer,
        amount -> Decimal,
        pkg_charges -> Nullable<Decimal>,
        tax1 -> Nullable<Decimal>,
        tax2 -> Nullable<Decimal>,
        tax3 -> Nullable<Decimal>,
        taxable_amount -> Nullable<Decimal>,
        total_amount -> Nullable<Decimal>,
        #[max_length = 300]
        comments -> Nullable<Varchar>,
        created_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    product_details (id) {
        id -> Integer,
        #[max_length = 100]
        short_name -> Varchar,
        #[max_length = 200]
        descr -> Nullable<Varchar>,
        #[max_length = 100]
        hsn_sac -> Varchar,
        price -> Decimal,
        created_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    state_list (id) {
        id -> Integer,
        #[max_length = 100]
        state_name -> Varchar,
        #[max_length = 2]
        tin -> Varchar,
        #[max_length = 50]
        iso -> Varchar,
        #[max_length = 100]
        capital -> Varchar,
        #[max_length = 2]
        vehicle_code -> Varchar,
    }
}

diesel::table! {
    tax_details (tax_id) {
        tax_id -> Integer,
        #[max_length = 100]
        tax_name1 -> Varchar,
        #[max_length = 100]
        tax_name2 -> Nullable<Varchar>,
        #[max_length = 100]
        tax_name3 -> Nullable<Varchar>,
        tax_rate1 -> Decimal,
        tax_rate2 -> Nullable<Decimal>,
        tax_rate3 -> Nullable<Decimal>,
        #[max_length = 100]
        financial_year -> Varchar,
        created_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::table! {
    vendor_details (vendor_id) {
        vendor_id -> Integer,
        #[max_length = 100]
        vendor_name -> Varchar,
        #[max_length = 100]
        address1 -> Nullable<Varchar>,
        #[max_length = 100]
        address2 -> Nullable<Varchar>,
        #[max_length = 100]
        address3 -> Nullable<Varchar>,
        #[max_length = 100]
        address4 -> Nullable<Varchar>,
        state_id -> Integer,
        #[max_length = 100]
        pin_code -> Nullable<Varchar>,
        #[max_length = 100]
        phone_number -> Nullable<Varchar>,
        #[max_length = 100]
        email_id -> Nullable<Varchar>,
        #[max_length = 100]
        vgstin -> Nullable<Varchar>,
        #[max_length = 100]
        fssai -> Nullable<Varchar>,
        #[max_length = 100]
        bank_name -> Nullable<Varchar>,
        #[max_length = 100]
        branch_name -> Nullable<Varchar>,
        #[max_length = 100]
        account_number -> Nullable<Varchar>,
        #[max_length = 100]
        ifsc_code -> Nullable<Varchar>,
        #[max_length = 100]
        account_holder_name -> Nullable<Varchar>,
        created_date -> Nullable<Timestamp>,
        modified_date -> Nullable<Timestamp>,
        is_deleted -> Nullable<Bool>,
    }
}

diesel::joinable!(branch_owner_details -> state_list (state_id));
diesel::joinable!(invoice_details -> invoice_summary (invoice_id));
diesel::joinable!(invoice_details -> product_details (product_id));
diesel::joinable!(invoice_summary -> tax_details (tax_id));
diesel::joinable!(invoice_summary -> vendor_details (vendor_id));
diesel::joinable!(vendor_details -> state_list (state_id));

diesel::allow_tables_to_appear_in_same_query!(
    branch_owner_details,
    financial_years,
    id_sequence,
    invoice_details,
    invoice_summary,
    product_details,
    state_list,
    tax_details,
    vendor_details,
);
