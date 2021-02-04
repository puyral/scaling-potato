pub mod sql_extracts {
    use regex::Captures;
    use std::collections::VecDeque;
    use crate::sql_extracts::extractor::Extractor;

    pub trait SqlExtractable {
        const PATTERN: &'static str;

        fn from(cap: Captures) -> Self;
    }

    mod extractor {
        use regex::Regex;
        use crate::sql_extracts::SqlExtractable;
        use std::collections::VecDeque;

        pub(crate) struct Extractor {
            rg: Regex
        }

        impl Extractor{
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
    }

    #[derive(Debug)]
    struct Category {
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

    pub fn extract(text: &str) -> () {
        let extractor: Extractor = extractor::Extractor::new::<Category>().unwrap();
        let mut vect: VecDeque<Category> = VecDeque::new();
        extractor.extract(text, &mut vect);
        println!("{:?}", vect);
    }
}