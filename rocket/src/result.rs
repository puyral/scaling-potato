use crate::categories::category::{Category, WeightedCategory, WeightedCategoryNoRef};
use crate::categories::Entry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::FromIterator;
use rocket_contrib::json::Json;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct Result<'a> {
	val: HashMap<u32, WeightedCategory<'a>>,
}

pub type ResultJSON = Vec<WeightedCategoryNoRef>;

impl<'a> FromIterator<WeightedCategory<'a>> for Result<'a> {
	fn from_iter<T: IntoIterator<Item = WeightedCategory<'a>>>(iter: T) -> Self {
		Result {
			val: iter
				.into_iter()
				.map(|sc| (sc.get_category().get_id(), sc))
				.collect(),
		}
	}
}

impl<'a> Result<'a> {
	pub fn to_json(&self) -> Json<ResultJSON> {
		Json(self.val.iter().map(|(_, sc)| WeightedCategoryNoRef::new(sc)).collect())
	}

	pub fn from_categories_and_weight(
		categories: &Vec<&'a Category>,
		weight: f64,
	) -> Result<'a> {
		categories.iter().map(|&c| WeightedCategory::new(c, weight)).collect()
	}

	pub fn empty() -> Result<'a> {
		Result { val: HashMap::new() }
	}
}

impl<'a> AddAssign<Result<'a>> for Result<'a> {
	fn add_assign(&mut self, rhs: Result<'a>) {
		for (id, category) in rhs.val {
			match self.val.get_mut(&id) {
				None => {self.val.insert(id, category);},
				Some(wcategory) =>
					wcategory.add_assign(category.get_weight())
			}
		}
	}
}

