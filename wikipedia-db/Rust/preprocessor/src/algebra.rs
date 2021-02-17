#[cfg(test)]
use std::fmt::Display;

pub mod lib;
pub mod page_rank;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct NonZeroCoeff<I, D> {
	from: I,
	to: I,
	data: D,
}

impl<I, D> NonZeroCoeff<I, D> {
	pub fn new(from: I, to: I, data: D) -> Self {
		NonZeroCoeff { from, to, data }
	}
}

impl<I, D> NonZeroCoeff<I, D> where I: Copy, D: Copy {
	pub fn to_tuple(&self) -> (I, I, D) {
		(self.from, self.to, self.data)
	}
	pub fn get_data(&self) -> D { self.data }
}

impl NonZeroCoeff<usize, f64> {
	pub fn cast(&self) -> NonZeroCoeff<u32, f64> {
		NonZeroCoeff {
			from: self.from as u32,
			to: self.to as u32,
			data: self.data as f64,
		}
	}
}

impl NonZeroCoeff<u32, f64> {
	pub fn cast(&self) -> NonZeroCoeff<usize, f64> {
		NonZeroCoeff {
			from: self.from as usize,
			to: self.to as usize,
			data: self.data as f64,
		}
	}
}

impl<I> NonZeroCoeff<I, f64> where I: Copy {
	pub fn to_tuple_calculate(&self) -> (I, I, f64) {
		(self.from, self.to, 1.0 / self.data)
	}
}

#[cfg(test)]
impl<I: Display, D: Display> NonZeroCoeff<I, D> {
	#[allow(dead_code)]
	pub fn serialize(&self) -> String {
		format!("NonZeroCoeff::new({},{},{})", self.from, self.to, self.data)
	}
}