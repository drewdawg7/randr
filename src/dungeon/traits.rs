use std::collections::HashMap;

use crate::{
    dungeon::definition::{Dungeon, DungeonRoom, DUNGEON_SIZE},
    mob::MobId,
};

impl Default for Dungeon {
    fn default() -> Self {
        // Create empty 5x5 grid
        let rooms = vec![vec![None; DUNGEON_SIZE]; DUNGEON_SIZE];
        let mob_table = HashMap::from([
            (MobId::Slime, 5),
            (MobId::Goblin, 5),
        ]);
        let name = "Village Dungeon".to_string();
        Self {
            name,
            rooms,
            mob_table,
            player_position: (0, 0),
            is_generated: false,
            boss: None,
        }
    }
}
