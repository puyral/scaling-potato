//! The sql extraction

use std::collections::HashMap;
use std::fs::File;

use rayon::prelude::{FromParallelIterator, ParallelIterator};

use crate::sql_extracts::categories::{Category, CategoryCategory, CategoryCategorySql};
use crate::sql_extracts::extractor::Extractor;

pub mod extractor;
pub mod categories;

pub fn merge_categories_links<'a>(
	categories: HashMap<String, Category>,
	catcats: impl ParallelIterator<Item = CategoryCategorySql> + 'a,
) -> impl ParallelIterator<Item = CategoryCategory> + 'a
{
	catcats.map(move |catcat| {
		CategoryCategory::from_hash(&catcat, &categories)
			.expect("no corresponding category")
	});


	categories
}

pub fn make_categories_links(
	categories: File, catcats: File,
) -> impl ParallelIterator<Item = CategoryCategory>
{
	let categories_hashmap =
		HashMap::<String, Category>::from_par_iter(
			Extractor::extract_par_iter_file_static::<Category>(categories)
				.map(|cat| (cat.title.clone(), cat))
		);
	let catcats_iter =
		Extractor::extract_par_iter_file_static::<CategoryCategorySql>(catcats);

	merge_categories_links(
		categories_hashmap,
		catcats_iter,
	)
}

#[cfg(test)]
mod tests {
	//TODO create test and test file to test mixing both the categories and the links

	use std::fs::File;

	use rayon::prelude::FromParallelIterator;

	use crate::sql_extracts::categories::CategoryCategory;
	use crate::sql_extracts::make_categories_links;

	#[test]
	fn parallel() -> () {
		let v = Vec::from_par_iter(make_categories_links(
			File::open("test_samples/nrm/nrmwiki-20210201-category.sql")
				.expect("Something went wrong reading the file category"),
			File::open("test_samples/nrm/nrmwiki-20210201-categorylinks.sql")
				.expect("Something went wrong reading the file categorylinks"),
		));

		assert_eq!(CategoryCategory { from: 9310, to: 5629 }, v[89]);
		assert_eq!(CategoryCategory { from: 2507, to: 10 }, v[7]);
		assert_eq!(CategoryCategory { from: 10251, to: 1938 }, v[120]);
		assert_eq!(147, v.len());
	}
}