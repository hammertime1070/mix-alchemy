use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::prelude::*;
use rocket_db_pools::diesel;

#[derive(Database)]
#[database("concrete_db")]
pub struct MixAlchemy(diesel::MysqlPool);