use std::fs;
use preprocessor::sql_extracts::extract;


fn main() {
    let contents = fs::read_to_string("../linesample")
        .expect("Something went wrong reading the file");

    //println!("{}", contents);
    extract(&contents);
}
