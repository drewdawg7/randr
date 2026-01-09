use bevy::prelude::*;

use crate::inventory::Inventory;

#[derive(Resource, Debug)]
pub struct Storage {
    pub inventory: Inventory,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new_unlimited(),
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}
