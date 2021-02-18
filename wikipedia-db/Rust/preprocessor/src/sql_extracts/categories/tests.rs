//! Tests
use std::fs;
use std::iter::FromIterator;

use crate::sql_extracts::categories::category::Category;
use crate::sql_extracts::categories::category_links::CategoryCategorySql;
use crate::sql_extracts::extractor::Extractor;

#[test]
fn extract_categories_iter() -> () {
    let text = fs::read_to_string("test_samples/nrm/nrmwiki-20210201-page.sql")
        .expect("Something went wrong reading the file");
    let v = Vec::from_iter(
        Extractor::new::<Category>()
            .expect("Something went wrong building the regexp")
            .extract_iter::<Category>(&text),
    );
    // println!("assert_eq!({:?},v[89]);\nassert_eq!({:?},v[7]);\nassert_eq!({:?},v[120]);\nassert_eq!({},v.len());",v[89],v[7],v[120],v.len());
    // assert_eq!(Category { id: 6106, title: "Ûrope".parse().unwrap() }, v[89]);
    // assert_eq!(Category { id: 2110, title: "Mort_en_2002".parse().unwrap() }, v[7]);
    // assert_eq!(Category { id: 6985, title: "Mort_en_1820".parse().unwrap() }, v[120]);
    assert_eq!(774, v.len());
}

#[test]
fn extract_links() -> () {
    let text_links = fs::read_to_string("test_samples/nrm/nrmwiki-20210201-categorylinks.sql")
        .expect("Something went wrong reading the file one_line_categorieslinks");

    let v = Vec::from_iter(
        Extractor::new::<CategoryCategorySql>()
            .unwrap()
            .extract_iter::<CategoryCategorySql>(&text_links),
    );

    assert_eq!(
        CategoryCategorySql {
            from: 9310,
            to: "Féchouneus_uk".parse().unwrap(),
        },
        v[89]
    );
    assert_eq!(
        CategoryCategorySql {
            from: 2507,
            to: "Astrononmie".parse().unwrap(),
        },
        v[7]
    );
    assert_eq!(
        CategoryCategorySql {
            from: 10251,
            to: "Ûrope".parse().unwrap(),
        },
        v[120]
    );
    assert_eq!(147, v.len());
}
