use diesel::prelude::*;
use diesel::result::Error as DieselError;
use crate::models::{ IdSequence, NewIdSequence };

pub fn find_first(id_seq: IdSequence, conn: &mut MysqlConnection) -> Result<IdSequence, DieselError> {
    use crate::schema::id_sequence::dsl::*;

    id_sequence
        .filter(seq_name.eq(id_seq.seq_name.clone()))
        .filter(prefix.eq(id_seq.prefix.clone()))
        .filter(sec_prefix.eq(id_seq.sec_prefix.clone()))
        .select(IdSequence::as_select())
        .first(conn)
}

// insert new sequence
pub fn save(new_id_seq: NewIdSequence, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::id_sequence;

    diesel::insert_into(id_sequence::table)
        .values(new_id_seq)
        .execute(conn)
}

// for updating sequence number
pub fn update_seq_number(id_seq: IdSequence, conn: &mut MysqlConnection) -> Result<usize, DieselError> {
    use crate::schema::id_sequence;
    use crate::schema::id_sequence::dsl::*;

    diesel::update(id_sequence::table)
        .filter(seq_name.eq(id_seq.seq_name.clone()))
        .filter(prefix.eq(id_seq.prefix.clone()))
        .filter(sec_prefix.eq(id_seq.sec_prefix.clone()))
        .set((
            seq_number.eq(id_seq.seq_number),
            modified_date.eq(id_seq.modified_date)
        ))
        .execute(conn)
}