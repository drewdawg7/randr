use std::collections::HashMap;

use crate::{
    dungeon::definition::{Dungeon, DungeonRoom, DUNGEON_SIZE},
    mob::MobId,
};

/// Trait for game elements that can be explored and cleared.
///
/// This trait abstracts the common pattern of exploration state management,
/// allowing consistent handling of visibility and progress across different
/// game systems (dungeon rooms, map tiles, etc.).
#[allow(dead_code)]
pub trait Explorable {
    /// Returns whether this element has been visited by the player.
    fn is_visited(&self) -> bool;

    /// Returns whether this element is visible on the map.
    fn is_revealed(&self) -> bool;

    /// Returns whether this element has been completed/cleared.
    fn is_cleared(&self) -> bool;

    /// Mark this element as visited (also reveals it).
    fn visit(&mut self);

    /// Mark this element as revealed (visible but not visited).
    fn reveal(&mut self);

    /// Mark this element as cleared/completed.
    fn clear(&mut self);
}

impl Explorable for DungeonRoom {
    fn is_visited(&self) -> bool {
        self.is_visited
    }

    fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    fn is_cleared(&self) -> bool {
        self.is_cleared
    }

    fn visit(&mut self) {
        self.is_visited = true;
        self.is_revealed = true;
    }

    fn reveal(&mut self) {
        self.is_revealed = true;
    }

    fn clear(&mut self) {
        self.is_cleared = true;
    }
}

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
