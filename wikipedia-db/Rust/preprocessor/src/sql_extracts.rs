//! The sql extraction

use rayon::prelude::*;

use crate::algebra::NonZeroCoeff;
use crate::sql_extracts::categories::category::{AbstractCategory, Category, PageRanked};
use crate::sql_extracts::categories::category::category_hash::CategoryHash;
use crate::sql_extracts::categories::category_links::{CategoryCategorySql, CategoryLinks};

pub mod categories;
pub mod extractor;

pub fn calculate_nzc<'a>(
    categories: &'a CategoryHash<Category>,
    category_links: &'a Vec<CategoryLinks>,
) -> impl ParallelIterator<Item = NonZeroCoeff<u32, f64>> + 'a {
    category_links.into_par_iter().map(move |c| {
        NonZeroCoeff::new(
            c.from,
            c.to,
            categories.get_by_index(c.from).unwrap().get_dout() as f64,
        )
    })
}

pub fn calculate_degrees<'a>(
    categories: &mut CategoryHash<Category>,
    category_links: impl Iterator<Item = &'a CategoryLinks>,
) {
    for edge in category_links {
        match categories.get_by_index_mut(edge.from) {
            None => eprintln!("no category with id {}, skipping...", edge.from),
            Some(category) => category.incr_dout(),
        }
    }
}

pub fn collect_pr<'a>(
    categories: &'a mut CategoryHash<Category>,
    category_links: impl ParallelIterator<Item = &'a CategoryLinks> + 'a,
    result: impl Iterator<Item = (u32, &'a f64)>,
) -> impl ParallelIterator<Item = &'a CategoryLinks> + 'a {
    // update the page rank score
    result.for_each(|(id, &value)| categories.get_by_index_mut(id).unwrap().set_pr(value));

    // filters out the edge with too low pr score on the out node
    category_links.filter(move |&c| {
        categories.get_by_index(c.from).unwrap().get_pr()
            >= categories.get_by_index(c.to).unwrap().get_pr()
    })
}

pub fn to_category_links_vec<'a, C: AbstractCategory + Sync>(
    categories: &'a CategoryHash<C>,
    category_links: impl ParallelIterator<Item = CategoryCategorySql> + 'a,
) -> impl ParallelIterator<Item = CategoryLinks> + 'a {
    category_links.filter_map(move |c| {
        match (
            categories.get_by_title(&c.to),
            categories.get_by_index(c.from),
        ) {
            (Some(category), Some(_)) => Some(CategoryLinks {
                from: c.from,
                to: category.get_id(),
            }),
            (to, from) => {
                if to.is_none() {
                    // eprintln!("no category named \"{}\", skipping...", &c.to);
                }
                if from.is_none() {
                    // eprintln!("no category with id {}, skipping...", &c.from);
                }
                None
            }
        }
    })
}

#[cfg(test)]
mod tests;
