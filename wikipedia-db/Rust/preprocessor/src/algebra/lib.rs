use std::collections::HashMap;
use std::iter::FromIterator;

use ordered_float::OrderedFloat;
use rayon::iter::Either::*;
use rayon::prelude::*;
use sprs::{CsMatI, CsVecI, TriMatI};

use crate::algebra::NonZeroCoeff;

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
	TriMatI::from_triplets(
		(dimension, dimension),
		col_inds, // We need to transpose
		row_inds,
		data,
	).to_csc()
}

pub fn make_vec(nzc: impl ParallelIterator<Item = u32>) -> CsVecI<f64, u32> {
	let vec = Vec::from_par_iter(nzc);
	let m = *vec.iter().max().unwrap_or(&0) + 1;

	let n = vec.len();

	CsVecI::new(m as usize, vec, vec![1.0; n])
}

pub fn collect(matrix: CsMatI<f64, u32>, vec: CsVecI<f64, u32>) -> Vec<NonZeroCoeff<usize, f64>> {
	assert!(matrix.is_csr());

	let tmp: HashMap<_, _> = vec.iter().collect();
	Vec::from_iter(matrix.outer_iterator().enumerate()
		.filter_map(|(row_ind, row_vec)| {
			let pr = tmp.get(&row_ind);
			match pr {
				None => None,
				Some(&&p) => Some(NonZeroCoeff::new(
					row_ind,
					row_vec.iter().max_by_key(|(i, _)| {
						OrderedFloat(**tmp.get(i).unwrap_or(&&0.0))
					}).unwrap_or((0, &0.0)).0,
					p,
				))
			}
		}))
}