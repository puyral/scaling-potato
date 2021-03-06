use std::cmp::Ordering;
use std::ops::AddAssign;

use serde::{Deserialize, Serialize};

use crate::categories::Entry;

pub mod category_hash;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Category {
    id: u32,
    page_rank: f64,
}

impl Category {
    fn copy(&self) -> Category {
        Category {
            id: self.id,
            page_rank: self.page_rank,
        }
    }
}

#[derive(Debug)]
pub struct WeightedCategory<'a> {
    category: &'a Category,
    weight: usize,
}

impl<'a> WeightedCategory<'a> {
    pub fn get_category(&self) -> &'a Category {
        self.category
    }

    pub fn get_weight(&self) -> usize {
        self.weight
    }

    pub fn set_weight(&mut self, weight: usize) {
        self.weight = weight;
    }

    pub fn new(category: &'a Category, weight: usize) -> Self {
        WeightedCategory { category, weight }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct WeightedCategoryNoRef {
    category: Category,
    weight: usize,
}

impl WeightedCategoryNoRef {
    pub fn new(sc: &WeightedCategory<'_>) -> WeightedCategoryNoRef {
        WeightedCategoryNoRef {
            category: sc.category.copy(),
            weight: sc.weight,
        }
    }

    pub fn cmp(&self, rhs: &WeightedCategoryNoRef) -> Ordering {
        match self.weight.cmp(&rhs.weight) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.category.page_rank.total_cmp(&rhs.category.page_rank),
            Ordering::Greater => Ordering::Greater,
        }
    }

    pub fn weight(&self) -> usize {
        self.weight
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

impl<'a> AddAssign<usize> for WeightedCategory<'a> {
    fn add_assign(&mut self, rhs: usize) {
        self.weight += rhs;
    }
}
