use std::ops::{Add, Mul};

use sprs::{CsMatI, CsVecI, SpIndex};
use sprs::prod::csr_mul_csvec;

pub fn page_rank(m: &CsMatI<f64, u32>, pi_ref: &CsVecI<f64, u32>, beta: f64, epsilon: f64) -> CsVecI<f64, u32> {
	let mut pi_new: CsVecI<f64, u32> = pi_ref.to_owned().map(|d| d / (pi_ref.nnz() as f64));
	let mut pi_old: CsVecI<f64, u32> = CsVecI::new(pi_ref.dim(), Vec::new(), Vec::new());


	// let n = pi_ref.dim() as f64;
	let (bm, bv) = ((1.0 - beta), beta);

	// println!("{:?}", pi_new.view());
	// println!("{}", bm);
	// println!("{:?}", csr_mul_csvec(m.view(), pi_new.view()).map(|d| d * bm));
	// println!("{:?}", csr_mul_csvec(m.view(), pi_new.view()).map(|d| d * bm) + pi_new.map(|d| d * bv));


	while (pi_old.map(|&d| -d) + &pi_new).l1_norm() > epsilon {
		pi_old = pi_new;
		pi_new = csr_mul_csvec(m.view(), pi_old.view()).map(|d| d * bm)
			+ pi_old.map(|d| d * bv);
	}
	pi_new.to_owned()
}