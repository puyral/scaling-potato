use std::collections::VecDeque;
use crate::sql_extracts::extractor::Extractor;
use crate::sql_extracts::category::Category;

mod sql_extracts;


pub fn extract(text: &str) -> () {
    let extractor = Extractor::new::<Category>().unwrap();
    let mut vect: VecDeque<Category> = VecDeque::new();
    extractor.extract(text, &mut vect);
    println!("{:?}", vect.len());
}