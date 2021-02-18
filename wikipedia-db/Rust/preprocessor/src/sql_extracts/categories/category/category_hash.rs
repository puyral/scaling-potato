use std::collections::HashMap;

use rayon::prelude::*;

use crate::sql_extracts::categories::category::AbstractCategory;

pub struct CategoryHash<C: AbstractCategory> {
	vec: Vec<C>,
	by_indices: HashMap<u32, usize>,
	by_title: HashMap<String, usize>,
}

impl<C> FromParallelIterator<C> for CategoryHash<C> where
	C: Sync + Send + AbstractCategory {
	fn from_par_iter<I>(par_iter: I) -> Self where
		I: IntoParallelIterator<Item = C> {
		let vec = Vec::from_par_iter(par_iter);
		let by_indices: HashMap<_, _>
			= vec.par_iter().enumerate().map(|(i, c)| { (c.get_id(), i) }).collect();
		let by_title: HashMap<_, _>
			= vec.par_iter().enumerate().map(|(i, c)| { (c.get_title().to_owned(), i) }).collect();

		CategoryHash { vec, by_indices, by_title }
	}
}

impl<C: AbstractCategory> CategoryHash<C> {
	pub fn get_by_index(&self, i: u32) -> Option<&C> {
		match self.by_indices.get(&i) {
			None => None,
			Some(&x) => Some(&self.vec[x])
		}
	}

	pub fn get_by_index_mut(&mut self, i: u32) -> Option<&mut C> {
		match self.by_indices.get(&i) {
			None => None,
			Some(&x) => Some(&mut self.vec[x])
		}
	}

	pub fn get_by_title(&self, s: &str) -> Option<&C> {
		match self.by_title.get(s) {
			None => None,
			Some(&x) => Some(&self.vec[x])
		}
	}

	// pub fn len(&self) -> usize { self.vec.len() }
}

impl<C: AbstractCategory> CategoryHash<C> {
	pub fn get_data(&self) -> &Vec<C> {
		&self.vec
	}
}