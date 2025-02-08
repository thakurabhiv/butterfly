use crate::models::StateList;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub fn search(query: String, conn: &mut MysqlConnection) -> Result<Vec<StateList>, DieselError> {
    use crate::schema::state_list::dsl::*;

    state_list
        .filter(state_name.like(format!("%{}%", query)))
        .select(StateList::as_select())
        .load(conn)
}

pub fn get_state(state_id: i32, conn: &mut MysqlConnection) -> Result<StateList, DieselError> {
    use crate::schema::state_list::dsl::*;

    state_list
        .filter(id.eq(state_id))
        .select(StateList::as_select())
        .first(conn)
}
