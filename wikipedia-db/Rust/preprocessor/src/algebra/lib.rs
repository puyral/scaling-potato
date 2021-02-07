use rayon::iter::Either::*;
use rayon::prelude::{FromParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use sprs::{CsMatI, CsVecI, TriMatI};

use crate::algebra::NonZeroCoeff;

// pub fn make_matrix(nzcs: impl Iterator<Item = NonZeroCoeff>, size: usize, dimension: usize) -> CsMatI<f64, u32> {
// 	let mut col_ind = Vec::with_capacity(size);
// 	let mut row_ind = Vec::with_capacity(size);
// 	let mut data = Vec::with_capacity(size);
//
// 	nzcs.for_each(|nzc| {
// 		col_ind.push(nzc.from);
// 		row_ind.push(nzc.to);
// 		data.push(1.0 / (nzc.n as f64));
// 	});
//
// 	TriMatI::from_triplets(
// 		(dimension, dimension),
// 		row_ind,
// 		col_ind,
// 		data
// 	).to_csc()
// }

pub fn make_matrix(nzcs: impl ParallelIterator<Item = (u32, u32, f64)>, dimension: usize) -> CsMatI<f64, u32> {
	let ((col_ind, row_ind), data): ((Vec<_>, Vec<_>), Vec<_>)
		= nzcs.flat_map(|(x, y, z)| {
		vec![(Some(x), None, None), (None, Some(y), None), (None, None, Some(z))]
	}).partition_map(|v| {
		match v {
			(Some(x), None, None) => Left(Left(x)),
			(None, Some(y), None) => Left(Right(y)),
			(None, None, Some(z)) => Right(z),
			_ => panic!("impossible")
		}
	});

	// nzcs.for_each(|nzc| {
	// 	col_ind.push(nzc.0);
	// 	row_ind.push(nzc.1);
	// 	data.push(nzc.2);
	// });

	TriMatI::from_triplets(
		(dimension, dimension),
		row_ind,
		col_ind,
		data,
	).to_csc()
}

pub fn make_vec(nzc: impl ParallelIterator<Item = u32>) -> CsVecI<f64, u32> {
	let vec = Vec::from_par_iter(nzc);
	let m = *vec.iter().max().unwrap_or(&0) + 1;

	let n = vec.len();

	CsVecI::new(m as usize, vec, vec![1.0; n])
}