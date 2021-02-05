//! The sql extraction



use crate::algebra::NonZeroCoeff;
use crate::sql_extracts::categories::Category;
use crate::sql_extracts::categories::category_category_vec::CategoryCategoryVec;

pub mod extractor;
pub mod categories;

pub fn merge_categories_links_triplets<'a>(
	categories: &'a Vec<Category>,
	catcats: &'a CategoryCategoryVec, //sorted array
) -> impl Iterator<Item = NonZeroCoeff> + 'a
{
	categories.iter().flat_map(move |cat| {
		let v = catcats.search(&cat.title);
		let n = v.len();

		v.into_iter().map(move |catcat| {
			NonZeroCoeff::new(catcat.from, cat.id, n as u32)
		})
	})
}

#[cfg(test)]
mod tests {
	use std::fs::File;
	use std::iter::FromIterator;

	use rayon::prelude::FromParallelIterator;

	use crate::algebra::NonZeroCoeff;
	use crate::sql_extracts::categories::{Category, CategoryCategorySql};
	use crate::sql_extracts::categories::category_category_vec::CategoryCategoryVec;
	use crate::sql_extracts::extractor::Extractor;
	use crate::sql_extracts::merge_categories_links_triplets;

	#[test]
	fn parallel() -> () {
		let categories = Vec::from_par_iter(
			Extractor::extract_par_iter_file::<Category>(
				File::open("test_samples/nrm/nrmwiki-20210201-category.sql")
					.expect("Something went wrong reading the file category")));
		let catcat = CategoryCategoryVec::from_par_iter(
			Extractor::extract_par_iter_file::<CategoryCategorySql>(
				File::open("test_samples/nrm/nrmwiki-20210201-categorylinks.sql")
					.expect("Something went wrong reading the file categorylinks")));

		let v =
			Vec::from_iter(merge_categories_links_triplets(&categories, &catcat));

		// println!("assert_eq!({},v.len());",v.len());
		// v[5..13].iter().for_each(|nzc|println!("{},",nzc.serialize()))

		assert_eq!(146, v.len());
		assert_eq!(v[5], NonZeroCoeff::new(3133, 14, 6));
		assert_eq!(v[6], NonZeroCoeff::new(5594, 14, 6));
		assert_eq!(v[7], NonZeroCoeff::new(5963, 14, 6));
		assert_eq!(v[8], NonZeroCoeff::new(10160, 14, 6));
		assert_eq!(v[9], NonZeroCoeff::new(10252, 14, 6));
		assert_eq!(v[10], NonZeroCoeff::new(10340, 14, 6));
		assert_eq!(v[11], NonZeroCoeff::new(1902, 15, 3));
		assert_eq!(v[12], NonZeroCoeff::new(2456, 15, 3));
		assert_eq!(v[13], NonZeroCoeff::new(5797, 15, 3));
		assert_eq!(v[14], NonZeroCoeff::new(6893, 22, 1));

		// for i in 5..15 {
		// 	println!("assert_eq!(v[{}],{});", i, v[i].serialize())
		// }
	}
}