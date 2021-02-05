use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::iter::FromIterator;

use rayon::prelude::FromParallelIterator;

use crate::sql_extracts::categories::{Category, CategoryCategorySql};
use crate::sql_extracts::categories::category_category_vec::CategoryCategoryVec;
use crate::sql_extracts::extractor::Extractor;
use crate::sql_extracts::merge_categories_links_triplets;

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


pub fn run(categoriesf: File, catcatsf: File) {
	let categories = Vec::from_par_iter(
		Extractor::extract_par_iter_file::<Category>(categoriesf));
	let catcat = CategoryCategoryVec::from_par_iter(
		Extractor::extract_par_iter_file::<CategoryCategorySql>(catcatsf));

	let _cells =
		merge_categories_links_triplets(&categories, &catcat);
}