use crate::categories::Entry;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Debug)]
struct Page{
	id: u32
}

impl Entry for Page{
	fn get_id(&self) -> u32 {
		self.id
	}
}