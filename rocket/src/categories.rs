pub mod category;
mod link;
mod page;

pub trait Entry {
    fn get_id(&self) -> u32;
}
