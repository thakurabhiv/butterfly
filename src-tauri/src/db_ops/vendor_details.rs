use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::models::{ VendorDetail, NewVendorDetails };

pub fn search(query: String, conn: &mut MysqlConnection) -> Result<Vec<VendorDetail>, DieselError> {
    use crate::schema::vendor_details::dsl::*;

    vendor_details
        .filter(vendor_name.like(format!("%{}%", query)))
        .filter(is_deleted.eq(false))
        .select(VendorDetail::as_select())
        .load(conn)
}

pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<VendorDetail>, DieselError> {
    use crate::schema::vendor_details::dsl::*;

    vendor_details
        .filter(is_deleted.eq(false))
        .select(VendorDetail::as_select())
        .load(conn)
}

pub fn save(nvd: NewVendorDetails, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::vendor_details;

    diesel::insert_into(vendor_details::table)
        .values(nvd)
        .execute(conn)
}

pub fn update(vd: VendorDetail, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::vendor_details::dsl::*;
    use crate::schema::vendor_details;

    diesel::update(vendor_details::table)
        .filter(vendor_id.eq(vd.vendor_id))
        .set(vd)
        .execute(conn)
}