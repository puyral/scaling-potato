//! The sql extraction

pub mod extractor;
pub mod categories;


#[cfg(test)]
mod tests {
	//TODO create test and test file to test mixing both the categories and the links

	// #[test]
	// fn extract_categories() -> () {
	// 	let text = fs::read_to_string("../linesample")
	// 		.expect("Something went wrong reading the file");
	// 	let mut vect: VecDeque<Category> = VecDeque::new();
	// 	Extractor::new::<Category>()
	// 		.expect("Something went wrong building the regexp")
	// 		.extract(&text, &mut vect);
	// 	assert_eq!(vect.len(), 28809);
	// }
}