//! The sql extraction


use std::collections::HashMap;
use std::fs::File;

use rayon::prelude::*;

use crate::sql_extracts::categories::{Category, CategoryCategory, CategoryCategorySql};
use crate::sql_extracts::categories::category_category_vec::CategoryCategoryVec;
use crate::sql_extracts::extractor::Extractor;

pub mod extractor;
pub mod categories;

pub fn merge_categories_links_triplets<'a>(
	categories: &'a impl ParallelIterator<Item = Category>,
	catcats: CategoryCategoryVec, //sorted array
) -> impl ParallelIterator<Item = CategoryCategory> + 'a
{
	categories.flat_map_iter(|cat| {})
}

pub fn make_categories_links(
	categoriesf: File, catcatsf: File,
) -> impl ParallelIterator<Item = CategoryCategory>
{
	merge_categories_links(
		Extractor::extract_par_iter_file(categoriesf),
		CategoryCategoryVec::from_par_iter(Extractor::extract_par_iter_file(catcatsf)),
	)
}

#[cfg(test)]
mod tests {
	//TODO create test and test file to test mixing both the categories and the links

	// #[test]
	// fn parallel() -> () {
	// 	let v = Vec::from_par_iter(make_categories_links(
	// 		File::open("test_samples/nrm/nrmwiki-20210201-category.sql")
	// 			.expect("Something went wrong reading the file category"),
	// 		File::open("test_samples/nrm/nrmwiki-20210201-categorylinks.sql")
	// 			.expect("Something went wrong reading the file categorylinks"),
	// 	));
	//
	// 	assert_eq!(CategoryCategory { from: 9310, to: 5629 }, v[89]);
	// 	assert_eq!(CategoryCategory { from: 2507, to: 10 }, v[7]);
	// 	assert_eq!(CategoryCategory { from: 10251, to: 1938 }, v[120]);
	// 	assert_eq!(147, v.len());
	// }
}