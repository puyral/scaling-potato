use crate::categories::category::Category;
use std::collections::{HashMap, HashSet};
use crate::categories::link::Link;
use crate::categories::Entry;
use tailcall::tailcall;

pub struct CategoryHash {
	vec: Vec<Category>,
	links: Vec<Vec<usize>>,
	indices: HashMap<u32, usize>,
	avg_degree: usize,
}

impl CategoryHash {
	fn new(categories: impl Iterator<Item = Category>, links: impl Iterator<Item = Link>) -> CategoryHash {
		let vec: Vec<_> = categories.collect();
		let mut vlinks = vec![Vec::new(); vec.len()];
		let mut indices = HashMap::new();
		let mut avr_degree = 0;

		vec.iter().enumerate().for_each(|(i, c)| {
			indices.insert(c.get_id(), i);
		});

		links.for_each(|l| {
			match (indices.get(&l.from), indices.get(&l.to)) {
				(Some(&i), Some(&j)) => {
					avr_degree += 1;
					vlinks.get_mut(i).expect("out of bound").push(j)
				}
				_ => eprintln!("no category with id : {} or {}", l.from, l.to),
			}
		});

		CategoryHash {
			vec: vec,
			links: vlinks,
			indices: indices,
			avg_degree: avr_degree / vec.len(),
		}
	}

	pub fn get_category(&self, id: u32) -> Option<&Category> {
		match self.indices.get(&id) {
			None => None,
			Some(&i) => Some(&self.vec[i])
		}
	}

	pub fn get_categories<'a>(&'a self, category: &Category) -> Option<impl Iterator<Item = &Category>+'a> {
		match self.indices.get(&category.id) {
			None => None,
			Some(&i) => Some(self.links[i].iter().map(|ci| self.vec.get(*ci).unwrap()))
		}
	}

	#[tailcall]
	fn graph_search_inter(links: &Vec<Vec<usize>>, avg_degree: usize, todo: HashSet<usize>, mut done: HashSet<usize>) -> HashSet<usize> {
		match todo.is_empty() {
			true => done,
			false => {
				let mut todo_next: HashSet<usize> =
					HashSet::with_capacity(todo.len() * avg_degree);

				for c in todo {
					todo_next.extend(links[c].to_owned());
					done.insert(c);
				}

				graph_search_inter(links, avg_degree, todo_next, done)
			}
		}
	}

	fn get_categories_from_positions(&self, categories: impl Iterator<Item = usize>) -> impl Iterator<Item = Option<&Category>> {
		categories.map(move |c| self.vec.get(c))
	}
}