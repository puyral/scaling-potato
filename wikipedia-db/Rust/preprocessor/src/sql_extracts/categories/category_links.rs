use rayon::iter::ParallelIterator;
use regex::Captures;

use crate::sql_extracts::categories::category::AbstractCategory;
use crate::sql_extracts::categories::category::category_hash::CategoryHash;
use crate::sql_extracts::extractor::SqlExtractable;

mod category_links_hash;

/// Because Wikipedia decided to have a structure id -> name (*yay....*) we must use this
	/// temporary type to extract the sql
#[derive(Debug)]
#[derive(PartialEq)]
pub struct CategoryCategorySql {
	pub from: u32,
	pub to: String,
}

impl SqlExtractable for CategoryCategorySql {
	const PATTERN: &'static str =
		r"(?P<from>\d+),'(?P<to>(?:[^']|(?:\\'))*)'(?:,'(?:[^']|(?:\\'))*'){4},'subcat'"; //beautiful !!

	fn from(cap: Captures) -> Self {
		CategoryCategorySql {
			from: cap["from"].parse::<u32>().unwrap(),
			to: String::from(&cap["to"]),
		}
	}
}

pub struct CategoryLinks {
	pub from: u32,
	pub to: u32,
}

pub fn to_category_links_vec<'a, C: AbstractCategory + Sync>(
	categories: &'a CategoryHash<C>,
	catcats: impl ParallelIterator<Item = CategoryCategorySql> + 'a)
	-> impl ParallelIterator<Item = CategoryLinks> + 'a {
	catcats.map(move |c| {
		CategoryLinks {
			from: c.from,
			to: categories.get_by_title(&c.to).unwrap().get_id(),
		}
	})
}