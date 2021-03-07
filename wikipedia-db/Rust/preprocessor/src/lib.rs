use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};

use atomic_counter::{AtomicCounter, RelaxedCounter};
use rayon::prelude::*;

use crate::cli::{only_r, print_done, Printer};
use crate::sql_extracts::categories::category::{AbstractCategory, Category, PageRanked};
use crate::sql_extracts::categories::category::category_hash::CategoryHash;
use crate::sql_extracts::categories::category_links::{CategoryCategorySql, CategoryLinks};
use crate::sql_extracts::categories::page_category_links::{PageCategoryLinks, PageCategorySql};
use crate::sql_extracts::extractor::Extractor;
use regex::Regex;

pub mod algebra;
pub mod cli;
mod sql_extracts;

#[cfg(test)]
mod tests;

/// do ^^'
pub fn run(
	categories_files: File,
	category_links_file: File,
	beta: f64,
	epsilon: f64,
) -> (CategoryHash<Category>, Vec<CategoryLinks>) {
	let mut categories = print_done("Parsing Categories...", || {
		let categories: CategoryHash<_> =
			Extractor::extract_par_iter_file::<Category>(categories_files).collect();
		let str = format!("{} categories", categories.len());
		cli::res(categories, Some(str))
	});

	let category_links: Vec<_> = print_done("Parsing and preprocessing links...", || {
		let from_err_counter = RelaxedCounter::new(0);
		let to_err_counter = RelaxedCounter::new(0);
		let category_links: Vec<_> = sql_extracts::to_category_links_vec(
			&categories,
			Extractor::extract_par_iter_file::<CategoryCategorySql>(category_links_file),
			&from_err_counter,
			&to_err_counter,
		)
			.collect();
		let str = format!(
			"{} links, skipped {} unknown titles (to) and {} unknown ids (from)",
			category_links.len(),
			to_err_counter.get(),
			from_err_counter.get()
		);
		cli::res(category_links, Some(str))
	});

	print_done("Calculating degrees...", || {
		sql_extracts::calculate_degrees(&mut categories, category_links.iter());
		cli::NONE
	});

	let (vec, matrix) = print_done("Switching to Algebra...", || {
		let vec = algebra::lib::make_vec(categories.get_data().par_iter().map(|c| c.get_id()));
		let matrix = algebra::lib::make_matrix(
			sql_extracts::calculate_nzc(&categories, &category_links)
				.map(|c| c.to_tuple_calculate()),
			vec.dim(),
		);

		cli::only_r((vec, matrix))
	});

	println!("Pageranking...");
	let page_rank = algebra::page_rank::page_rank(&matrix, &vec, beta, epsilon);

	let final_category_links: Vec<_> = print_done("Extracting results...", || {
		cli::only_r(sql_extracts::collect_pr(
			&mut categories,
			category_links.par_iter(),
			page_rank.iter().map(|(id, value)| (id as u32, value)),
		))
	})
		.collect();

	let out_category_links = print_done("Finalizing...", || {
		let mut out_category_links = Vec::with_capacity(final_category_links.len());
		for &c in final_category_links {
			out_category_links.push(c)
		}
		cli::only_r(out_category_links)
	});
	(categories, out_category_links)
}

pub fn run_pages(
	categories: &CategoryHash<Category>,
	category_links_file: File,
) -> Vec<PageCategoryLinks> {
	print_done("Parsing and preprocessing page links...", || {
		let to_err_counter = RelaxedCounter::new(0);
		let page_category_links: Vec<_> = sql_extracts::to_page_category_links_vec(
			&categories,
			Extractor::extract_par_iter_file::<PageCategorySql>(category_links_file),
			&to_err_counter,
		)
			.collect();

		let str = format!(
			"{} links, skipped  {} unknown titles (to)",
			page_category_links.len(),
			to_err_counter.get()
		);
		Printer {
			result: page_category_links,
			description: Some(str),
		}
	})
}

