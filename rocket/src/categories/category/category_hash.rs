use std::collections::{HashMap, HashSet};

use rocket_contrib::databases::rusqlite::Connection;
use tailcall::tailcall;

use crate::categories::category::Category;
use crate::categories::Entry;
use crate::categories::link::Link;

pub struct CategoryHash {
    vec: Vec<Category>,
    links: Vec<Vec<usize>>,
    indices: HashMap<u32, usize>,
    avg_degree: usize,
}

impl CategoryHash {
    fn new(
        categories: impl Iterator<Item = Category>,
        links: impl Iterator<Item = Link>,
    ) -> CategoryHash {
        let vec: Vec<_> = categories.collect();
        let mut vlinks = vec![Vec::new(); vec.len()];
        let mut indices = HashMap::new();
        let mut avr_degree = 0;

        vec.iter().enumerate().for_each(|(i, c)| {
            indices.insert(c.get_id(), i);
        });

        links.for_each(|l| match (indices.get(&l.from), indices.get(&l.to)) {
            (Some(&i), Some(&j)) => {
                avr_degree += 1;
                vlinks.get_mut(i).expect("out of bound").push(j)
            }
            _ => eprintln!("no category with id : {} or {}", l.from, l.to),
        });
        avr_degree /= vec.len();
        CategoryHash {
            vec: vec,
            links: vlinks,
            indices: indices,
            avg_degree: avr_degree,
        }
    }

    /// Generate a hashmap of the hashcategories according to the wikipedia they belong to
    pub fn generate(conn: Connection) -> HashMap<String, CategoryHash> {
        conn.prepare(r"select * from `Wikipedias`;")
            .expect("error while preparing the query")
            .query_map(&[], |row| row.get(0))
            .expect("error while retrieving the result")
            .map(|wp| wp.unwrap())
            .map(|wp: String| {
                let mut smt = conn
                    .prepare(&*format!("select `id`, `page_rank` from `{}-categories`;", &wp))
                    .expect("error while preparing the query");
                let categories = smt
                    .query_map(&[], |row| Category {
                        id: row.get(0),
                        page_rank: row.get(1),
                    })
                    .expect("error while retrieving the result")
                    .map(|c| c.expect("unable to create category"));

                let mut smt = conn
                    .prepare(&*format!("select `from_id`,`to_id` from `{}-category-category`;", &wp))
                    .expect("error while preparing the query");
                let links = smt
                    .query_map(&[], |row| Link {
                        from: row.get(0),
                        to: row.get(0),
                    })
                    .expect("error while retrieving the result")
                    .map(|c| c.expect("unable to create link"));

                (wp, CategoryHash::new(categories, links))
            })
            .collect()
    }

    pub fn get(&self, id: u32) -> Option<&Category> {
        match self.indices.get(&id) {
            None => None,
            Some(&i) => Some(&self.vec[i]),
        }
    }

    /// Do the bfs
    #[tailcall]
    #[allow(unused_mut)]
    fn graph_search_inter(
        links: &Vec<Vec<usize>>,
        avg_degree: usize,
        todo: HashSet<usize>,
        mut done: HashSet<usize>,
    ) -> HashSet<usize> {
        match todo.is_empty() {
            true => done,
            false => {
                let mut todo_next: HashSet<usize> = HashSet::with_capacity(todo.len() * avg_degree);

                for c in todo {
                    for &i in &links[c] {
                        if !done.contains(&i) {
                            todo_next.insert(i);
                        }
                    }
                    done.insert(c);
                }

                CategoryHash::graph_search_inter(links, avg_degree, todo_next, done)
            }
        }
    }

    #[allow(dead_code)]
    fn get_categories_from_positions(
        &self,
        categories: impl Iterator<Item = usize>,
    ) -> impl Iterator<Item = Option<&Category>> {
        categories.map(move |c| self.vec.get(c))
    }

    pub fn build_top_categories_intern<'a>(
        &'a self,
        bottoms: impl Iterator<Item = &'a Category>,
    ) -> Vec<&Category> {
        CategoryHash::graph_search_inter(
            &self.links,
            self.avg_degree,
            bottoms
                .map(|c| self.indices.get(&c.id).unwrap().to_owned())
                .collect(),
            HashSet::new(),
        )
        .iter()
        .map(|&i| &self.vec[i])
        .collect()
    }
}

impl PartialEq for Category {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
