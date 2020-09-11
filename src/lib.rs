pub mod server;
pub mod storage;
pub mod hashchain;
pub mod types;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use std::time::{SystemTime};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_timestamp() -> i32{
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32
}