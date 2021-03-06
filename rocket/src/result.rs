use crate::categories::category::{Category, ScoredCategory, ScoredCategoryNoRef};
use crate::categories::Entry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::FromIterator;
use rocket_contrib::json::Json;

#[derive(Debug)]
pub struct Result <'a>{
	val: HashMap<u32, ScoredCategory<'a>>,
}

pub type ResultJSON = Vec<ScoredCategoryNoRef>;

impl<'a> FromIterator<ScoredCategory<'a>> for Result<'a> {
	fn from_iter<T: IntoIterator<Item = ScoredCategory<'a>>>(iter: T) -> Self {
		Result {
			val: iter
				.into_iter()
				.map(|sc| (sc.get_category().get_id(), sc))
				.collect(),
		}
	}
}

impl<'a> Result<'a>{
	pub fn to_json(&self) -> Json<ResultJSON> {
		Json(self.val.iter().map(|(_, sc)| ScoredCategoryNoRef::new(sc)).collect())
	}
}

