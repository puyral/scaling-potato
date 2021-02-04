use regex::{Regex, Captures};
use std::collections::VecDeque;

pub struct Extractor {
    rg: Regex
}

impl Extractor {
    pub fn extract<T: SqlExtractable>(&self, sql: &str, queue: &mut VecDeque<T>) -> () {
        for cap in self.rg.captures_iter(sql) {
            queue.push_back(T::from(cap));
        }
    }

    pub fn new<T: SqlExtractable>() -> Result<Self, regex::Error> {
        let rg = Regex::new(&format!("\\({}\\)", T::PATTERN))?;
        return Ok(Self {
            rg
        });
    }
}

pub trait SqlExtractable {
    const PATTERN: &'static str;

    fn from(cap: Captures) -> Self;
}