use std::cmp::Ordering;
use std::iter::FromIterator;

use rayon::prelude::{FromParallelIterator, IntoParallelIterator};
use rayon::prelude::ParallelSliceMut;

use crate::sql_extracts::categories::CategoryCategorySql;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct CategoryCategoryVec {
	vec: Vec<CategoryCategorySql>
}

impl CategoryCategoryVec {
	pub fn new(mut vec: Vec<CategoryCategorySql>) -> CategoryCategoryVec {
		vec.par_sort_by(|a, b| a.to.cmp(&b.to));
		CategoryCategoryVec { vec }
	}

	pub fn search(&self, str: &str) -> &[CategoryCategorySql] {
		let (mut i, mut j) = (0, self.vec.len() - 1);

		if (self.vec[i].to > str.to_string()) || (self.vec[j].to < str.to_string()) {
			return &self.vec[0..0];
		}

		let mut c: usize;

		while (i < j) && (!self.vec[i].to.eq(&self.vec[j].to)) {
			c = (i + j) / 2;

			if (c == j) || (c == i) { return &self.vec[0..0]; }
			match str.cmp(&self.vec[c].to) {
				Ordering::Less => j = c,
				Ordering::Greater => i = c,
				Ordering::Equal => {
					i = c;
					j = c;
					while (i > 0) && (self.vec[i].to.eq(str)) { i -= 1 }
					i += 1;
					while (j < self.vec.len()) && (self.vec[j].to.eq(str)) {
						j += 1
					}
					break;
				}
			}
		}


		&self.vec[i..j]
	}
}

impl FromIterator<CategoryCategorySql> for CategoryCategoryVec {
	fn from_iter<T: IntoIterator<Item = CategoryCategorySql>>(iter: T) -> Self {
		CategoryCategoryVec::new(Vec::from_iter(iter))
	}
}

impl FromParallelIterator<CategoryCategorySql> for CategoryCategoryVec {
	fn from_par_iter<I>(par_iter: I) -> Self where
		I: IntoParallelIterator<Item = CategoryCategorySql> {
		CategoryCategoryVec::new(Vec::from_par_iter(par_iter))
	}
}

#[cfg(test)]
mod tests {
	use rayon::prelude::FromParallelIterator;

	use crate::sql_extracts::categories::category_category_vec::CategoryCategoryVec;
	use crate::sql_extracts::categories::CategoryCategorySql;

	#[test]
	fn build() {
		let t = vec![
			CategoryCategorySql { from: 25, to: "a".parse().unwrap() },
			CategoryCategorySql { from: 753, to: "z".parse().unwrap() },
			CategoryCategorySql { from: 410, to: "b".parse().unwrap() },
			CategoryCategorySql { from: 78, to: "a".parse().unwrap() },
			CategoryCategorySql { from: 0, to: "q".parse().unwrap() },
			CategoryCategorySql { from: 453, to: "z".parse().unwrap() },
			CategoryCategorySql { from: 45, to: "v".parse().unwrap() },
			CategoryCategorySql { from: 13, to: "q".parse().unwrap() },
			CategoryCategorySql { from: 75, to: "a".parse().unwrap() }];

		let v = CategoryCategoryVec::from_par_iter(t);

		assert_eq!(v,
				   CategoryCategoryVec {
					   vec: Vec::from([
						   CategoryCategorySql { from: 25, to: "a".parse().unwrap() },
						   CategoryCategorySql { from: 78, to: "a".parse().unwrap() },
						   CategoryCategorySql { from: 75, to: "a".parse().unwrap() },
						   CategoryCategorySql { from: 410, to: "b".parse().unwrap() },
						   CategoryCategorySql { from: 0, to: "q".parse().unwrap() },
						   CategoryCategorySql { from: 13, to: "q".parse().unwrap() },
						   CategoryCategorySql { from: 45, to: "v".parse().unwrap() },
						   CategoryCategorySql { from: 753, to: "z".parse().unwrap() },
						   CategoryCategorySql { from: 453, to: "z".parse().unwrap() }])
				   })
	}

	#[test]
	fn search() {
		let t = vec![
			CategoryCategorySql { from: 25, to: "a".parse().unwrap() },
			CategoryCategorySql { from: 753, to: "z".parse().unwrap() },
			CategoryCategorySql { from: 410, to: "b".parse().unwrap() },
			CategoryCategorySql { from: 78, to: "a".parse().unwrap() },
			CategoryCategorySql { from: 0, to: "q".parse().unwrap() },
			CategoryCategorySql { from: 453, to: "z".parse().unwrap() },
			CategoryCategorySql { from: 45, to: "v".parse().unwrap() },
			CategoryCategorySql { from: 13, to: "q".parse().unwrap() },
			CategoryCategorySql { from: 75, to: "a".parse().unwrap() }];

		let v = CategoryCategoryVec::from_par_iter(t);

		assert_eq!(v.search("q"), &[
			CategoryCategorySql { from: 0, to: "q".parse().unwrap() },
			CategoryCategorySql { from: 13, to: "q".parse().unwrap() }]);
		assert_eq!(v.search("greg"), &[]);
	}
}