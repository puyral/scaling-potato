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
}