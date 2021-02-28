//! The sql extraction

use atomic_counter::{AtomicCounter, RelaxedCounter};
use rayon::prelude::*;

use crate::algebra::NonZeroCoeff;
use crate::sql_extracts::categories::category::{AbstractCategory, Category, PageRanked};
use crate::sql_extracts::categories::category::category_hash::CategoryHash;
use crate::sql_extracts::categories::category_links::{CategoryCategorySql, CategoryLinks};
use crate::sql_extracts::categories::page_category_links::{PageCategoryLinks, PageCategorySql};

pub mod categories;
pub mod extractor;

/// turns `category_links` into valid non-zero coefficients to be used for a page rank. Make sure to
/// have called [`calculate_degree`] on `categories` before hand
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

/// Calculate $\delta_{out}$ of every category. This is done sequentially and in place in `categories`
pub fn calculate_degrees<'a>(
    categories: &mut CategoryHash<Category>,
    category_links: impl Iterator<Item = &'a CategoryLinks>,
) {
    for edge in category_links {
        match categories.get_by_index_mut(edge.from) {
            None => (),
            Some(category) => category.incr_dout(),
        }
    }
}

/// update the `pr` field of the [`Category`]s
///
/// It also filters out the useless [`CategoryLinks`]. This is where the program does something useful
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

/// Get a level higher from the plain sql
///
/// `from_err_counter` and `to_err_counter` are monitoring counters to count how many dead link there were
pub fn to_category_links_vec<'a, C: AbstractCategory + Sync>(
    categories: &'a CategoryHash<C>,
    category_links: impl ParallelIterator<Item = CategoryCategorySql> + 'a,
    from_err_counter: &'a RelaxedCounter,
    to_err_counter: &'a RelaxedCounter,
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
                    to_err_counter.inc();
                    // eprintln!("no category named \"{}\", skipping...", &c.to);
                }
                if from.is_none() {
                    from_err_counter.inc();
                    // eprintln!("no category with id {}, skipping...", &c.from);
                }
                None
            }
        }
    })
}

/// Get a level higher from the plain sql
pub fn to_page_category_links_vec<'a, C: AbstractCategory + Sync>(
    categories: &'a CategoryHash<C>,
    category_links: impl ParallelIterator<Item = PageCategorySql> + 'a,
    to_err_counter: &'a RelaxedCounter,
) -> impl ParallelIterator<Item = PageCategoryLinks> + 'a {
    category_links.filter_map(move |c| match categories.get_by_title(&c.to) {
        Some(category) => Some(PageCategoryLinks {
            from: c.from,
            to: category.get_id(),
        }),
        None => {
            to_err_counter.inc();
            None
        }
    })
}

#[cfg(test)]
mod tests;
