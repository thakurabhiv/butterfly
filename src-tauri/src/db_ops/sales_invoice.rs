use crate::constants::{SALES_INVOICE_PREFIX, SALES_INVOICE_SEQ_NAME};
use crate::db_ops::id_sequence;
use crate::db_ops::product_details as pd_db;
use crate::models::{
    IdSequence, InvoiceDetail, InvoiceDetailsWithProduct, InvoiceSummary, NewIdSequence,
    NewInvoiceDetail, NewInvoiceSummary, TaxDetail, VendorDetail,
};
use chrono;
use diesel::prelude::*;
use diesel::result::{Error as DieselError, QueryResult};

pub fn get_all_financial_year_list(conn: &mut MysqlConnection) -> Result<Vec<String>, DieselError> {
    use crate::schema::invoice_summary::dsl::*;

    invoice_summary
        .order(financial_year.asc())
        .select(financial_year)
        .distinct()
        .load(conn)
}

pub fn save_invoice(
    new_invoice_summary: NewInvoiceSummary,
    mut new_invoice_details: Vec<NewInvoiceDetail>,
    fy: String,
    conn: &mut MysqlConnection,
) -> Result<(), DieselError> {
    use crate::schema::{invoice_details, invoice_summary};

    // save sales invoice data using transaction
    conn.transaction(|connection| {
        log::debug!("Saving invoice summary!");
        // insert invoice summary
        diesel::insert_into(invoice_summary::dsl::invoice_summary)
            .values(new_invoice_summary)
            .execute(connection)?;
        log::debug!("Invoice summary saved!");

        log::debug!("Retriving");
        let latest_result: Vec<InvoiceSummary> = invoice_summary::dsl::invoice_summary
            .order(invoice_summary::dsl::invoice_id.desc())
            .select(InvoiceSummary::as_select())
            .load(connection)?;
        log::debug!("Retrived");

        if latest_result.len() == 0 {
            return Err(DieselError::NotFound);
        }
        let latest_row = &latest_result[0];

        new_invoice_details.iter_mut().for_each(|value| {
            log::debug!("Setting new invoice id {}", latest_row.invoice_id);
            value.invoice_id = latest_row.invoice_id;
        });

        log::debug!("Inserting into invoice_details");
        // insert invoice details
        diesel::insert_into(invoice_details::dsl::invoice_details)
            .values(new_invoice_details)
            .execute(connection)?;
        log::debug!("Inserted into invoice_details");

        // update id sequence
        update_id_sequence(
            SALES_INVOICE_SEQ_NAME.to_string(),
            SALES_INVOICE_PREFIX.to_string(),
            fy,
            connection,
        )?;
        log::debug!("Id sequence updated");
        QueryResult::Ok(())
    })?;

    Ok(())
}

pub fn update_invoice(
    mut inv_summary: InvoiceSummary,
    inv_details: Vec<InvoiceDetail>,
    conn: &mut MysqlConnection,
) -> Result<(), DieselError> {
    use crate::schema::{invoice_details, invoice_summary};

    conn.transaction(|connection| {
        let invoice_id = inv_summary.invoice_id.clone();
        inv_summary.modified_date = Some(chrono::Utc::now().naive_utc());

        log::debug!("Updating invoice summary for invoice id {} !", invoice_id);
        diesel::update(invoice_summary::table)
            .filter(invoice_summary::dsl::invoice_id.eq(inv_summary.invoice_id))
            .set(inv_summary)
            .execute(connection)?;
        log::debug!("Updated invoice summary");

        // get all products that need to updated
        let inv_prod_ids: Vec<i32> = inv_details
            .iter()
            .filter(|inv_detail| inv_detail.id != 0)
            .map(|inv_detail| inv_detail.id)
            .collect();

        // get all existing product from database
        let existing_prod_ids: Vec<i32> = invoice_details::dsl::invoice_details
            .filter(invoice_details::dsl::invoice_id.eq(invoice_id))
            .select(invoice_details::id)
            .load(connection)?;

        let difference: Vec<i32> = existing_prod_ids
            .into_iter()
            .filter(|id| !inv_prod_ids.contains(id))
            .collect();

        log::debug!("Product to be deleted: {:?}", difference);
        diesel::update(invoice_details::table)
            .filter(invoice_details::dsl::id.eq_any(difference))
            .set(invoice_details::dsl::is_deleted.eq(true))
            .execute(connection)?;
        log::debug!("Products mark as deleted");

        log::debug!("Updating invoice details !");
        for mut inv_detail in inv_details {
            if inv_detail.id == 0 {
                inv_detail.invoice_id = invoice_id;
                let new_invoice_details = NewInvoiceDetail::from(&inv_detail);
                log::debug!("Invoice summary adding: {:?}", new_invoice_details);

                diesel::insert_into(invoice_details::table)
                    .values(new_invoice_details)
                    .execute(connection)
            } else {
                inv_detail.modified_date = Some(chrono::Utc::now().naive_utc());
                log::debug!("Invoice summary updating: {:?}", inv_detail);

                diesel::update(invoice_details::table)
                    .filter(invoice_details::dsl::id.eq(inv_detail.id))
                    .set(inv_detail)
                    .execute(connection)
            }?;
        }
        log::debug!("Updated invoice details !");

        log::debug!("Sales invoice updated!");

        QueryResult::Ok(())
    })?;

    Ok(())
}

