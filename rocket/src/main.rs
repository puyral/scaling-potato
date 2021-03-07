#![feature(proc_macro_hygiene, decl_macro)]
#![feature(total_cmp)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::collections::HashMap;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::NamedFile;
use rocket_contrib::databases::rusqlite;
use rocket_contrib::databases::rusqlite::Connection;

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

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn main() {
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
        .attach(CORS())

        .launch();
}
