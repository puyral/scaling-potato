mod lib;
pub mod page_rank;

#[cfg(test)]
mod tests;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct NonZeroCoeff {
	from: u32,
	to: u32,
	n: u32,
}

impl NonZeroCoeff {
	pub fn new(from: u32, to: u32, n: u32) -> Self {
		NonZeroCoeff { from, to, n }
	}

	#[cfg(test)]
	#[allow(dead_code)]
	pub fn serialize(&self) -> String {
		format!("NonZeroCoeff::new({},{},{})", self.from, self.to, self.n)
	}

	pub fn to_tuple_calculate(&self) -> (u32, u32, f64) {
		(self.from, self.to, 1.0 / (self.n as f64))
	}
}