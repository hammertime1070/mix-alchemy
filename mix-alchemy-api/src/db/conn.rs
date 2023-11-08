use rocket_sync_db_pools::database;
use rocket_sync_db_pools::mysql;

#[database("concrete_db")]
pub struct MySqlDatabase(mysql::MySqlPool);