use std::fs;

use preprocessor::extract;

fn main() {
	let contents = fs::read_to_string("../one_line_categories")
		.expect("Something went wrong reading the file");

	//println!("{}", contents);
	let vect = extract(&contents);

	for v in vect {
		println!("{:?}", v)
	}
}
