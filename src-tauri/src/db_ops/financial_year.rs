use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::models::FinancialYear;

pub fn get_all_financal_years(conn: &mut MysqlConnection) -> Result<Vec<FinancialYear>, DieselError> {
    use crate::schema::financial_years::dsl::*;

    financial_years
        .select(FinancialYear::as_select())
        .load(conn)
}
