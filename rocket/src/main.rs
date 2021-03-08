#![feature(proc_macro_hygiene, decl_macro)]
#![feature(total_cmp)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::databases::rusqlite;
use rocket_contrib::databases::rusqlite::Connection;
use rocket_contrib::templates::Template;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use crate::categories::category::category_hash::CategoryHash;

pub mod api;
pub mod categories;
mod result;
mod sql_interface;

#[database("db")]
pub struct Db(rusqlite::Connection);
struct AssetsDir(String);
struct Domain(String);

pub type Categories = HashMap<String, CategoryHash>;

const DB_LOCATION: &str = "../wikipedia-db/db.sqlite";

#[get("/")]
fn index(domain: State<Domain>) -> Template {
    let mut context = HashMap::new();
    context.insert("assets", domain.0.to_owned()+"/assets");
    context.insert("domain", domain.0.to_owned());
    Template::render("index", &context)
}

#[get("/<asset..>")]
fn assets(asset: PathBuf, assets_dir: State<AssetsDir>) -> Option<NamedFile> {
    NamedFile::open(Path::new(&assets_dir.0).join(asset)).ok()
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

        // templates
        .attach(Template::custom(|engine|{
            engine.tera.autoescape_on(vec![]);
        }))

        // assets
        .mount("/assets", routes![assets])
        .attach(AdHoc::on_attach("Assets Config", |rocket| {
            let assets_dir = rocket.config()
                .get_str("assets_dir")
                .unwrap_or("assets/")
                .to_string();

            Ok(rocket.manage(AssetsDir(assets_dir)))
        }))

        // domain
        .attach(AdHoc::on_attach("Domain Config", |rocket| {
            let domain = rocket.config()
                .get_str("domain")
                .unwrap()
                .to_string();

            Ok(rocket.manage(Domain(domain)))
        }))

        // routes
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
