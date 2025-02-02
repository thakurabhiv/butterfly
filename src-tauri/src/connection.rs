use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::ConnectionError;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<MysqlConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
}
