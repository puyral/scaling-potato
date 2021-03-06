pub mod category;
mod page;
mod link;

pub trait Entry{
	fn get_id(&self) -> u32;
}