pub fn next_invoice_id(
    sec_prefix: String,
    conn: &mut MysqlConnection,
) -> Result<String, DieselError> {
    // create id_seq object from parameters
    let mut id_seq = IdSequence::default();
    id_seq.seq_name = SALES_INVOICE_SEQ_NAME.to_string();
    id_seq.prefix = SALES_INVOICE_PREFIX.to_string();
    id_seq.sec_prefix = sec_prefix.clone();

    let existing_seq_res = id_sequence::find_first(id_seq, conn);
    if existing_seq_res.is_ok() {
        // if present, increment seq number and update same in table
        let mut existing_seq = existing_seq_res?;
        existing_seq.seq_number += 1;

        Ok(format!(
            "{}{}{}",
            existing_seq.prefix, existing_seq.sec_prefix, existing_seq.seq_number
        ))
    } else {
        Ok(format!("{}{}{}", SALES_INVOICE_PREFIX, sec_prefix, 1))
    }
}

fn update_id_sequence(
    seq_name: String,
    prefix: String,
    sec_prefix: String,
    conn: &mut MysqlConnection,
) -> Result<usize, DieselError> {
    let mut id_seq = IdSequence::default();
    id_seq.seq_name = seq_name.clone();
    id_seq.prefix = prefix.clone();
    id_seq.sec_prefix = sec_prefix.clone();

    // try to fetch seq data
    match id_sequence::find_first(id_seq, conn) {
        Ok(mut record) => {
            record.seq_number += 1;
            record.modified_date = Some(chrono::Utc::now().naive_utc());

            id_sequence::update_seq_number(record, conn)
        }
        Err(_) => {
            let mut new_id_seq = NewIdSequence::default();
            new_id_seq.seq_name = seq_name;
            new_id_seq.prefix = prefix;
            new_id_seq.sec_prefix = sec_prefix;
            new_id_seq.seq_number = 1;

            id_sequence::save(new_id_seq, conn)
        }
    }
}

pub fn search(
    query: String,
    conn: &mut MysqlConnection,
) -> Result<Vec<InvoiceSummary>, DieselError> {
    use crate::schema::invoice_summary::dsl::*;

    invoice_summary
        .filter(is_deleted.eq(false))
        .filter(invoice_number.like(format!("%{}%", query)))
        .select(InvoiceSummary::as_select())
        .load(conn)
}

pub fn get_invoice_with_details(
    invoice_summary: InvoiceSummary,
    conn: &mut MysqlConnection,
) -> Result<
    (
        InvoiceSummary,
        VendorDetail,
        TaxDetail,
        Vec<InvoiceDetailsWithProduct>,
    ),
    DieselError,
> {
    use crate::schema::invoice_details;
    use crate::schema::tax_details;
    use crate::schema::vendor_details;

    let vd = vendor_details::dsl::vendor_details
        .filter(vendor_details::dsl::vendor_id.eq(invoice_summary.vendor_id))
        .select(VendorDetail::as_select())
        .first(conn)?;

    let td: TaxDetail = tax_details::dsl::tax_details
        .filter(tax_details::dsl::tax_id.eq(invoice_summary.tax_id))
        .select(TaxDetail::as_select())
        .first(conn)?;

    let details: Vec<InvoiceDetail> = invoice_details::dsl::invoice_details
        .filter(invoice_details::dsl::invoice_id.eq(invoice_summary.invoice_id))
        .filter(invoice_details::dsl::is_deleted.eq(false))
        .select(InvoiceDetail::as_select())
        .load(conn)?;

    let de = details
        .into_iter()
        .map(|inv_detail| {
            let pd = pd_db::get_product(inv_detail.product_id, conn).unwrap();
            InvoiceDetailsWithProduct {
                invoice_detail: inv_detail,
                product_detail: pd,
            }
        })
        .collect::<Vec<InvoiceDetailsWithProduct>>();

    Ok((invoice_summary, vd, td, de))
}
