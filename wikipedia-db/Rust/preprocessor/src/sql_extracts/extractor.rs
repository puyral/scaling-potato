use regex::{Regex, Captures};
use std::collections::VecDeque;

/// A tool to
pub struct Extractor {
    rg: Regex
}

impl Extractor {
    /// Do the actual extraction
    /// - `sql` is the text to extract from
    /// - `queue` is where to put the result, this is done so to be more practical for multithreading
    pub fn extract<T: SqlExtractable>(&self, sql: &str, queue: &mut VecDeque<T>) -> () {
        for cap in self.rg.captures_iter(sql) {
            queue.push_back(T::from(cap));
        }
    }

    /// Make a new [Extractor]. Make sure to tell what `T` when using this
    pub fn new<T: SqlExtractable>() -> Result<Self, regex::Error> {
        let rg = Regex::new(&format!("\\({}\\)", T::PATTERN))?;
        return Ok(Self {
            rg
        });
    }
}

pub trait SqlExtractable {
    /// The regexp to match the sql INSERT querry.
    /// # Example
    /// for the categories this is
    /// ```rust
    /// r"(?P<id>\d+),'(?P<title>.*?)',\d+,\d+,\d+";
    /// ```
    const PATTERN: &'static str;

    /// Make `self` from the [Captures] object extracted thanks to [PATTERN].
    fn from(cap: Captures) -> Self;
}