use std::collections::HashMap;

use bevy::prelude::*;

use super::definition::Item;
use super::definitions::{ItemId, ItemSpec};

#[derive(Resource)]
pub struct ItemRegistry(HashMap<ItemId, ItemSpec>);

impl ItemRegistry {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn register(&mut self, spec: ItemSpec) {
        self.0.insert(spec.id, spec);
    }

    pub fn get(&self, id: ItemId) -> &ItemSpec {
        self.0
            .get(&id)
            .unwrap_or_else(|| panic!("No item spec for {id:?}"))
    }

    pub fn spawn(&self, id: ItemId) -> Item {
        self.0
            .get(&id)
            .unwrap_or_else(|| panic!("No item spec for {id:?}"))
            .to_item()
    }
}
