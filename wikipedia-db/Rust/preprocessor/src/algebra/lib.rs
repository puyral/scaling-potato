use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::iter::FromIterator;

use ordered_float::OrderedFloat;
use rayon::iter::Either::*;
use rayon::prelude::*;
use sprs::{CsMatI, CsVecI, OuterIterator, TriMatI};

use crate::algebra::{NonZeroCoeff, NonZeroCoeffF};

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


/// (from, to, _)
pub fn make_matrix(nzcs: impl ParallelIterator<Item = (u32, u32, f64)>, dimension: usize) -> CsMatI<f64, u32> {
	let ((col_inds, row_inds), data): ((Vec<_>, Vec<_>), Vec<_>)
		= nzcs
		//.filter(|(x, y, _)| (*x < dimension as u32) && (*y < dimension as u32))
		.flat_map(|(x, y, z)| {
			vec![(Some(x), None, None), (None, Some(y), None), (None, None, Some(z))]
		}).partition_map(|v| {
		match v {
			(Some(x), None, None) => Left(Left(x)),
			(None, Some(y), None) => Left(Right(y)),
			(None, None, Some(z)) => Right(z),
			_ => panic!("unreachable")
		}
	});

	println!("dim={}, col={}, row={}", dimension, col_inds.iter().max().unwrap(), row_inds.iter().max().unwrap());
	TriMatI::from_triplets(
		(dimension, dimension),
		row_inds,
		col_inds,
		data,
	).to_csr()
}

pub fn make_vec(nzc: impl ParallelIterator<Item = u32>) -> CsVecI<f64, u32> {
	let vec = Vec::from_par_iter(nzc);
	let m = *vec.iter().max().unwrap_or(&0) + 1;
	println!("m={:}", m);

	let n = vec.len();

	CsVecI::new(m as usize, vec, vec![1.0; n])
}

pub fn collect(matrix: CsMatI<f64, u32>, vec: CsVecI<f64, u32>) -> Vec<NonZeroCoeffF> {
	assert!(matrix.is_csr());

	let tmp: HashMap<_, _> = vec.iter().collect();
	Vec::from_iter(matrix.outer_iterator().enumerate()
		.filter_map(|(row_ind, row_vec)| {
			match row_vec.iter().max_by_key(|(i, _)| OrderedFloat(**tmp.get(i).unwrap_or(&&0.0))) {
				None => None,
				Some((id, _)) => Some(NonZeroCoeffF::new(
					row_ind,
					id,
					**tmp.get(&row_ind).unwrap(),
				))
			}
		}))
}