use regex::Captures;

use crate::sql_extracts::extractor::SqlExtractable;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Category {
	pub id: u32,
	pub title: String,
}

impl Default for Category {
	fn default() -> Self {
		return Category {
			id: 0,
			title: String::new(),
		};
	}
}

impl SqlExtractable for Category {
	const PATTERN: &'static str = r"(?P<id>\d+),'(?P<title>(?:[^']|(?:\\'))*)'(?:,\d+){3}";

	fn from(cap: Captures) -> Self {
		return Category {
			id: cap["id"].parse::<u32>().unwrap(),
			title: String::from(&cap["title"]),
			..Default::default()
		};
	}
}

//---- CATEGORY_CATEGORY -----


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

#[cfg(test)]
mod tests;


pub mod category_category_vec;