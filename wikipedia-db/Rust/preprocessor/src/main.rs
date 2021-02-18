use std::fs::File;
use std::io;
use std::io::Write;

use clap::{App, Arg};

use preprocessor::{make_sql, run};

fn main() {
	let matches = App::new("Scalling-Potatoes' category preprocessor")
		.version("0.2.0")
		.author("simony2222 <7871851+simony2222@users.noreply.github.com>\nHakido")
		.about("Preprocess the wikipedia categories")
		.arg(
			Arg::with_name("categories")
				.short("c")
				.long("categories")
				.takes_value(true)
				.help(
					"The file where to find the categories.\
			It is usually called **wiki-YYYYMMDD-page.sql",
				)
				.required(true)
				.value_name("FILE"),
		)
		.arg(
			Arg::with_name("category links")
				.short("C")
				.long("category-links")
				.takes_value(true)
				.help(
					"The file where to find the links between categories.\
			It is usually called **wiki-YYYYMMDD-categorylinks.sql",
				)
				.required(true)
				.value_name("FILE"),
		)
		.arg(
			Arg::with_name("output file")
				.short("o")
				.long("out")
				.takes_value(true)
				.help("The *.sql to put the result")
				.required(true)
				.value_name("FILE"),
		)
		.arg(
			Arg::with_name("beta")
				.short("b")
				.long("beta")
				.takes_value(true)
				.help("beta for the pagerank")
				.value_name("FLOAT"),
		)
		.arg(
			Arg::with_name("epsilon")
				.short("e")
				.long("epsilon")
				.takes_value(true)
				.help("epsilon for the pagerank")
				.value_name("FLOAT"),
		)
		.get_matches();

	// process args
	let categories_path = matches.value_of("categories").expect("required argument");
	let category_links_path = matches
		.value_of("category links")
		.expect("required argument");
	let out_path = matches.value_of("output file").expect("required argument");
	let beta = matches
		.value_of("beta")
		.unwrap_or("0.2")
		.parse::<f64>()
		.expect("beta should be a float");
	let epsilon = matches
		.value_of("epsilon")
		.unwrap_or("1e-10")
		.parse::<f64>()
		.expect("beta should be a float");

	println!("[START]");

	//open files
	print!("Opening files...");
	io::stdout().flush().ok().expect("Could not flush stdout");
	let text_cat = File::open(categories_path).expect(&*format!(
		"Something went wrong reading the categories file : {}",
		categories_path
	));
	let text_links = File::open(category_links_path).expect(&*format!(
		"Something went wrong reading the category-links file : {}",
		category_links_path
	));
	let mut output = File::create(out_path).expect(&*format!("can't create file {}", out_path));
	println!("[DONE]");

	// do
	let (categories, category_links) = run(text_cat, text_links, beta, epsilon);
	let sql_output = make_sql(&categories.get_data(), &category_links, "eo");

	output
		.write_all(sql_output.as_bytes())
		.expect("unable to write");
	println!(
		"[FINISHED] ({} categories and {} links)",
		categories.get_data().len(),
		category_links.len()
	)
}
