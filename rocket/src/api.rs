#![feature(proc_macro_hygiene, decl_macro)]

use crate::categories::category::category_hash::CategoryHash;
use crate::categories::category::{Category, ScoredCategory};
use crate::{Categories, Db};
use rocket::State;
use rocket_contrib::json::Json;
use std::iter::Once;

#[get("/<wp>/category?<id>&<weight>")]
pub fn simple_category_get(
	categories: State<Categories>,
	wp: String,
	id: u32,
	weight: Option<f64>,
) -> Option<Json<crate::result::ResultJSON>> {
	let category_hash = categories.inner().get(&wp)?;
	let category = category_hash.get(id)?;

	let top = category_hash.build_top_categories_intern(std::iter::once(category));

	let weight = weight.unwrap_or(1.0);
	Some(
		top.iter()
			.map(|&c| ScoredCategory::new(c, weight))
			.collect::<crate::result::Result>()
			.to_json()
	)
}

#[get("/<wp>/page?<id>&<weight>")]
pub fn simple_page_get(
	categories: State<Categories>,
	conn: Db,
	wp: String,
	id: u32,
	weight: Option<f64>,
) -> Option<Json<crate::result::ResultJSON>> {
	let category_hash = categories.inner().get(&wp)?;
	let mut smt = conn
		.prepare(&*format!("select to_id from `{}-page-category` where from_id={};", &wp, id))
		.ok()?;
	let categories = smt
		.query_map(&[], |row| category_hash.get(row.get(0)))
		.ok()?
		.flat_map(|c| c.ok()?);

	let top = category_hash.build_top_categories_intern(categories);

	let weight = weight.unwrap_or(1.0);

	Some(
		top.iter()
			.map(|&c| ScoredCategory::new(c, weight))
			.collect::<crate::result::Result>()
			.to_json()
	)
}
