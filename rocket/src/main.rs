#![feature(proc_macro_hygiene, decl_macro)]

mod sql_interface;
pub mod categories;
pub mod api;
mod result;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate try_opt;

use rocket_contrib::databases::rusqlite;
use rocket_contrib::databases::rusqlite::types::ToSql;
use rocket_contrib::databases::rusqlite::{Connection, MappedRows};
use std::collections::HashMap;

use crate::categories::category::category_hash::CategoryHash;

//ub const NO_PARAMS: &[&dyn ToSql] = &[];

#[database("db")]
pub struct Db(rusqlite::Connection);

pub type Categories = HashMap<String,CategoryHash>;

#[get("/")]
fn index() -> &'static str {
	"Hello, the world!"
}

fn main() {
	rocket::ignite()
		.attach(Db::fairing())
		.manage(CategoryHash::generate(Connection::open("db.sqlite").unwrap()))
		.mount("/", routes![index])
		.mount("/api", routes![api::simple_category_get, api::simple_page_get, api::categories_post])
		.launch();
}