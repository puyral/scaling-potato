#![feature(proc_macro_hygiene, decl_macro)]

use crate::categories::category::category_hash::CategoryHash;
use crate::categories::category::{Category, WeightedCategory};
use crate::{Categories, Db};
use rocket::State;
use rocket_contrib::json::Json;
use std::iter::Once;
use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeightedCategoriesListInput{
	categories: Vec<u32>,
	weight:f64
}

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
			.map(|&c| WeightedCategory::new(c, weight))
			.collect::<crate::result::Result>()
			.to_json()
	)
}

#[post("/<wp>/category", data = "<data>")]
pub fn categories_post(
	categories: State<Categories>,
	wp: String,
	data: Json<Vec<WeightedCategoriesListInput>>
) -> Option<Json<crate::result::ResultJSON>> {
	let category_hash = categories.inner().get(&wp)?;

	let mut r = crate::result::Result::empty();

	for wcategories in data.into_inner() {
		let top =
			category_hash.build_top_categories_intern(wcategories.categories.iter().flat_map(|&id|category_hash.get(id)));
		let r_tmp =
			crate::result::Result::from_categories_and_weight(&top, wcategories.weight);

		r+=r_tmp
	}
	Some(r.to_json())
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
			.map(|&c| WeightedCategory::new(c, weight))
			.collect::<crate::result::Result>()
			.to_json()
	)
}
