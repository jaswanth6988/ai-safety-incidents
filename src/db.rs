use rocket_sync_db_pools::database;

#[database("sqlite_incidents")]
pub struct DbConn(diesel::SqliteConnection);
