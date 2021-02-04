use crate::sql_extracts::extractor::SqlExtractable;
use regex::Captures;

#[derive(Debug)]
pub struct Category {
    cat_id: u32,
    cat_title: String,
}

impl SqlExtractable for Category {
    const PATTERN: &'static str = r"(?P<id>\d+),'(?P<title>.*?)',\d+,\d+,\d+";

    fn from(cap: Captures) -> Self {
        return Category {
            cat_id: cap["id"].parse::<u32>().unwrap(),
            cat_title: String::from(&cap["title"]),
        };
    }
}