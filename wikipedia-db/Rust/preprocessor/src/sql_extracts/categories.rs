use std::collections::HashMap;

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

/// An edge in the graph of the category
#[derive(Debug)]
#[derive(PartialEq)]
pub struct CategoryCategory {
	pub from: u32,
	pub to: u32,
}


/// Because Wikipedia decided to have a structure id -> name (*yay....*) we must use this
/// temporary type to extract the sql
#[derive(Debug)]
#[derive(PartialEq)]
pub struct CategoryCategorySql {
	from: u32,
	to: String,
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

impl CategoryCategory {
	/// Builds from a [CategoryCategorySql] and [Category]
	fn from(from: &CategoryCategorySql, to: &Category) -> CategoryCategory {
		CategoryCategory { from: from.from, to: to.id }
	}

	/// Same as [Self](from_add) but retrieves the `cat_to`from a [HashMap] to make up for wikipedia's stupid design....
	pub fn from_hash<'a>(cat_from: &'a CategoryCategorySql, categories: &'a HashMap<String, Category>)
						 -> Option<CategoryCategory> {
		match categories.get(&cat_from.to) {
			None => None,
			Some(cat_to) => Some(Self::from(cat_from, cat_to))
		}
	}
}

#[cfg(test)]
mod tests;