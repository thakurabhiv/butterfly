use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::models::NewInvoiceSummary;

pub fn save(new_invoice_summary: NewInvoiceSummary, conn: &mut MysqlConnection) -> Result<uszie, DieselError> {
    use crate::schema::invoice_summary;

    diesel::insert_into(invoice_summary::table)
        .values(new_invoice_summary)
        .execute(conn)
}