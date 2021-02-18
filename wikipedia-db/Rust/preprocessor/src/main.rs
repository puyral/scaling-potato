use std::fs::File;
use std::io;
use std::io::Write;

use clap::{App, Arg};
use termion::{color, style};

use preprocessor::{make_sql, run};

fn main() {
    let matches = App::new("Scalling-Potatoes' category preprocessor")
        .version("0.2.0")
        .author("simony2222 <7871851+simony2222@users.noreply.github.com>\nHakido")
        .about("Preprocess the wikipedia categories")
        .arg(
            Arg::with_name("categories")
                .short("c")
                .long("categories")
                .takes_value(true)
                .help(
                    "The file where to find the categories.\
			It is usually called **wiki-YYYYMMDD-page.sql",
                )
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("category links")
                .short("C")
                .long("category-links")
                .takes_value(true)
                .help(
                    "The file where to find the links between categories.\
			It is usually called **wiki-YYYYMMDD-categorylinks.sql",
                )
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("output file")
                .short("o")
                .long("out")
                .takes_value(true)
                .help("The *.sql to put the result")
                .required(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("beta")
                .short("b")
                .long("beta")
                .takes_value(true)
                .help("beta for the pagerank")
                .value_name("FLOAT"),
        )
        .arg(
            Arg::with_name("epsilon")
                .short("e")
                .long("epsilon")
                .takes_value(true)
                .help("epsilon for the pagerank")
                .value_name("FLOAT"),
        )
        .get_matches();

    // process args
    let categories_path = matches.value_of("categories").expect(&*format!(
        "{red}required argument",
        red = color::Fg(color::Red)
    ));
    let category_links_path = matches.value_of("category links").expect(&*format!(
        "{red}required argument",
        red = color::Fg(color::Red)
    ));
    let out_path = matches.value_of("output file").expect(&*format!(
        "{red}required argument",
        red = color::Fg(color::Red)
    ));
    let beta = matches
        .value_of("beta")
        .unwrap_or("0.2")
        .parse::<f64>()
        .expect(&*format!(
            "{red}beta should be a float",
            red = color::Fg(color::Red)
        ));
    let epsilon = matches
        .value_of("epsilon")
        .unwrap_or("1e-10")
        .parse::<f64>()
        .expect(&*format!(
            "{red}beta should be a float",
            red = color::Fg(color::Red)
        ));

    println!(
        "{yellow}{bold}[START]{reset_c}{reset_s}",
        bold = style::Bold,
        yellow = color::Fg(color::Yellow),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );

    //open files
    print!("Opening files...");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let text_cat = File::open(categories_path).expect(&*format!(
        "{red}Something went wrong reading the categories file : {}",
        categories_path,
        red = color::Fg(color::Red)
    ));
    let text_links = File::open(category_links_path).expect(&*format!(
        "{red}Something went wrong reading the category-links file : {}",
        category_links_path,
        red = color::Fg(color::Red)
    ));
    let mut output = File::create(out_path).expect(&*format!("can't create file {}", out_path));
    println!(
        "{green}{bold}[DONE]{reset_c}{reset_s}",
        bold = style::Bold,
        green = color::Fg(color::Green),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    );

    // do
    let (categories, category_links) = run(text_cat, text_links, beta, epsilon);
    let sql_output = make_sql(&categories.get_data(), &category_links, "eo");

    output
        .write_all(sql_output.as_bytes())
        .expect("unable to write");
    println!(
        "{yellow}{bold}[FINISHED]{reset_c}{reset_s} ({} categories and {} links)",
        categories.len(),
        category_links.len(),
        bold = style::Bold,
        yellow = color::Fg(color::Yellow),
        reset_c = color::Fg(color::Reset),
        reset_s = style::Reset
    )
}
