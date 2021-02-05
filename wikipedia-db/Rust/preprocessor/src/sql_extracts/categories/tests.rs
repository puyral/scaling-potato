//! Tests

use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

use crate::sql_extracts::categories::{Category, CategoryCategorySql};
use crate::sql_extracts::extractor::Extractor;

#[test]
fn extract_categories_iter() -> () {
	let text = fs::read_to_string("test_samples/nrm/nrmwiki-20210201-category.sql")
		.expect("Something went wrong reading the file");
	let v = Vec::from_iter(
		Extractor::new::<Category>().expect("Something went wrong building the regexp")
			.extract_iter::<Category>(&text)
	);
	assert_eq!(Category { id: 460, title: "Mort_en_1499".parse().unwrap() }, v[456]);
	assert_eq!(Category { id: 8, title: "Arqùéologie".parse().unwrap() }, v[7]);
	assert_eq!(Category { id: 2131, title: "Féchouneus_eur".parse().unwrap() }, v[2000]);
	assert_eq!(2308, v.len());
}

#[test]
fn extract_links() -> () {
	let text_cat = fs::read_to_string("test_samples/nrm/nrmwiki-20210201-category.sql")
		.expect("Something went wrong reading the file one_line_categories");
	let text_links = fs::read_to_string("test_samples/nrm/nrmwiki-20210201-categorylinks.sql")
		.expect("Something went wrong reading the file one_line_categorieslinks");

	let mut _cats = HashMap::<String, Category>::from_iter(
		Extractor::new::<Category>().unwrap().extract_iter::<Category>(&text_cat)
			.map(|cat| (cat.title.clone(), cat))
	);

	let v = Vec::from_iter(
		Extractor::new::<CategoryCategorySql>().unwrap()
			.extract_iter::<CategoryCategorySql>(&text_links)
	);

	// println!("assert_eq!({:?},v[89]);\nassert_eq!({:?},v[7]);\nassert_eq!({:?},v[120]);",v[89],v[7],v[120]);

	assert_eq!(CategoryCategorySql { from: 10251, to: "Ûrope".parse().unwrap() }, v[120]);
	assert_eq!(CategoryCategorySql { from: 2507, to: "Astrononmie".parse().unwrap() }, v[7]);
	assert_eq!(CategoryCategorySql { from: 6187, to: "Animâ".parse().unwrap() }, v[45]);
	assert_eq!(147, v.len());
}