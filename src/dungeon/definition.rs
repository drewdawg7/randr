use std::collections::HashMap;

use rand::Rng;

use crate::{
    chest::Chest,
    dungeon::enums::{Direction, DungeonError, RoomType},
    entities::mob::{Mob, MobId},
    loot::{HasLoot, LootDrop},
    system::game_state,
};

/// Grid size for the dungeon (5x5)
pub const DUNGEON_SIZE: usize = 5;
/// Maximum percentage of grid that can have rooms (50%)
pub const MAX_FILL_PERCENT: f32 = 0.50;

#[derive(Debug)]
pub struct Dungeon {
    pub name: String,
    /// 2D grid of optional rooms - None means no room at that position
    pub rooms: Vec<Vec<Option<DungeonRoom>>>,
    pub mob_table: HashMap<MobId, i32>,
    pub player_position: (i32, i32),
    pub is_generated: bool,
}

impl Dungeon {
    /// Get neighbors of a room (for navigation display)
    pub fn get_neighbors(&self, room: &DungeonRoom) -> Vec<Option<&DungeonRoom>> {
        let x = room.x;
        let y = room.y;
        vec![
            self.get_room(x, y - 1), // North
            self.get_room(x + 1, y), // East
            self.get_room(x, y + 1), // South
            self.get_room(x - 1, y), // West
        ]
    }

    /// Get a room at the given coordinates
    pub fn get_room(&self, x: i32, y: i32) -> Option<&DungeonRoom> {
        if x < 0 || x >= DUNGEON_SIZE as i32 || y < 0 || y >= DUNGEON_SIZE as i32 {
            return None;
        }
        self.rooms[y as usize][x as usize].as_ref()
    }

    /// Get a mutable reference to a room at the given coordinates
    pub fn get_room_mut(&mut self, x: i32, y: i32) -> Option<&mut DungeonRoom> {
        if x < 0 || x >= DUNGEON_SIZE as i32 || y < 0 || y >= DUNGEON_SIZE as i32 {
            return None;
        }
        self.rooms[y as usize][x as usize].as_mut()
    }

    /// Get the current room the player is in
    pub fn current_room(&self) -> Option<&DungeonRoom> {
        self.get_room(self.player_position.0, self.player_position.1)
    }

    /// Get mutable reference to the current room
    pub fn current_room_mut(&mut self) -> Option<&mut DungeonRoom> {
        let (x, y) = self.player_position;
        self.get_room_mut(x, y)
    }

    /// Spawn a random mob from the dungeon's mob table
    pub fn spawn_mob(&self) -> Result<Mob, DungeonError> {
        let total_weight: i32 = self.mob_table.values().sum();
        if total_weight == 0 {
            return Err(DungeonError::MobSpawnError);
        }

        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..total_weight);

        for (mob_kind, weight) in &self.mob_table {
            roll -= weight;
            if roll < 0 {
                return Ok(game_state().spawn_mob(*mob_kind));
            }
        }
        Err(DungeonError::MobSpawnError)
    }

    /// Move the player in the given direction
    pub fn move_player(&mut self, direction: Direction) -> Result<(), DungeonError> {
        let (dx, dy) = direction.offset();
        let new_x = self.player_position.0 + dx;
        let new_y = self.player_position.1 + dy;

        // Check if there's a room at the new position
        if self.get_room(new_x, new_y).is_some() {
            self.player_position = (new_x, new_y);
            // Mark the new room as visited and reveal adjacent rooms
            if let Some(room) = self.get_room_mut(new_x, new_y) {
                room.visit();
            }
            self.reveal_adjacent_rooms(new_x, new_y);
            Ok(())
        } else {
            Err(DungeonError::NoRoomAtPosition)
        }
    }

    /// Reveal all rooms adjacent to the given position
    pub fn reveal_adjacent_rooms(&mut self, x: i32, y: i32) {
        let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        for (dx, dy) in offsets {
            if let Some(room) = self.get_room_mut(x + dx, y + dy) {
                room.reveal();
            }
        }
    }

    /// Mark the starting room as visited (call after generation)
    pub fn mark_start_visited(&mut self) {
        let (x, y) = self.player_position;
        if let Some(room) = self.get_room_mut(x, y) {
            room.visit();
        }
    }

    /// Get available directions the player can move
    pub fn available_directions(&self) -> Vec<Direction> {
        let (x, y) = self.player_position;
        Direction::all()
            .into_iter()
            .filter(|dir| {
                let (dx, dy) = dir.offset();
                self.get_room(x + dx, y + dy).is_some()
            })
            .collect()
    }

    /// Check if all rooms in the dungeon are cleared
    pub fn is_completed(&self) -> bool {
        for row in &self.rooms {
            for room_opt in row {
                if let Some(room) = room_opt {
                    if !room.is_cleared {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Count total rooms in the dungeon
    pub fn room_count(&self) -> usize {
        self.rooms
            .iter()
            .flat_map(|row| row.iter())
            .filter(|r| r.is_some())
            .count()
    }

    /// Count cleared rooms in the dungeon
    pub fn cleared_count(&self) -> usize {
        self.rooms
            .iter()
            .flat_map(|row| row.iter())
            .filter_map(|r| r.as_ref())
            .filter(|r| r.is_cleared)
            .count()
    }
}

#[derive(Debug, Clone)]
pub struct DungeonRoom {
    pub room_type: RoomType,
    pub is_cleared: bool,
    pub is_visited: bool,
    pub is_revealed: bool,
    pub x: i32,
    pub y: i32,
    pub chest: Option<Chest>,
}

impl DungeonRoom {
    /// Create a new dungeon room
    pub fn new(room_type: RoomType, x: i32, y: i32) -> Self {
        let chest = if room_type == RoomType::Chest {
            Some(Chest::default())
        } else {
            None
        };

        // Rest rooms are always cleared (player can leave and return to heal)
        let is_cleared = room_type == RoomType::Rest;

        Self {
            room_type,
            is_cleared,
            is_visited: false,
            is_revealed: false,
            x,
            y,
            chest,
        }
    }

    /// Mark the room as visited (also reveals it)
    pub fn visit(&mut self) {
        self.is_visited = true;
        self.is_revealed = true;
    }

    /// Mark the room as revealed (visible on map but not visited)
    pub fn reveal(&mut self) {
        self.is_revealed = true;
    }

    /// Mark the room as cleared
    pub fn clear(&mut self) {
        self.is_cleared = true;
    }

    /// Open the chest and get loot drops (only for chest rooms)
    pub fn open_chest(&mut self) -> Vec<LootDrop> {
        if let Some(chest) = self.chest.take() {
            chest.roll_drops()
        } else {
            Vec::new()
        }
    }
}
