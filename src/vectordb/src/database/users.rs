use crate::database::memory::{get_stable_btree_memory, Memory};
use crate::database::error::Error;
use ic_stable_structures::StableBTreeMap;

use std::cell::RefCell;

thread_local! {
    pub static ADMINS: RefCell<StableBTreeMap<String, bool, Memory>> = RefCell::new(init_stable_data());
}

fn init_stable_data() -> StableBTreeMap<String, bool, Memory> {
    StableBTreeMap::init(get_stable_btree_memory())
}
