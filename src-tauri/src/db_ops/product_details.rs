use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::models::{ NewProductDetails, ProductDetail };

pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<ProductDetail>, DieselError> {
    use crate::schema::product_details::dsl::*;

    product_details
        .filter(is_deleted.eq(false))
        .select(ProductDetail::as_select())
        .load(conn)
}

pub fn search(query: String, conn: &mut MysqlConnection) -> Result<Vec<ProductDetail>, DieselError> {
    use crate::schema::product_details::dsl::*;

    product_details
        .filter(short_name.like(format!("%{}%", query)))
        .filter(is_deleted.eq(false))
        .select(ProductDetail::as_select())
        .load(conn)
}

pub fn get_product(prod_id: i32, conn: &mut MysqlConnection) -> Result<ProductDetail, DieselError> {
    use crate::schema::product_details::dsl::*;

    product_details
        .filter(is_deleted.eq(false))
        .filter(id.eq(prod_id))
        .select(ProductDetail::as_select())
        .first(conn)
}

pub fn get_multiple_products(product_ids: Vec<i32>, conn: &mut MysqlConnection) -> Result<Vec<ProductDetail>, DieselError> {
    use crate::schema::product_details::dsl::*;

    product_details
        .filter(id.eq_any(product_ids))
        .select(ProductDetail::as_select())
        .load(conn)
}

pub fn save(new_product_details: NewProductDetails, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::product_details;

    diesel::insert_into(product_details::table)
        .values(new_product_details)
        .execute(conn)
}

pub fn update(pd: ProductDetail, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::product_details;
    use crate::schema::product_details::dsl::*;

    diesel::update(product_details::table)
        .filter(id.eq(pd.id))
        .set(pd)
        .execute(conn)
}