pub mod features;
pub mod routes;
pub mod app_state;
pub mod schema;

use diesel::{Connection, PgConnection};
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}