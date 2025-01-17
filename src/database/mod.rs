pub mod insert_table;
mod conn;

use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use diesel::PgConnection;
use std::env;

pub(crate) type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_pool() -> DbPool {
    dotenv().ok(); 
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool")
}