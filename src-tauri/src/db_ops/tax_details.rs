use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::models::{ TaxDetail, NewTaxDetails };

pub fn has_tax_details_for(fy: String, conn: &mut MysqlConnection) -> Result<bool, DieselError> {
    use crate::schema::tax_details::dsl::*;

    let count: i64 = tax_details
        .filter(is_deleted.eq(false))
        .filter(financial_year.eq(fy))
        .count()
        .get_result(conn)?;

    Ok(count > 0)
}

pub fn find_all(fy: String, conn: &mut MysqlConnection) -> Result<Vec<TaxDetail>, DieselError> {
    use crate::schema::tax_details::dsl::*;

    tax_details
        .filter(is_deleted.eq(false))
        .filter(financial_year.eq(fy))
        .order(created_date.desc())
        .select(TaxDetail::as_select())
        .load(conn)
}

pub fn find_latest(fy: String, conn: &mut MysqlConnection) -> Result<TaxDetail, DieselError> {
    use crate::schema::tax_details::dsl::*;

    tax_details
        .filter(is_deleted.eq(false))
        .filter(financial_year.eq(fy))
        .order(created_date.desc())
        .first(conn)
}

pub fn get_all_financial_year_list(conn: &mut MysqlConnection) -> Result<Vec<String>, DieselError> {
    use crate::schema::tax_details::dsl::*;

    tax_details
        .order(financial_year.asc())
        .select(financial_year)
        .distinct()
        .load(conn)
}

pub fn save(new_tax_details: NewTaxDetails, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::tax_details;

    diesel::insert_into(tax_details::table)
        .values(&new_tax_details)
        .execute(conn)
}

pub fn update(td: TaxDetail, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::tax_details;
    use crate::schema::tax_details::dsl::*;

    diesel::update(tax_details::table)
        .filter(tax_id.eq(td.tax_id))
        .set(td)
        .execute(conn)
}