#![feature(proc_macro_hygiene, decl_macro)]
#![feature(total_cmp)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::collections::HashMap;

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

#[get("/")]
fn index() -> &'static str {
    "Hello, the world!"
}

fn main() {
    rocket::ignite()
        .attach(Db::fairing())
        .manage(CategoryHash::generate(
            Connection::open("db.sqlite").unwrap(),
        ))
        .mount("/", routes![index])
        .mount(
            "/api",
            routes![
                api::simple_category_get,
                api::simple_page_get,
                api::categories_post
            ],
        )
        .launch();
}
