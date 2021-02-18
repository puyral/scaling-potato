use std::io;
use std::io::Write;
use std::ops::Add;

use sprs::{CsMatI, CsVecI};

/// Applies the PageRank algorithm:
pub fn page_rank(
	m: &CsMatI<f64, u32>,
	pi_ref: &CsVecI<f64, u32>,
	beta: f64,
	epsilon: f64,
) -> CsVecI<f64, u32> {
	println!("\t=> {} categories and {} links", pi_ref.nnz(), m.nnz());
	let n = pi_ref.nnz() as f64;

	let mut pi_new = pi_ref.map(|&d| d * 1.0 / n);
	let mut pi_old; //= CsVecI::new(pi_ref.dim(), Vec::new(), Vec::new());

	let (bm, bv) = (beta, (1.0 - beta) / n);
	let pi_added = pi_ref.map(|&d| d * bv);

	let mut diff = 1.0;
	let mut i = 0;
	while diff > epsilon {
		// so the epsilon doesn't depends on size of the graph

		print!("\r\tnorm: {},\tdiff: {}", pi_new.l1_norm(), diff);
		io::stdout().flush().ok().expect("Could not flush stdout");
		pi_old = pi_new.map(|&d| -d);
		// pi_new = csr_mul_csvec(m.view(), pi_new.view()).map(|&d| d * bm).add(&pi_added);
		pi_new = (m * &pi_new).map(|&d| d * bm) + &pi_added;
		diff = pi_old.add(&pi_new).l1_norm();

		i += 1;
	}
	println!(
		"\r\tnorm: {},\tdiff: {}\n\t[DONE] (in {} loops)",
		pi_new.l1_norm(),
		diff,
		i
	);
	pi_new
}
