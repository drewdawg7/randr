use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Registry<K: Eq + Hash, V>(HashMap<K, V>);

impl<K: Eq + Hash + Debug, V> Registry<K, V> {
    pub fn new(map: HashMap<K, V>) -> Self {
        Self(map)
    }

    pub fn get(&self, id: K) -> &V {
        self.0
            .get(&id)
            .unwrap_or_else(|| panic!("No spec for {id:?}"))
    }
}
