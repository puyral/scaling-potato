use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

use crate::sql_extracts::categories::Category;
use crate::sql_extracts::extractor::Extractor;

mod sql_extracts;
mod algebra;


pub fn extract(text: String) -> VecDeque<Category> {
	let extractor = Extractor::new::<Category>().unwrap();
	let mut vect: VecDeque<Category> = VecDeque::new();
	extractor.extract(&text, &mut vect);
	return vect;
}

pub fn extract_categories(text: String) -> HashMap<String, Category> {
	HashMap::from_iter(
		Extractor::new::<Category>().unwrap().extract_iter::<Category>(&text)
			.map(|cat| (cat.title.clone(), cat))
	)
}