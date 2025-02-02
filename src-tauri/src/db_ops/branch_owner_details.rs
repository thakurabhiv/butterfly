use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::models::{ BranchOwnerDetail, NewBranchOwnerDetail };

pub fn find_all(conn: &mut MysqlConnection) -> Result<Vec<BranchOwnerDetail>, DieselError> {
    use crate::schema::branch_owner_details::dsl::*;

    branch_owner_details
        .filter(is_deleted.eq(false))
        .select(BranchOwnerDetail::as_select())
        .load(conn)
}

pub fn save(new_bwd: NewBranchOwnerDetail, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::branch_owner_details;

    diesel::insert_into(branch_owner_details::table)
        .values(new_bwd)
        .execute(conn)
}

pub fn update(bod: BranchOwnerDetail, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::branch_owner_details;
    use crate::schema::branch_owner_details::dsl::*;

    diesel::update(branch_owner_details::table)
        .filter(id.eq(bod.id))
        .set(bod)
        .execute(conn)
}