use std::fs::File;
use std::io;
use std::io::Write;

use atomic_counter::{AtomicCounter, RelaxedCounter};
use rayon::prelude::*;
use termion::{color, style};

use crate::sql_extracts::categories::category::category_hash::CategoryHash;
use crate::sql_extracts::categories::category::{AbstractCategory, Category, PageRanked};
use crate::sql_extracts::categories::category_links::{CategoryCategorySql, CategoryLinks};
use crate::sql_extracts::extractor::Extractor;

pub mod algebra;
mod sql_extracts;

#[cfg(test)]
mod tests;

/*#[derive(Debug)]
struct CategoryF {
    id: u32,
    parent: u32,
    data: f64,
}

pub fn extract(text: String) -> VecDeque<Category> {
    let extractor = Extractor::new::<Category>().unwrap();
    let mut vect: VecDeque<Category> = VecDeque::new();
    extractor.extract(&text, &mut vect);
    return vect;
}

pub fn extract_categories(text: String) -> HashMap<String, Category> {
    HashMap::from_iter(
        Extractor::new::<Category>().unwrap().extract_iter::<Category>(&text)
            .map(|cat| (String::from(cat.get_title()), cat))
    )
}*/

/// do ^^'
pub fn run(
    categories_files: File,
    category_links_file: File,
    beta: f64,
    epsilon: f64,
) -> (CategoryHash<Category>, Vec<CategoryLinks>) {
    print!("Parsing Categories...");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut categories: CategoryHash<_> =
        Extractor::extract_par_iter_file::<Category>(categories_files).collect();
    // println!("[DONE] ({} categories)", categories.len());
    println!(
        "{green}{bold}[DONE]{reset_c}{reset_s} ({} categories)",
        categories.len(),
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );

    print!("Parsing and preprocessing links...");
    io::stdout().flush().ok().expect("Could not flush stdout");

    let from_err_counter = RelaxedCounter::new(0);
    let to_err_counter = RelaxedCounter::new(0);
    let category_links: Vec<_> = sql_extracts::to_category_links_vec(
        &categories,
        Extractor::extract_par_iter_file::<CategoryCategorySql>(category_links_file),
        &from_err_counter,
        &to_err_counter,
    )
    .collect();
    println!("{green}{bold}[DONE]{reset_c}{reset_s}\n\t({} links, skipped {} unknown ids (to) and {} unknown titles (from))",
			 category_links.len(),
			 to_err_counter.get(),
			 from_err_counter.get(),
			 bold = style::Bold,
			 green = color::Fg(color::Green),
			 reset_c = color::Fg(color::Reset),
			 reset_s = style::Reset
	);

    print!("Calculating degrees...");
    io::stdout().flush().ok().expect("Could not flush stdout");
    sql_extracts::calculate_degrees(&mut categories, category_links.iter());
    println!("[DONE]");

    print!("Switching to Algebra...");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let vec = algebra::lib::make_vec(categories.get_data().par_iter().map(|c| c.get_id()));
    let matrix = algebra::lib::make_matrix(
        sql_extracts::calculate_nzc(&categories, &category_links).map(|c| c.to_tuple_calculate()),
        vec.dim(),
    );
    println!(
        "{green}{bold}[DONE]{reset_c}{reset_s}",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );

    println!("Pageranking...");
    let page_rank = algebra::page_rank::page_rank(&matrix, &vec, beta, epsilon);

    print!("Extracting results...");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let final_category_links: Vec<_> = sql_extracts::collect_pr(
        &mut categories,
        category_links.par_iter(),
        page_rank.iter().map(|(id, value)| (id as u32, value)),
    )
    .collect();
    println!(
        "{green}{bold}[DONE]{reset_c}{reset_s}",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );

    print!("Finalizing...");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut out_category_links = Vec::with_capacity(final_category_links.len());
    for &c in final_category_links {
        out_category_links.push(c)
    }
    println!(
        "{green}{bold}[DONE]{reset_c}{reset_s}",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );
    (categories, out_category_links)
}

/// Print the result to a sql string.
///
/// `wiki_name` is used to generate the names of the table (`{wiki_name}-categories`
/// and `{wiki_name}-category-category`)
pub fn make_sql(
    categories: &Vec<Category>,
    category_links: &Vec<CategoryLinks>,
    wiki_name: &str,
) -> String {
    println!("Exporting to sql file...");
    let mut out = String::new();

    // make the table
    out.push_str(&*format!(
        "BEGIN;
		DROP TABLE IF EXISTS `{wiki_name}-categories`;
		CREATE TABLE `{wiki_name}-categories` (
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
        wiki_name = wiki_name
    ));
    print!("\tcreating table...");
    println!(
        "{green}{bold}[DONE]{reset_c}{reset_s}",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );

    print!("\tadding data...");
    io::stdout().flush().ok().expect("Could not flush stdout");
    // the poor man's multithreading
    let mut categories_str = String::new();
    let mut categories_links_str = String::new();
    rayon::scope(|s| {
        // categories
        s.spawn(|_| {
            categories.iter().for_each(|c| {
                categories_str.push_str(&*format!(
                    "INSERT INTO `{wiki_name}-categories` VALUES ({id}, {page_rank});\n",
                    wiki_name = wiki_name,
                    id = c.get_id(),
                    page_rank = c.get_pr()
                ))
            });
        });

        // links
        s.spawn(|_| {
            category_links.iter().for_each(|c| {
                categories_links_str.push_str(&*format!(
                    "INSERT INTO `{wiki_name}-category-category` VALUES ({from}, {to});\n",
                    wiki_name = wiki_name,
                    from = c.from,
                    to = c.to
                ))
            });
        });
    });
    println!(
        "{green}{bold}[DONE]{reset_c}{reset_s}",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );

    print!("\tMerging Strings and finalizing...");
    io::stdout().flush().ok().expect("Could not flush stdout");
    // merging
    out.push_str(&categories_str);
    out.push_str(&categories_links_str);

    // finalize commit
    out.push_str("COMMIT;");
    println!(
        "{green}{bold}[DONE]{reset_c}{reset_s}",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );

    out
}
