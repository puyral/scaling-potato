pub mod category_hash;

use crate::categories::Entry;
use std::ops::AddAssign;
use serde::{Deserialize, Serialize};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use crate::Categories;
use rocket::http::Status;


#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
	id: u32,
	page_rank: f64,
}

impl Category {
	fn get_pr(&self) -> f64 {
		self.page_rank
	}

	fn copy(&self) -> Category {
		Category { id: self.id, page_rank: self.page_rank }
	}
}

#[derive(Debug)]
pub struct ScoredCategory<'a> {
	category: &'a Category,
	score: f64,
}

impl<'a> ScoredCategory<'a> {
	pub fn get_category(&self) -> &'a Category {
		self.category
	}

	pub fn get_score(&self) -> f64 {
		self.score
	}

	pub fn set_score(&mut self, score: f64) {
		self.score = score;
	}

	pub fn new(category: &'a Category, score: f64) -> Self {
		ScoredCategory { category, score }
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScoredCategoryNoRef {
	category: Category,
	score: f64,
}

impl ScoredCategoryNoRef {
	pub fn new(sc: &ScoredCategory<'_>) -> ScoredCategoryNoRef {
		ScoredCategoryNoRef { category: sc.category.copy(), score: sc.score }
	}
}

impl Entry for Category {
	fn get_id(&self) -> u32 {
		self.id
	}
}

impl<'a> Entry for ScoredCategory<'a> {
	fn get_id(&self) -> u32 {
		self.category.get_id()
	}
}

impl<'a> AddAssign<f64> for ScoredCategory<'a> {
	fn add_assign(&mut self, rhs: f64) {
		self.score += rhs;
	}
}