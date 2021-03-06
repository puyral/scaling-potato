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
pub struct WeightedCategory<'a> {
	category: &'a Category,
	weight: f64,
}

impl<'a> WeightedCategory<'a> {
	pub fn get_category(&self) -> &'a Category {
		self.category
	}

	pub fn get_weight(&self) -> f64 {
		self.weight
	}

	pub fn set_weight(&mut self, weight: f64) {
		self.weight = weight;
	}

	pub fn new(category: &'a Category, weight: f64) -> Self {
		WeightedCategory { category, weight }
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeightedCategoryNoRef {
	category: Category,
	weight: f64,
}

impl WeightedCategoryNoRef {
	pub fn new(sc: &WeightedCategory<'_>) -> WeightedCategoryNoRef {
		WeightedCategoryNoRef { category: sc.category.copy(), weight: sc.weight }
	}
}

impl Entry for Category {
	fn get_id(&self) -> u32 {
		self.id
	}
}

impl<'a> Entry for WeightedCategory<'a> {
	fn get_id(&self) -> u32 {
		self.category.get_id()
	}
}

impl<'a> AddAssign<f64> for WeightedCategory<'a> {
	fn add_assign(&mut self, rhs: f64) {
		self.weight += rhs;
	}
}