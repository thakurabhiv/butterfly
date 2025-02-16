use std::error::Error;
use diesel::MysqlConnection;
use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/mysql");

pub fn run_migrations(mut connection: MysqlConnection) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let migration_res = connection.run_pending_migrations(MIGRATIONS)?;

    migration_res.iter().for_each(|val| log::debug!("{}", val));
    
    Ok(())
}