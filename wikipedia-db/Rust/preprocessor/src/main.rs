use std::fs::File;
use std::io::Write;

use preprocessor::{make_sql, run};

fn main() {
	let text_cat = File::open("../../eo/eowiki-20210201-page.sql")
		.expect("Something went wrong reading the file one_line_categories");
	let text_links = File::open("../../eo/eowiki-20210201-categorylinks.sql")
		.expect("Something went wrong reading the file one_line_categorieslinks");

	let (categories, category_links)
		= run(text_cat, text_links);

	let mut output = File::create("test_samples/eo.sql").unwrap();
	output.write_all(
		make_sql(&categories.get_data(), &category_links, "eo").as_bytes()).unwrap();
}
