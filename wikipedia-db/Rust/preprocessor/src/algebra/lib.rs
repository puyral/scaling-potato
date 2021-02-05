use crate::algebra::NonZeroCoeff;
use sprs::TriMatI;

pub fn make_matrix(nzcs: impl Iterator<Item = NonZeroCoeff>, size: usize, dimension: usize) -> TriMatI<f64, u32> {
	let mut col_ind = Vec::with_capacity(size);
	let mut row_ind = Vec::with_capacity(size);
	let mut data = Vec::with_capacity(size);

	nzcs.for_each(|nzc| {
		col_ind.push(nzc.from);
		row_ind.push(nzc.to);
		data.push(1.0 / (nzc.n as f64));
	});

	TriMatI::from_triplets(
		(dimension, dimension),
		row_ind,
		col_ind,
		data
	)
}