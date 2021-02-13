use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::iter::FromIterator;

use rayon::prelude::*;

use crate::algebra::lib::{collect, make_matrix, make_vec};
use crate::algebra::page_rank::page_rank;
use crate::sql_extracts::categories::{Category, CategoryCategorySql};
use crate::sql_extracts::categories::category_category_vec::CategoryCategoryVec;
use crate::sql_extracts::extractor::Extractor;
use crate::sql_extracts::merge_categories_links_triplets;

mod sql_extracts;
pub mod algebra;

#[cfg(test)]
mod tests;


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

	let cells =
		merge_categories_links_triplets(&categories, &catcat);

	let vec = make_vec(categories.par_iter().map(|c| c.id));
	let matrix = make_matrix(
		cells.map(|c| c.to_tuple_calculate()).par_bridge(),
		vec.dim());

	let res = page_rank(&matrix, &vec, 0.2, 0.1);

	let tmp = collect(matrix, res);
	println!("{:?}", tmp);
}