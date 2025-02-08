use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::ConnectionError;

use crate::constants::DATABASE_URL;

pub fn establish_connection() -> Result<MysqlConnection, ConnectionError> {
    MysqlConnection::establish(DATABASE_URL)
}
