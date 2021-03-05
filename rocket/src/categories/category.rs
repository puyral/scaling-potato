mod category_hash;

use crate::categories::Entry;
use std::ops::AddAssign;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
	id: u32,
	pr: f64,
}

impl Category {
	fn get_pr(&self) -> f64 {
		self.pr
	}

	fn copy(&self) -> Category {
		Category { id: self.id, pr: self.pr }
	}
}

#[derive(Debug)]
pub struct ScoredCategory {
	category: &'static Category,
	score: f64,
}

impl<'a> ScoredCategory {
	fn get_category(&self) -> &'a Category {
		self.category
	}

	fn get_score(&self) -> f64 {
		self.score
	}

	fn set_score(&mut self, score: f64) {
		self.score = score;
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoredCategoryNoRef {
	category: Category,
	score: f64,
}

impl ScoredCategoryNoRef {
	fn new(sc: ScoredCategory) -> ScoredCategoryNoRef {
		ScoredCategoryNoRef { category: sc.category.copy(), score: sc.score }
	}
}

impl Entry for Category {
	fn get_id(&self) -> u32 {
		self.id
	}
}

impl Entry for ScoredCategory {
	fn get_id(&self) -> u32 {
		self.category.get_id()
	}
}

impl AddAssign<f64> for ScoredCategory {
	fn add_assign(&mut self, rhs: f64) {
		self.score += rhs;
	}
}