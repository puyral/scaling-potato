use std::fs::File;

use crate::run;

#[test]
fn norman() {
	let text_cat = File::open("test_samples/nrm/nrmwiki-20210201-category.sql")
		.expect("Something went wrong reading the file one_line_categories");
	let text_links = File::open("test_samples/nrm/nrmwiki-20210201-categorylinks.sql")
		.expect("Something went wrong reading the file one_line_categorieslinks");

	run(text_cat, text_links)
}