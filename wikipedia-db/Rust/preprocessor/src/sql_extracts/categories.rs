use std::collections::HashMap;

use regex::Captures;

use crate::sql_extracts::extractor::SqlExtractable;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Category {
	pub id: u32,
	pub title: String,
	tot: f64,
}

impl Default for Category {
	fn default() -> Self {
		return Category {
			id: 0,
			title: String::new(),
			tot: 0.0,
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
	from: u32,
	to: u32,
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

	/// Same as [Self](from) but it adds to the edge counter
	fn from_add(cat_from: &CategoryCategorySql, cat_to: &mut Category) -> CategoryCategory {
		cat_to.tot += 1.0;
		Self::from(cat_from, cat_to)
	}

	/// Same as [Self](from_add) but retrieves the `cat_to`from a [HashMap] to make up for wikipedia's stupid design....
	pub fn from_hash_add(cat_from: &CategoryCategorySql, categories: &mut HashMap<String, Category>)
						 -> Option<CategoryCategory> {
		match categories.get_mut(&cat_from.to) {
			None => None,
			Some(cat_to) => Some(Self::from_add(cat_from, cat_to))
		}
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;
	use std::fs;
	use std::iter::FromIterator;

	use crate::sql_extracts::categories::{Category, CategoryCategorySql};
	use crate::sql_extracts::extractor::Extractor;

	#[test]
	fn extract_categories_iter() -> () {
		let text = fs::read_to_string("../linesample")
			.expect("Something went wrong reading the file");
		let v = Vec::from_iter(
			Extractor::new::<Category>().expect("Something went wrong building the regexp")
				.extract_iter::<Category>(&text)
		);
		assert_eq!(Category { id: 464, title: String::from("1699"), tot: 0.0 }, v[457]);
		assert_eq!(Category { id: 11, title: String::from("0°_E"), tot: 0.0 }, v[7]);
		assert_eq!(Category { id: 8914, title: String::from("Mortintoj_en_1531"), tot: 0.0 }, v[7522]);
		assert_eq!(v.len(), 28809);
	}

	#[test]
	fn extract_links() -> () {
		let text_cat = fs::read_to_string("../linesample")
			.expect("Something went wrong reading the file linesample");
		let text_links = fs::read_to_string("../linesample_links")
			.expect("Something went wrong reading the file linesample_links");

		let mut _cats = HashMap::<String, Category>::from_iter(
			Extractor::new::<Category>().unwrap().extract_iter::<Category>(&text_cat)
				.map(|cat| (cat.title.clone(), cat))
		);

		let v = Vec::from_iter(
			Extractor::new::<CategoryCategorySql>().unwrap()
				.extract_iter::<CategoryCategorySql>(&text_links)
			// .map(|catcat| {
			// 	CategoryCategory::from_hash_add(&catcat, &mut cats)
			// 		.expect("no corresponding category")
			// })
		);

		// println!("assert_eq!({:?},v[89]);\nassert_eq!({:?},v[7]);\nassert_eq!({:?},v[120]);",v[89],v[7],v[120]);
		assert_eq!(CategoryCategorySql { from: 23152, to: String::from("Astronomiaj_studfakoj") }, v[89]);
		assert_eq!(CategoryCategorySql { from: 21173, to: String::from("Monoteismaj_religioj") }, v[7]);
		assert_eq!(CategoryCategorySql { from: 23350, to: String::from("Spiriteco") }, v[120]);
		assert_eq!(v.len(), 220);
	}
}