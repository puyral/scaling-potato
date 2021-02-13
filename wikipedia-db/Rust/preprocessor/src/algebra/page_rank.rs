use std::ops::{Add, Mul};

use sprs::{CsMatI, CsVecI, SpIndex};
use sprs::prod::csr_mul_csvec;

/// Applies the PageRank algorithm:
pub fn page_rank(m: &CsMatI<f64, u32>, pi_ref: &CsVecI<f64, u32>, beta: f64, epsilon: f64) -> CsVecI<f64, u32> {
	let mut pi_new: CsVecI<f64, u32> = pi_ref.map(|&d| 1.0 / (pi_ref.nnz() as f64));
	let mut pi_old: CsVecI<f64, u32> = CsVecI::new(pi_ref.dim(), Vec::new(), Vec::new());

	let n = pi_ref.nnz() as f64;
	let (bv, bm) = ((1.0 - beta), beta);

	while (pi_old.map(|&d| -d) + &pi_new).l1_norm() / n > epsilon { // so the epsilon doesn't depends on size of the graph
		pi_old = pi_new;
		pi_new = csr_mul_csvec(m.view(), pi_old.view()).map(|&d| d * bm)
			+ pi_old.map(|&d| d * bv);
	}
	pi_new
}