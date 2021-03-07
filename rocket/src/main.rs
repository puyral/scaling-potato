#![feature(proc_macro_hygiene, decl_macro)]
#![feature(total_cmp)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::collections::HashMap;

use rocket::http::Method;
use rocket::response::NamedFile;
use rocket_contrib::databases::rusqlite;
use rocket_contrib::databases::rusqlite::Connection;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use crate::categories::category::category_hash::CategoryHash;

pub mod api;
pub mod categories;
mod result;
mod sql_interface;

#[database("db")]
pub struct Db(rusqlite::Connection);

pub type Categories = HashMap<String, CategoryHash>;

const DB_LOCATION: &str = "../wikipedia-db/db.sqlite";

#[get("/")]
fn index() -> &'static str {
    "Hello, the world!"
}

#[get("/dump")]
fn dump_db()-> Option<NamedFile>{
    NamedFile::open(DB_LOCATION).ok()
}


fn main() {
    let allowed_origins = AllowedOrigins::All;

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get,Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors().unwrap();

    rocket::ignite()
        .attach(Db::fairing())
        .manage(CategoryHash::generate(
            Connection::open(DB_LOCATION).unwrap(),
        ))
        .mount("/", routes![index, dump_db])
        .mount(
            "/api",
            routes![
                api::simple_category_get,
                //api::simple_page_get,
                api::categories_post,
                api::simple_category_get_title
            ],
        )

        // CORS
        .attach(cors)

        .launch();
}
