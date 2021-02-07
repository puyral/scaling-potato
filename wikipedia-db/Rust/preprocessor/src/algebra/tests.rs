use rayon::prelude::IntoParallelIterator;

use crate::algebra::lib::{make_matrix, make_vec};
use crate::algebra::page_rank::page_rank;

#[test]
fn page_rank1() {
	let p = make_vec((0..(3 as u32)).into_par_iter());
	let m = make_matrix(
		vec![(0, 0, 0.8), (1, 0, 0.2), (1, 1, 1.0), (2, 2, 1.0)].into_par_iter(),
		p.dim());

	let other = page_rank(&m, &p, 0.1, 0.1);

	println!("{:?}", other);

	();
}