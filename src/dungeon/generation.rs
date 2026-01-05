use rand::seq::SliceRandom;
use rand::Rng;

use crate::dungeon::{
    definition::{Dungeon, DungeonRoom, DUNGEON_SIZE, MAX_FILL_PERCENT},
    enums::RoomType,
    traits::Explorable,
};

/// Minimum number of rooms to generate
const MIN_ROOMS: usize = 5;

impl Dungeon {
    /// Generate a new dungeon layout with corridor-like structure
    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();

        // Calculate room count (between MIN_ROOMS and max fill)
        let max_rooms = ((DUNGEON_SIZE * DUNGEON_SIZE) as f32 * MAX_FILL_PERCENT) as usize;
        let target_rooms = rng.gen_range(MIN_ROOMS..=max_rooms);

        // Reset rooms grid
        self.rooms = vec![vec![None; DUNGEON_SIZE]; DUNGEON_SIZE];

        // Start with entry room at a random edge position
        let start_pos = self.random_edge_position(&mut rng);
        self.player_position = start_pos;

        // Track which positions have rooms
        let mut room_positions: Vec<(i32, i32)> = vec![start_pos];

        // Add entry room (always a monster room for first pass)
        let mut entry_room = DungeonRoom::new(RoomType::Monster, start_pos.0, start_pos.1);
        entry_room.visit(); // Mark entry room as visited
        self.rooms[start_pos.1 as usize][start_pos.0 as usize] = Some(entry_room);

        // Direction indices: 0=North, 1=East, 2=South, 3=West
        let directions: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        // Track current position and direction for corridor generation
        let mut current_pos = start_pos;
        let mut current_dir: usize = rng.gen_range(0..4);
        let mut stuck_count = 0;

        // Generate remaining rooms using corridor-favoring random walk
        while room_positions.len() < target_rooms && stuck_count < 50 {
            // 70% chance to continue in same direction, 30% to turn
            let try_same_dir = rng.gen_ratio(7, 10);

            let dir_order: Vec<usize> = if try_same_dir {
                // Favor current direction, then perpendicular, then reverse
                let perpendicular = [(current_dir + 1) % 4, (current_dir + 3) % 4];
                let reverse = (current_dir + 2) % 4;
                let mut order = vec![current_dir];
                order.extend(perpendicular.iter().cloned());
                order.push(reverse);
                order
            } else {
                // Random order but still avoid reverse if possible
                let mut order: Vec<usize> = (0..4).collect();
                order.shuffle(&mut rng);
                order
            };

            let mut found = false;
            for &dir_idx in &dir_order {
                let (dx, dy) = directions[dir_idx];
                let new_x = current_pos.0 + dx;
                let new_y = current_pos.1 + dy;

                // Check bounds
                if new_x < 0
                    || new_x >= DUNGEON_SIZE as i32
                    || new_y < 0
                    || new_y >= DUNGEON_SIZE as i32
                {
                    continue;
                }

                // Check if position is empty
                if self.rooms[new_y as usize][new_x as usize].is_some() {
                    continue;
                }

                // Create a new room at this position
                let room_type = self.random_room_type(&mut rng);
                let room = DungeonRoom::new(room_type, new_x, new_y);
                self.rooms[new_y as usize][new_x as usize] = Some(room);
                room_positions.push((new_x, new_y));

                current_pos = (new_x, new_y);
                current_dir = dir_idx;
                found = true;
                stuck_count = 0;
                break;
            }

            // If stuck, jump to a random existing room and try a new direction
            if !found {
                stuck_count += 1;
                current_pos = room_positions
                    .choose(&mut rng)
                    .copied()
                    .unwrap_or(start_pos); // fallback to start (should never happen)
                current_dir = rng.gen_range(0..4);
            }
        }

        // Guarantee at least 1 Chest and 1 Rest room
        self.ensure_room_type(&mut rng, &room_positions, RoomType::Chest, start_pos);
        self.ensure_room_type(&mut rng, &room_positions, RoomType::Rest, start_pos);

        // Place exactly 1 Boss room at a dead end (revealed on minimap)
        self.place_boss_room(&mut rng, &room_positions, start_pos);

        // Reveal rooms adjacent to the starting position
        self.reveal_adjacent_rooms(start_pos.0, start_pos.1);

