use regex::Captures;

use crate::sql_extracts::extractor::SqlExtractable;

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
		// println!("cap: {:?}, from:{}, to:{}", &cap, &cap["from"], &cap["to"]);
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