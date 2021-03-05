#![feature(proc_macro_hygiene, decl_macro)]

mod sql_interface;
mod categories;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, the world!"
}

fn main() {


    rocket::ignite().mount("/", routes![index]).launch();
}