use serde::{Deserialize, Serialize};

use crate::categories::Entry;

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    id: u32,
}

impl Entry for Page {
    fn get_id(&self) -> u32 {
        self.id
    }
}
