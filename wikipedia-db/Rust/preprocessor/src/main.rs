use std::fs::File;
use std::io::Write;

use preprocessor::{make_sql, run};
use clap::{App, Arg};

fn main() {
	let matches = App::new("Scalling-Potatoes' category preprocessor")
		.version("0.2.0")
		.author("simony2222 <7871851+simony2222@users.noreply.github.com>\nHakido")
		.about("Preprocess the wikipedia categories")
		.arg(Arg::with_name("categories")
			.short("c")
			.long("categories")
			.takes_value(true)
			.help("The file where to find the categories.\
			It is usually called **wiki-YYYYMMDD-page.sql")
			.required(true))
		.arg(Arg::with_name("category links")
			.short("C")
			.long("category-links")
			.takes_value(true)
			.help("The file where to find the links between categories.\
			It is usually called **wiki-YYYYMMDD-categorylinks.sql")
			.required(true))
		.arg(Arg::with_name("output file")
			.short("o")
			.long("out")
			.takes_value(true)
			.help("The *.sql to put the result")
			.required(true))
		.get_matches();

	let myfile = matches.value_of("file").unwrap_or("input.txt");
	println!("The file passed is: {}", myfile);

	let num_str = matches.value_of("num");
	match num_str {
		None => println!("No idea what your favorite number is."),
		Some(s) => {
			match s.parse::<i32>() {
				Ok(n) => println!("Your favorite number must be {}.", n + 5),
				Err(_) => println!("That's not a number! {}", s),
			}
		}
	}

	// process args
	let categories_path = matches.value_of("categories").expect("required argument");
	let category_links_path = matches.value_of("category links").expect("required argument");
	let out_path = matches.value_of("output file").expect("required argument");


	//open files
	let text_cat = File::open(categories_path)
		.expect(&*format!("Something went wrong reading the categories file : {}", categories_path));
	let text_links = File::open(category_links_path)
		.expect(&*format!("Something went wrong reading the category-links file : {}", category_links_path));
	let mut output = File::create(out_path)
		.expect(&*format!("can't create file {}", out_path));

	// do
	let (categories, category_links)
		= run(text_cat, text_links);
	let sql_output = make_sql(&categories.get_data(), &category_links, "eo");

	output.write_all(sql_output.as_bytes()).expect("unable to write");
}
