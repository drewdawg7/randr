use bevy::prelude::*;
use rand::Rng;

use crate::rock::RockId;

/// Different types of ore that can be mined.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OreType {
    Copper,
    Tin,
    Coal,
    Mixed,
}

impl OreType {
    /// Get the RockId associated with this ore type.
    pub fn rock_id(&self) -> RockId {
        match self {
            OreType::Copper => RockId::Iron,
            OreType::Tin => RockId::Gold,
            OreType::Coal => RockId::Coal,
            OreType::Mixed => RockId::Mixed,
        }
    }

    /// Get the color for rendering this ore type.
    pub fn color(&self) -> Color {
        match self {
            OreType::Copper => Color::srgb(0.72, 0.45, 0.20), // Copper color
            OreType::Tin => Color::srgb(0.6, 0.6, 0.6),       // Silver-ish
            OreType::Coal => Color::srgb(0.2, 0.2, 0.2),      // Dark gray
            OreType::Mixed => Color::srgb(0.5, 0.4, 0.3),     // Mixed brown
        }
    }
}

/// Different tile types in the mine grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MineTile {
    /// Wall - impassable, appears at edges
    Wall,
    /// Floor - walkable empty space
    Floor,
    /// Rock - mineable, may drop ore
    Rock,
    /// Ore - mineable, guaranteed ore drop
    Ore(OreType),
    /// Ladder - exit tile, returns to Town
    Ladder,
}

impl MineTile {
    /// Get the color for rendering this tile.
    pub fn color(&self) -> Color {
        match self {
            MineTile::Wall => Color::srgb(0.3, 0.3, 0.35),
            MineTile::Floor => Color::srgb(0.15, 0.15, 0.15),
            MineTile::Rock => Color::srgb(0.4, 0.4, 0.4),
            MineTile::Ore(ore_type) => ore_type.color(),
            MineTile::Ladder => Color::srgb(0.6, 0.5, 0.2),
        }
    }

    /// Check if this tile is walkable.
    pub fn is_walkable(&self) -> bool {
        matches!(self, MineTile::Floor | MineTile::Ladder)
    }

    /// Check if this tile is mineable.
    pub fn is_mineable(&self) -> bool {
        matches!(self, MineTile::Rock | MineTile::Ore(_))
    }
}

/// 12x8 grid structure for the mine.
#[derive(Debug, Clone)]
pub(super) struct MineGrid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<MineTile>>,
}

impl MineGrid {
    /// Create a new mine grid with walls around the edges and randomized interior.
    pub fn new() -> Self {
        let width = 12;
        let height = 8;
        let mut rng = rand::thread_rng();

        let mut tiles = vec![vec![MineTile::Floor; width]; height];

        // Create walls around edges
        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    tiles[y][x] = MineTile::Wall;
                }
            }
        }

        // Add random rocks and ores in the interior
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let roll = rng.gen_range(0..100);
                tiles[y][x] = match roll {
                    0..=10 => {
                        // 11% chance of ore
                        let ore_roll = rng.gen_range(0..100);
                        let ore_type = match ore_roll {
                            0..=40 => OreType::Copper,
                            41..=70 => OreType::Tin,
                            71..=90 => OreType::Coal,
                            _ => OreType::Mixed,
                        };
                        MineTile::Ore(ore_type)
                    }
                    11..=40 => MineTile::Rock, // 30% chance of regular rock
                    _ => MineTile::Floor,      // 59% chance of floor
                };
            }
        }

        // Place ladder at bottom-right corner (but not in the wall)
        tiles[height - 2][width - 2] = MineTile::Ladder;

        Self {
            width,
            height,
            tiles,
        }
    }

    /// Get a tile at a specific position.
    pub fn get(&self, x: usize, y: usize) -> Option<MineTile> {
        if y < self.height && x < self.width {
            Some(self.tiles[y][x])
        } else {
            None
        }
    }

    /// Set a tile at a specific position.
    pub fn set(&mut self, x: usize, y: usize, tile: MineTile) {
        if y < self.height && x < self.width {
            self.tiles[y][x] = tile;
        }
    }

    /// Check if a position is walkable.
    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get(x, y).map(|t| t.is_walkable()).unwrap_or(false)
    }

    /// Check if a position is mineable.
    pub fn is_mineable(&self, x: usize, y: usize) -> bool {
        self.get(x, y).map(|t| t.is_mineable()).unwrap_or(false)
    }
}

impl Default for MineGrid {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource tracking the current state within the Mine screen.
#[derive(Resource, Default)]
pub struct MineScreenState {
    /// The mine grid
    pub grid: MineGrid,
    /// Player position in the grid (x, y)
    pub player_pos: (usize, usize),
    /// Optional mining message to display
    pub mining_message: Option<String>,
}

impl MineScreenState {
    /// Create a new mine screen state.
    pub fn new() -> Self {
        Self {
            grid: MineGrid::new(),
            player_pos: (1, 1), // Start at top-left interior
            mining_message: None,
        }
    }

    /// Move the player in a direction if the destination is walkable.
    pub fn move_player(&mut self, dx: i32, dy: i32) -> bool {
        let new_x = (self.player_pos.0 as i32 + dx).max(0) as usize;
        let new_y = (self.player_pos.1 as i32 + dy).max(0) as usize;

        if self.grid.is_walkable(new_x, new_y) {
            self.player_pos = (new_x, new_y);
            true
        } else {
            false
        }
    }

    /// Check if the player is standing on the ladder.
    pub fn is_on_ladder(&self) -> bool {
        matches!(
            self.grid.get(self.player_pos.0, self.player_pos.1),
            Some(MineTile::Ladder)
        )
    }

    /// Get adjacent mineable positions (up, down, left, right).
    pub fn get_adjacent_mineable(&self) -> Vec<(usize, usize)> {
        let (x, y) = self.player_pos;
        let mut mineable = Vec::new();

        // Check all four cardinal directions
        let adjacent = [
            (x.wrapping_sub(1), y), // Left
            (x + 1, y),             // Right
            (x, y.wrapping_sub(1)), // Up
            (x, y + 1),             // Down
        ];

        for (ax, ay) in adjacent {
            if self.grid.is_mineable(ax, ay) {
                mineable.push((ax, ay));
            }
        }

        mineable
    }

    /// Set a mining message.
    pub fn set_message(&mut self, message: String) {
        self.mining_message = Some(message);
    }

    /// Clear the mining message.
    pub fn clear_message(&mut self) {
        self.mining_message = None;
    }
}
