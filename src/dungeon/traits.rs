use std::collections::HashMap;

use crate::{dungeon::definition::{Dungeon, DungeonRoom}, entities::mob::MobId};

impl Default for Dungeon {
    fn default() -> Self {
        let rooms = Vec::new();
        let mob_table = HashMap::from([
            (MobId::Slime, 5),
            (MobId::Goblin, 5),
            (MobId::Dragon, 1),
        ]);
        let name = "Village Dungeon".to_string();
        Self {
            name,
            rooms,
            mob_table
        }
    }
}