/// Print the result to a sql string.
///
/// `wp_code` is used to generate the names of the table (`{wp_code}-categories`
/// and `{wp_code}-category-category`) see [List of Wikipedias](https://en.wikipedia.org/wiki/List_of_Wikipedias#Editions_overview)
pub fn make_sql(
	categories: &Vec<Category>,
	category_links: &Vec<CategoryLinks>,
	wp_code: &str,
) -> String {
	println!("Exporting to sql file...");
	let mut out = String::new();

	print_done("\tcreating table...", || {
		// make the table
		out.push_str(&*format!(
			"BEGIN;
		DROP TABLE IF EXISTS `{wiki_name}-categories`;
		CREATE TABLE `{wiki_name}-categories` (
			`name` VARCHAR NOT NULL,
			`id` INTEGER NOT NULL PRIMARY KEY,
			`page_rank` REAL NOT NULL
		);
		DROP TABLE IF EXISTS `{wiki_name}-category-category`;
		CREATE TABLE `{wiki_name}-category-category` (
			`from_id` INTEGER NOT NULL,
			`to_id` INTEGER NOT NULL,
			FOREIGN KEY(`from_id`) REFERENCES `{wiki_name}-categories`(`id`),
			FOREIGN KEY(`to_id`) REFERENCES `{wiki_name}-categories`(`id`)
		);\n",
			wiki_name = wp_code
		));

		cli::NONE
	});

	// the poor man's multithreading
	let mut categories_str = String::new();
	let mut categories_links_str = String::new();
	print_done("\tadding data...", || {
		rayon::scope(|s| {
			// categories
			s.spawn(|_| {
				categories.iter().for_each(|c| {
					categories_str.push_str(&*format!(
						"INSERT INTO `{wiki_name}-categories` VALUES ('{name}', {id}, {page_rank});\n",
						wiki_name = wp_code,
						id = c.get_id(),
						page_rank = c.get_pr(),
						name = prepare(c.get_title())
					))
				});
			});

			// links
			s.spawn(|_| {
				category_links.iter().for_each(|c| {
					categories_links_str.push_str(&*format!(
						"INSERT INTO `{wiki_name}-category-category` VALUES ({from}, {to});\n",
						wiki_name = wp_code,
						from = c.from,
						to = c.to
					))
				});
			});
		});

		cli::NONE
	});

	print_done("\tMerging Strings and finalizing...", || {
		// merging
		out.push_str(&categories_str);
		out.push_str(&categories_links_str);

		// finalize commit
		out.push_str("COMMIT;");
		only_r(out)
	})
}

pub fn make_page_sql(
	page_category_links: &Vec<PageCategoryLinks>,
	wp_code: &str,
	mut output: File,
) {
	println!("Exporting page to sql file...");

	print_done("\tcreating table...", || {
		// make the table
		output
			.write(
				format!(
					"BEGIN;
		DROP TABLE IF EXISTS `{wiki_name}-page-category`;
		CREATE TABLE `{wiki_name}-page-category` (
			`from_id` INTEGER NOT NULL,
			`to_id` INTEGER NOT NULL,
			FOREIGN KEY(`to_id`) REFERENCES `{wiki_name}-categories`(`id`)
		);\n",
					wiki_name = wp_code
				)
					.as_bytes(),
			)
			.expect("couldn't write");
		cli::NONE
	});

	// the rich man's multithreading :)
	let arc_m = Arc::new(Mutex::new(output));
	let chunk_size = 10000;
	print_done(&format!("\twriting by groups of {}...", chunk_size), || {
		page_category_links.par_chunks(chunk_size).for_each(|pcl| {
			let mut tmp = String::new();
			pcl.iter().for_each(|pc| {
				tmp.push_str(&*format!(
					"INSERT INTO `{wiki_name}-page-category` VALUES ({from}, {to});\n",
					wiki_name = wp_code,
					from = pc.from,
					to = pc.to
				))
			});
			let mut file = arc_m.lock().expect("unable to lock...");
			file.write(tmp.as_bytes()).expect("unable to write...");
		});

		// finalize commit
		let mut file = arc_m.lock().expect("unable to lock...");
		file.write("COMMIT;".as_bytes())
			.expect("unable to write...");

		cli::NONE
	});
}

fn prepare(title: &str) -> String {
	title.replace("\\", "").replace("'", "''")
}

#[cfg(test)]
mod tests_local{

	#[test]
	fn replace(){

		println!("{}",crate::prepare("this \\\" is ' a test"))
	}
}