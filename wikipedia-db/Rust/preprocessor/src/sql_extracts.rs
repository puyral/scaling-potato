//! The sql extraction

use rayon::prelude::*;

use crate::algebra::NonZeroCoeff;
use crate::sql_extracts::categories::category::Category;
use crate::sql_extracts::categories::category::category_hash::CategoryHash;
use crate::sql_extracts::categories::category_links::CategoryLinks;

pub mod extractor;
pub mod categories;

pub fn calculate_nzc<'a>(
	categories: &'a CategoryHash<Category>,
	category_links: &'a Vec<CategoryLinks>,
) -> impl ParallelIterator<Item = NonZeroCoeff<u32, f64>> + 'a {
	category_links.into_par_iter().map(move |c| {
		NonZeroCoeff::new(c.from, c.to, categories.get_by_index(c.from).unwrap().get_dout() as f64)
	})
}

pub fn calculate_degrees<'a>(
	categories: &mut CategoryHash<Category>,
	category_links: impl Iterator<Item = &'a CategoryLinks>,
) {
	for edge in category_links {
		categories.get_by_index_mut(edge.from).unwrap().incr_dout()
	}
}


#[cfg(test)]
mod tests;