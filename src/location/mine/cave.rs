use rand::Rng;
use std::collections::HashSet;

use crate::loot::LootTable;
use super::rock::RockId;

pub const CAVE_WIDTH: usize = 60;
pub const CAVE_HEIGHT: usize = 20;
pub const MAX_ROCKS: usize = 8;

/// Rock types with spawn weights (Iron: 50, Coal: 30, Gold: 20)
#[derive(Clone, Copy, Debug)]
pub enum RockType {
    Iron,
    Coal,
    Gold,
}

impl RockType {
    /// Get the loot table for this rock type
    pub fn loot_table(&self) -> LootTable {
        match self {
            RockType::Iron => RockId::Iron.spec().loot.clone(),
            RockType::Coal => RockId::Coal.spec().loot.clone(),
            RockType::Gold => RockId::Gold.spec().loot.clone(),
        }
    }

    /// Randomly select a rock type based on spawn weights
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..100);
        if roll < 50 {
            RockType::Iron
        } else if roll < 80 {
            RockType::Coal
        } else {
            RockType::Gold
        }
    }
}

/// A rock placed in the cave
#[derive(Clone, Copy, Debug)]
pub struct CaveRock {
    pub x: usize,
    pub y: usize,
    pub rock_type: RockType,
}

/// Cell types in the cave
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Wall,
    Floor,
}

/// Generated cave layout - persisted across screen transitions
pub struct CaveLayout {
    cells: [[Cell; CAVE_WIDTH]; CAVE_HEIGHT],
    pub rocks: Vec<CaveRock>,
    pub player_x: usize,
    pub player_y: usize,
    pub exit_x: usize,
    pub exit_y: usize,
}

impl CaveLayout {
    /// Generate a new procedural cave using cellular automata
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = [[Cell::Wall; CAVE_WIDTH]; CAVE_HEIGHT];

        // Step 1: Seed the center with guaranteed floor (large ellipse)
        let center_x = CAVE_WIDTH / 2;
        let center_y = CAVE_HEIGHT / 2;
        let radius_x = CAVE_WIDTH / 3;
        let radius_y = CAVE_HEIGHT / 3;

        for y in 3..CAVE_HEIGHT - 3 {
            for x in 3..CAVE_WIDTH - 3 {
                let dx = (x as f32 - center_x as f32) / radius_x as f32;
                let dy = (y as f32 - center_y as f32) / radius_y as f32;
                let dist = dx * dx + dy * dy;

                if dist < 0.7 {
                    cells[y][x] = Cell::Floor;
                } else if dist < 1.2 && rng.gen_bool(0.65) {
                    cells[y][x] = Cell::Floor;
                }
            }
        }

        // Step 2: Run cellular automata iterations to smooth edges
        for _ in 0..4 {
            cells = Self::automata_step(&cells);
        }

        // Step 3: Ensure edges are always walls (3 cells thick)
        for y in 0..CAVE_HEIGHT {
            for x in 0..CAVE_WIDTH {
                if x < 3 || x >= CAVE_WIDTH - 3 || y < 3 || y >= CAVE_HEIGHT - 3 {
                    cells[y][x] = Cell::Wall;
                }
            }
        }

        // Step 4: Keep only the largest connected floor region
        cells = Self::keep_largest_region(cells);

        // Step 5: Find exit position (as centered as possible)
        let (exit_x, exit_y) = Self::find_center_floor(&cells);

        // Step 6: Place 6-8 rocks randomly on floor tiles (avoiding exit)
        let rocks = Self::place_rocks(&cells, exit_x, exit_y);

        // Step 7: Find player spawn point (center of floor area, avoiding exit and rocks)
        let (player_x, player_y) = Self::find_spawn_point(&cells, &rocks, exit_x, exit_y);

