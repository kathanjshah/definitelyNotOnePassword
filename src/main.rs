#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/hello")]
fn hello() -> String {
    format!("Hello")
}

#[get("/world")]
fn world() -> String {
    format!("World!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, world])
}

