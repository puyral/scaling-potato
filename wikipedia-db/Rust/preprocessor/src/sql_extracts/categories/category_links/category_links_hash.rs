// use crate::sql_extracts::categories::category::{Category, AbstractCategory};
// use std::collections::HashMap;
// use rayon::prelude::*;
// use std::rc::Rc;
// use std::iter::FromIterator;
// use crate::sql_extracts::categories::category_links::CategoryCategorySql;
// use crate::sql_extracts::categories::category::category_hash::CategoryHash;
//
// pub struct CategoryLinksHash {
// 	hash: HashMap<u32, Vec<u32>>
// }
//
// impl CategoryLinksHash {
// 	pub fn new<C>(categories: CategoryHash<C>, iter: impl Iterator<Item = (u32,u32)>) -> Self where
// 		C: AbstractCategory {
// 		let hash = HashMap::with_capacity(categories.len());
//
// 		iter.for_each(|(from, to)|{
// 			hash.entry()
// 		});
// 		CategoryLinksHash{hash}
// 	}
// }
//
//
// // impl<C: AbstractCategory> CategoryHash<C>{
// // 	pub fn get_by_index(&self, i:u32)-> Option<&C> {
// // 		match self.by_indices.get(&i){
// // 			None => None,
// // 			Some(&x) => Some(&self.vec[x])
// // 		}
// // 	}
// //
// // 	pub fn get_by_title(&self, s: &str)-> Option<&C> {
// // 		match self.by_title.get(s){
// // 			None => None,
// // 			Some(&x) => Some(&self.vec[x])
// // 		}
// // 	}
// // }