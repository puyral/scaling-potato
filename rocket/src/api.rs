use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::{Categories, Db};
use crate::categories::category::WeightedCategory;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeightedCategoriesListInput {
    categories: Vec<u32>,
    weight: usize,
}

#[get("/<wp>/category?<id>&<weight>&<limit>")]
pub fn simple_category_get(
    categories: State<Categories>,
    wp: String,
    id: u32,
    limit: Option<usize>,
    weight: Option<usize>,
) -> Option<Json<crate::result::ResultJSON>> {
    let category_hash = categories.inner().get(&wp)?;
    let category = category_hash.get(id)?;

    let top = category_hash.build_top_categories_intern(std::iter::once(category));

    let weight = weight.unwrap_or(1);
    Some(
        top.iter()
            .map(|&c| WeightedCategory::new(c, weight))
            .collect::<crate::result::Result>()
            .to_json(limit),
    )
}

#[post("/<wp>/category?<limit>", data = "<data>")]
pub fn categories_post(
    categories: State<Categories>,
    wp: String,
    limit: Option<usize>,
    data: Json<Vec<WeightedCategoriesListInput>>,
) -> Option<Json<crate::result::ResultJSON>> {
    let category_hash = categories.inner().get(&wp)?;

    let mut r = crate::result::Result::empty();

    for wcategories in data.into_inner() {
        let top = category_hash.build_top_categories_intern(
            wcategories
                .categories
                .iter()
                .flat_map(|&id| category_hash.get(id)),
        );
        let r_tmp = crate::result::Result::from_categories_and_weight(&top, wcategories.weight);

        r += r_tmp
    }
    Some(r.to_json(limit))
}

#[get("/<wp>/page?<id>&<weight>&<limit>")]
pub fn simple_page_get(
    categories: State<Categories>,
    conn: Db,
    wp: String,
    id: u32,
    limit: Option<usize>,
    weight: Option<usize>,
) -> Option<Json<crate::result::ResultJSON>> {
    let category_hash = categories.inner().get(&wp)?;
    let mut smt = conn
        .prepare(&*format!(
            "select to_id from `{}-page-category` where from_id={};",
            &wp, id
        ))
        .ok()?;
    let categories = smt
        .query_map(&[], |row| category_hash.get(row.get(0)))
        .ok()?
        .flat_map(|c| c.ok()?);

    let top = category_hash.build_top_categories_intern(categories);

    let weight = weight.unwrap_or(1);

    Some(
        top.iter()
            .map(|&c| WeightedCategory::new(c, weight))
            .collect::<crate::result::Result>()
            .to_json(limit),
    )
}
