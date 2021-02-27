use std::str;

use regex::bytes::Captures;

use crate::sql_extracts::extractor::SqlExtractable;

/// Because Wikipedia decided to have a structure id -> name (*yay....*) we must use this
/// temporary type to extract the sql
#[derive(Debug, PartialEq)]
pub struct PageCategorySql {
	pub from: u32,
	pub to: String,
}

impl SqlExtractable for PageCategorySql {
	const PATTERN: &'static str =
		r"(?P<from>\d+),'(?P<to>(?:[^']|(?:\\'))*)'(?:,'(?:[^']|(?:\\'))*'){4},'page'"; //beautiful !!

	fn from(cap: Captures) -> Self {
		// println!("cap: {:?}, from:{}, to:{}", &cap, str::from_utf8(&cap["from"]).unwrap(), str::from_utf8(&cap["to"]).unwrap());
		PageCategorySql {
			from: str::from_utf8(&cap["from"])
				.expect("not a valid utf-8")
				.parse::<u32>()
				.expect("not a number"),
			to: String::from_utf8(Vec::from(&cap["to"])).expect("not a valid utf-8"),
		}
	}
}

#[derive(Clone, Copy)]
pub struct PageCategoryLinks {
	pub from: u32,
	pub to: u32,
}