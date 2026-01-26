//! Declarative builder for dungeon layouts.

use std::ops::RangeInclusive;

use rand::seq::SliceRandom;
use rand::Rng;

use super::layout::DungeonLayout;
use super::room_patterns::{Rect, RoomPattern, RoomPatternKind};
use super::spawn::SpawnTable;
use super::tile::{Tile, TileType};

pub struct LayoutBuilder {
    width: usize,
    height: usize,
    entrance: Option<(usize, usize)>,
    exit: Option<(usize, usize)>,
    door: Option<(usize, usize)>,
    spawn_table: Option<SpawnTable>,
    torch_count: Option<RangeInclusive<u8>>,
    patterns: Vec<(Rect, RoomPatternKind)>,
}

impl LayoutBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            entrance: None,
            exit: None,
            door: None,
            spawn_table: None,
            torch_count: None,
            patterns: Vec::new(),
        }
    }

    pub fn entrance(mut self, x: usize, y: usize) -> Self {
        self.validate_interior_position(x, y, "entrance");
        self.entrance = Some((x, y));
        self
    }

    pub fn exit(mut self, x: usize, y: usize) -> Self {
        self.validate_wall_position(x, y, "exit");
        self.exit = Some((x, y));
        self
    }

    pub fn door(mut self, x: usize, y: usize) -> Self {
        assert!(y == 0, "door must be on back wall (y=0), got y={}", y);
        self.door = Some((x, y));
        self
    }

    pub fn spawn(mut self, spawn_table: SpawnTable) -> Self {
        self.spawn_table = Some(spawn_table);
        self
    }

    pub fn torches(mut self, count: RangeInclusive<u8>) -> Self {
        self.torch_count = Some(count);
        self
    }

    /// Apply a pattern to a rectangular region.
    pub fn pattern_at(mut self, bounds: Rect, pattern: RoomPatternKind) -> Self {
        self.patterns.push((bounds, pattern));
        self
    }

    /// Apply a pattern to the entire layout.
    pub fn pattern(self, pattern: RoomPatternKind) -> Self {
        let bounds = Rect::new(0, 0, self.width, self.height);
        self.pattern_at(bounds, pattern)
    }

    pub fn build(self) -> DungeonLayout {
        let entrance = self
            .entrance
            .expect("LayoutBuilder: entrance must be set before calling build()");

        let mut layout = DungeonLayout::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        // Fill grid with walls on perimeter, floor inside
        for y in 0..self.height {
            for x in 0..self.width {
                let is_perimeter =
                    y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1;

                let tile_type = if is_perimeter {
                    TileType::Wall
                } else {
                    TileType::Floor
                };

                let variant = if tile_type == TileType::Floor {
                    if rng.gen_range(0u8..4) < 3 { 0 } else { rng.gen_range(1u8..5) }
                } else {
                    0
                };

                layout.set_tile(x, y, Tile::new(tile_type).with_variant(variant));
            }
        }

        // Apply patterns in order
        for (bounds, pattern) in &self.patterns {
            pattern.apply(&mut layout, *bounds, &mut rng);
        }

        // Set entrance (PlayerSpawn)
        let (ex, ey) = entrance;
        let spawn_variant = if rng.gen_range(0u8..4) < 3 { 0 } else { rng.gen_range(1u8..5) };
        layout.set_tile(ex, ey, Tile::new(TileType::PlayerSpawn).with_variant(spawn_variant));
        layout.entrance = entrance;

        // Set exit if provided
        if let Some((x, y)) = self.exit {
            layout.set_tile(x, y, Tile::new(TileType::Exit));
            layout.exit = Some((x, y));
        }

        // Set door if provided
        if let Some((x, y)) = self.door {
            layout.set_tile(x, y, Tile::new(TileType::Door));
        }

        // Place torches on back wall if set
        if let Some(torch_range) = self.torch_count {
            let count = rng.gen_range(torch_range) as usize;

            let exit_x = self.exit.map(|(x, _)| x);
            let door_x = self.door.map(|(x, _)| x);
            let mut positions: Vec<usize> = (1..self.width - 1)
                .filter(|x| Some(*x) != exit_x && Some(*x) != door_x)
                .collect();
            positions.shuffle(&mut rng);

            for &x in positions.iter().take(count) {
                layout.set_tile(x, 0, Tile::new(TileType::TorchWall));
            }
        }

        // Apply spawn table if set
        if let Some(spawn_table) = self.spawn_table {
            spawn_table.apply(&mut layout, &mut rng);
        }

        layout
    }

    fn validate_interior_position(&self, x: usize, y: usize, name: &str) {
        let on_perimeter = y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1;
        if on_perimeter {
            panic!(
                "{} must be in the interior (not on walls), got ({}, {})",
                name, x, y
            );
        }
    }

    fn validate_wall_position(&self, _x: usize, y: usize, name: &str) {
        let on_top = y == 0;
        let on_bottom = y == self.height - 1;
        if !on_top && !on_bottom {
            panic!(
                "{} must be on top or bottom wall (y=0 or y={}), got y={}",
                name,
                self.height - 1,
                y
            );
        }
    }
}
