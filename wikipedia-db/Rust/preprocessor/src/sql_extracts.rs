//! The sql extraction

pub mod extractor;
pub mod category;



#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use crate::sql_extracts::category::Category;
    use crate::sql_extracts::extractor::Extractor;
    use std::fs;

    #[test]
    fn extract() -> () {
        let text = fs::read_to_string("../linesample")
            .expect("Something went wrong reading the file");
        let mut vect: VecDeque<Category> = VecDeque::new();
        Extractor::new::<Category>()
            .expect("Something went wrong building the regexp")
            .extract(&text, &mut vect);
        assert_eq!(vect.len(), 28809);
    }
}