use std::cmp::{max, min};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::AddAssign;

use rocket_contrib::json::Json;

use crate::categories::category::{Category, WeightedCategory, WeightedCategoryNoRef};
use crate::categories::Entry;

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
    pub fn to_json(&self, limit: Option<usize>) -> Json<ResultJSON> {
        let mut tmp = self
            .val
            .iter()
            .map(|(_, sc)| WeightedCategoryNoRef::new(sc))
            .collect::<Vec<_>>();
        tmp.sort_unstable_by(|a, b| a.cmp(b).reverse());

        match limit {
            None => Json(tmp),
            Some(l) => {
                let mut i = 0;
                let n = tmp.len();

                if n == 0 {
                    return Json(tmp);
                }

                let old_weight = tmp[0].weight();
                while (i < n) && (old_weight == tmp[1].weight()) {
                    i += 1;
                }
                let b = min(n, i + l);
                let a = max(0, b - l);

                return Json(tmp[a..b].to_vec());
            }
        }
    }

    pub fn from_categories_and_weight(categories: &Vec<&'a Category>, weight: usize) -> Result<'a> {
        categories
            .iter()
            .map(|&c| WeightedCategory::new(c, weight))
            .collect()
    }

    pub fn empty() -> Result<'a> {
        Result {
            val: HashMap::new(),
        }
    }
}

impl<'a> AddAssign<Result<'a>> for Result<'a> {
    fn add_assign(&mut self, rhs: Result<'a>) {
        for (id, category) in rhs.val {
            match self.val.get_mut(&id) {
                None => {
                    self.val.insert(id, category);
                }
                Some(wcategory) => wcategory.add_assign(category.get_weight()),
            }
        }
    }
}
