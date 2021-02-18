use regex::Captures;

use crate::sql_extracts::extractor::SqlExtractable;

pub mod category_hash;

pub trait AbstractCategory {
    fn get_id(&self) -> u32;
    fn get_title(&self) -> &str;
    fn get_title_move(self) -> String;
}

pub trait PageRanked {
    fn get_pr(&self) -> f64;
    fn set_pr(&mut self, pr: f64);
}

#[derive(Debug, PartialEq)]
pub struct Category {
    id: u32,
    title: String,
    pr: f64,
    dout: u32,
}

impl Default for Category {
    fn default() -> Self {
        return Category {
            id: 0,
            title: String::new(),
            pr: 0.0,
            dout: 0,
        };
    }
}

impl Category {
    pub fn new(id: u32, title: String) -> Self {
        Category {
            id,
            title,
            ..Default::default()
        }
    }
    pub fn get_dout(&self) -> u32 {
        self.dout
    }
    pub fn incr_dout(&mut self) {
        self.dout += 1
    }
}

impl PageRanked for Category {
    fn get_pr(&self) -> f64 {
        self.pr
    }
    fn set_pr(&mut self, pr: f64) {
        self.pr = pr
    }
}

impl AbstractCategory for Category {
    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_title_move(self) -> String {
        self.title
    }
}

impl SqlExtractable for Category {
    // const PATTERN: &'static str = r"(?P<id>\d+),'(?P<title>(?:[^']|(?:\\'))*)'(?:,\d+){3}";
    const PATTERN: &'static str = r"(?P<id>\d+),14,'(?P<title>(?:[^']|(?:\\'))*)','(?:[^']|(?:\\'))*'(?:,\d+){2},\d+.\d+(?:,(?:(?:NULL)|(?:'(?:[^']|(?:\\'))*'))){2},\d+,\d+(?:,(?:(?:NULL)|(?:'(?:[^']|(?:\\'))*'))){2}";
    fn from(cap: Captures) -> Self {
        return Category {
            id: cap["id"].parse::<u32>().unwrap(),
            title: String::from(&cap["title"]),
            ..Default::default()
        };
    }
}