        Self {
            cells,
            rocks,
            player_x,
            player_y,
            exit_x,
            exit_y,
        }
    }

    /// Get the cell at the given position
    pub fn cell_at(&self, x: usize, y: usize) -> Cell {
        self.cells[y][x]
    }

    /// Check if a cell is a floor
    pub fn is_floor(&self, x: usize, y: usize) -> bool {
        x < CAVE_WIDTH && y < CAVE_HEIGHT && self.cells[y][x] == Cell::Floor
    }

    /// Get all floor positions (for rock spawning)
    pub fn floor_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for y in 0..CAVE_HEIGHT {
            for x in 0..CAVE_WIDTH {
                if self.cells[y][x] == Cell::Floor {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

    /// Spawn a rock at a random valid floor position
    /// Returns true if a rock was spawned, false if no valid position found or max rocks reached
    pub fn spawn_rock(&mut self) -> bool {
        if self.rocks.len() >= MAX_ROCKS {
            return false;
        }

        // Collect occupied positions
        let occupied: HashSet<(usize, usize)> = self
            .rocks
            .iter()
            .map(|r| (r.x, r.y))
            .chain(std::iter::once((self.player_x, self.player_y)))
            .chain(std::iter::once((self.exit_x, self.exit_y)))
            .collect();

        // Find valid floor positions
        let valid_positions: Vec<(usize, usize)> = self
            .floor_positions()
            .into_iter()
            .filter(|pos| !occupied.contains(pos))
            .collect();

        if valid_positions.is_empty() {
            return false;
        }

        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..valid_positions.len());
        let (x, y) = valid_positions[idx];

        self.rocks.push(CaveRock {
            x,
            y,
            rock_type: RockType::random(),
        });

        true
    }

    /// Find the most centered floor tile
    fn find_center_floor(cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT]) -> (usize, usize) {
        let center_x = CAVE_WIDTH / 2;
        let center_y = CAVE_HEIGHT / 2;

        for radius in 0..CAVE_WIDTH.max(CAVE_HEIGHT) {
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let x = center_x as i32 + dx;
                    let y = center_y as i32 + dy;

                    if x >= 0
                        && y >= 0
                        && (x as usize) < CAVE_WIDTH
                        && (y as usize) < CAVE_HEIGHT
                        && cells[y as usize][x as usize] == Cell::Floor
                    {
                        return (x as usize, y as usize);
                    }
                }
            }
        }

        (center_x, center_y)
    }

    /// Find a spawn point for the player near the center (avoiding exit and rocks)
    fn find_spawn_point(
        cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT],
        rocks: &[CaveRock],
        exit_x: usize,
        exit_y: usize,
    ) -> (usize, usize) {
        let center_x = CAVE_WIDTH / 2;
        let center_y = CAVE_HEIGHT / 2;

        for radius in 1..CAVE_WIDTH.max(CAVE_HEIGHT) {
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let x = center_x as i32 + dx;
                    let y = center_y as i32 + dy;

                    if x >= 0 && y >= 0 && (x as usize) < CAVE_WIDTH && (y as usize) < CAVE_HEIGHT {
                        let (x, y) = (x as usize, y as usize);
                        if cells[y][x] == Cell::Floor {
                            if x == exit_x && y == exit_y {
                                continue;
                            }
                            let has_rock = rocks.iter().any(|r| r.x == x && r.y == y);
                            if !has_rock {
                                return (x, y);
                            }
                        }
                    }
                }
            }
        }

        (center_x, center_y)
    }

    /// Try to move the player in a direction. Returns true if moved.
    pub fn move_player(&mut self, dx: i32, dy: i32) -> bool {
        let new_x = self.player_x as i32 + dx;
        let new_y = self.player_y as i32 + dy;

        if new_x < 0 || new_y < 0 || new_x as usize >= CAVE_WIDTH || new_y as usize >= CAVE_HEIGHT {
            return false;
        }

        let (new_x, new_y) = (new_x as usize, new_y as usize);

        if self.cells[new_y][new_x] == Cell::Wall {
            return false;
        }

        let has_rock = self.rocks.iter().any(|r| r.x == new_x && r.y == new_y);
        if has_rock {
            return false;
        }

        self.player_x = new_x;
        self.player_y = new_y;
        true
    }

    /// Check if player is adjacent to any rock. Returns the index if so.
    pub fn adjacent_rock_index(&self) -> Option<usize> {
        for (i, rock) in self.rocks.iter().enumerate() {
            let dx = (rock.x as i32 - self.player_x as i32).abs();
            let dy = (rock.y as i32 - self.player_y as i32).abs();
            if dx <= 1 && dy <= 1 && !(dx == 0 && dy == 0) {
                return Some(i);
            }
        }
        None
    }

    /// Check if player is adjacent to any rock
    pub fn is_adjacent_to_rock(&self) -> bool {
        self.adjacent_rock_index().is_some()
    }

    /// Mine the adjacent rock, removing it and returning its type
    pub fn mine_adjacent_rock(&mut self) -> Option<RockType> {
        if let Some(idx) = self.adjacent_rock_index() {
            let rock = self.rocks.remove(idx);
            Some(rock.rock_type)
        } else {
            None
        }
    }

    /// Check if player is standing on the exit ladder
    pub fn is_on_exit(&self) -> bool {
        self.player_x == self.exit_x && self.player_y == self.exit_y
    }

    /// Place 6-8 rocks uniformly spread across the cave floor (avoiding exit)
    fn place_rocks(
        cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT],
        exit_x: usize,
        exit_y: usize,
    ) -> Vec<CaveRock> {
        let mut rng = rand::thread_rng();
        let rock_count = rng.gen_range(6..=8);

        let zone_width = CAVE_WIDTH / 3;
        let zone_height = CAVE_HEIGHT / 3;

        let mut zones: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 9];
        for y in 0..CAVE_HEIGHT {
            for x in 0..CAVE_WIDTH {
                if cells[y][x] == Cell::Floor && !(x == exit_x && y == exit_y) {
                    let zone_x = (x / zone_width).min(2);
                    let zone_y = (y / zone_height).min(2);
                    let zone_idx = zone_y * 3 + zone_x;
                    zones[zone_idx].push((x, y));
                }
            }
        }

        let mut available_zones: Vec<usize> = zones
            .iter()
            .enumerate()
            .filter(|(_, z)| !z.is_empty())
            .map(|(i, _)| i)
            .collect();

        let mut rocks = Vec::new();
        let mut zone_idx = 0;

        for _ in 0..rock_count {
            if available_zones.is_empty() {
                break;
            }

            let zone = available_zones[zone_idx % available_zones.len()];
            zone_idx += 1;

            if !zones[zone].is_empty() {
                let pos_idx = rng.gen_range(0..zones[zone].len());
                let (x, y) = zones[zone].remove(pos_idx);
                rocks.push(CaveRock {
                    x,
                    y,
                    rock_type: RockType::random(),
                });
            }

            available_zones.retain(|&z| !zones[z].is_empty());
        }

        rocks
    }

    /// Find all connected floor regions and keep only the largest one
    fn keep_largest_region(
        mut cells: [[Cell; CAVE_WIDTH]; CAVE_HEIGHT],
    ) -> [[Cell; CAVE_WIDTH]; CAVE_HEIGHT] {
        let mut visited = [[false; CAVE_WIDTH]; CAVE_HEIGHT];
        let mut regions: Vec<Vec<(usize, usize)>> = Vec::new();

        for y in 0..CAVE_HEIGHT {
            for x in 0..CAVE_WIDTH {
                if cells[y][x] == Cell::Floor && !visited[y][x] {
                    let region = Self::flood_fill(&cells, &mut visited, x, y);
                    regions.push(region);
                }
            }
        }

        if let Some(largest) = regions.iter().max_by_key(|r| r.len()) {
            let largest_set: HashSet<(usize, usize)> = largest.iter().copied().collect();

            for y in 0..CAVE_HEIGHT {
                for x in 0..CAVE_WIDTH {
                    if cells[y][x] == Cell::Floor && !largest_set.contains(&(x, y)) {
                        cells[y][x] = Cell::Wall;
                    }
                }
            }
        }

        cells
    }

    /// Flood fill to find all connected floor cells starting from (start_x, start_y)
    fn flood_fill(
        cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT],
        visited: &mut [[bool; CAVE_WIDTH]; CAVE_HEIGHT],
        start_x: usize,
        start_y: usize,
    ) -> Vec<(usize, usize)> {
        let mut region = Vec::new();
        let mut stack = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            if visited[y][x] {
                continue;
            }
            if cells[y][x] != Cell::Floor {
                continue;
            }

            visited[y][x] = true;
            region.push((x, y));

            if x > 0 {
                stack.push((x - 1, y));
            }
            if x < CAVE_WIDTH - 1 {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y < CAVE_HEIGHT - 1 {
                stack.push((x, y + 1));
            }
        }

        region
    }

    /// Single cellular automata step
    fn automata_step(
        cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT],
    ) -> [[Cell; CAVE_WIDTH]; CAVE_HEIGHT] {
        let mut new_cells = *cells;

        for y in 1..CAVE_HEIGHT - 1 {
            for x in 1..CAVE_WIDTH - 1 {
                let wall_count = Self::count_wall_neighbors(cells, x, y);

                if wall_count >= 5 {
                    new_cells[y][x] = Cell::Wall;
                } else if wall_count < 4 {
                    new_cells[y][x] = Cell::Floor;
                }
            }
        }

        new_cells
    }

    /// Count wall neighbors in 3x3 area around cell
    fn count_wall_neighbors(cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT], x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                // Treat out-of-bounds as walls
                if nx < 0
                    || ny < 0
                    || nx as usize >= CAVE_WIDTH
                    || ny as usize >= CAVE_HEIGHT
                    || cells[ny as usize][nx as usize] == Cell::Wall
                {
                    count += 1;
                }
            }
        }
        count
    }

    /// Get the number of rocks currently in the cave
    pub fn rock_count(&self) -> usize {
        self.rocks.len()
    }
}
