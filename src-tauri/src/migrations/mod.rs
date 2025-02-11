use std::error::Error;
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };

use crate::connection::establish_connection;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/mysql");

pub fn run_migrations() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // TODO get database name from app state
    // check if exists and if not exists create one
    let mut connection: diesel::mysql::MysqlConnection = establish_connection()?;
    let migration_res = connection.run_pending_migrations(MIGRATIONS)?;

    migration_res.iter().for_each(|val| log::debug!("{}", val));
    
    Ok(())
}