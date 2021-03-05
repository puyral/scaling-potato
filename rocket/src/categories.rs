mod category;
mod page;
mod link;

trait Entry{
	fn get_id(&self) -> u32;
}