        self.is_generated = true;
    }

    /// Ensure at least one room of a given type exists, converting a Monster room if needed
    fn ensure_room_type(
        &mut self,
        rng: &mut impl Rng,
        room_positions: &[(i32, i32)],
        room_type: RoomType,
        start_pos: (i32, i32),
    ) {
        // Check if room type already exists
        let has_type = room_positions.iter().any(|&(x, y)| {
            self.rooms[y as usize][x as usize]
                .as_ref()
                .map(|r| r.room_type == room_type)
                .unwrap_or(false)
        });

        if has_type {
            return;
        }

        // Find Monster rooms (excluding entry room) to convert
        let monster_rooms: Vec<(i32, i32)> = room_positions
            .iter()
            .filter(|&&pos| pos != start_pos)
            .filter(|&&(x, y)| {
                self.rooms[y as usize][x as usize]
                    .as_ref()
                    .map(|r| r.room_type == RoomType::Monster)
                    .unwrap_or(false)
            })
            .copied()
            .collect();

        // Convert a random Monster room to the required type
        if let Some(&(x, y)) = monster_rooms.choose(rng) {
            if let Some(room) = &mut self.rooms[y as usize][x as usize] {
                room.room_type = room_type;
                // Rest rooms should be pre-cleared
                if room_type == RoomType::Rest {
                    room.is_cleared = true;
                }
            }
        }
    }

    /// Get a random position on the edge of the grid
    fn random_edge_position(&self, rng: &mut impl Rng) -> (i32, i32) {
        let size = DUNGEON_SIZE as i32;
        let edge = rng.gen_range(0..4);
        match edge {
            0 => (rng.gen_range(0..size), 0),            // Top edge
            1 => (size - 1, rng.gen_range(0..size)),     // Right edge
            2 => (rng.gen_range(0..size), size - 1),     // Bottom edge
            _ => (0, rng.gen_range(0..size)),            // Left edge
        }
    }

    /// Get a random room type (60% Monster, 25% Chest, 15% Rest)
    fn random_room_type(&self, rng: &mut impl Rng) -> RoomType {
        let roll = rng.gen_range(0..100);
        if roll < 60 {
            RoomType::Monster
        } else if roll < 85 {
            RoomType::Chest
        } else {
            RoomType::Rest
        }
    }

    /// Place exactly 1 Boss room at a dead end (room with only 1 adjacent room)
    fn place_boss_room(
        &mut self,
        rng: &mut impl Rng,
        room_positions: &[(i32, i32)],
        start_pos: (i32, i32),
    ) {
        let directions: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        // Find all dead ends (rooms with exactly 1 neighbor), excluding start room
        let dead_ends: Vec<(i32, i32)> = room_positions
            .iter()
            .filter(|&&pos| pos != start_pos)
            .filter(|&&(x, y)| {
                let neighbor_count = directions
                    .iter()
                    .filter(|&&(dx, dy)| self.get_room(x + dx, y + dy).is_some())
                    .count();
                neighbor_count == 1
            })
            .copied()
            .collect();

        // Pick a random dead end and convert it to a Boss room
        if let Some(&(x, y)) = dead_ends.choose(rng) {
            if let Some(room) = &mut self.rooms[y as usize][x as usize] {
                room.room_type = RoomType::Boss;
            }
        } else {
            // Fallback: if no dead ends, pick any non-entry Monster room
            let monster_rooms: Vec<(i32, i32)> = room_positions
                .iter()
                .filter(|&&pos| pos != start_pos)
                .filter(|&&(x, y)| {
                    self.rooms[y as usize][x as usize]
                        .as_ref()
                        .map(|r| r.room_type == RoomType::Monster)
                        .unwrap_or(false)
                })
                .copied()
                .collect();

            if let Some(&(x, y)) = monster_rooms.choose(rng) {
                if let Some(room) = &mut self.rooms[y as usize][x as usize] {
                    room.room_type = RoomType::Boss;
                }
            }
        }
    }

    /// Reset the dungeon to an ungenerated state
    pub fn reset(&mut self) {
        self.rooms = vec![vec![None; DUNGEON_SIZE]; DUNGEON_SIZE];
        self.player_position = (0, 0);
        self.is_generated = false;
    }
}
