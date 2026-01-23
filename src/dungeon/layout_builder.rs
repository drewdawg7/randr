//! Declarative builder for dungeon layouts.
//!
//! Provides a builder-pattern API for defining dungeon layouts, following
//! the pattern established by `Column`/`Row` widgets.
//!
//! # Example
//!
//! ```ignore
//! let layout = LayoutBuilder::new(40, 21)
//!     .entrance(20, 19)
//!     .exit(20, 20)
//!     .build();
//! ```

use std::ops::RangeInclusive;

use rand::seq::SliceRandom;
use rand::Rng;

use super::layout::DungeonLayout;
use super::spawn::SpawnTable;
use super::tile::{Tile, TileType};

/// A builder for creating dungeon layouts declaratively.
///
/// The layout is always a rectangular grid filled with `TileType::Floor` tiles,
/// surrounded by a 1-tile `TileType::Wall` border.
pub struct LayoutBuilder {
    width: usize,
    height: usize,
    entrance: Option<(usize, usize)>,
    exit: Option<(usize, usize)>,
    door: Option<(usize, usize)>,
    spawn_table: Option<SpawnTable>,
    torch_count: Option<RangeInclusive<u8>>,
}

impl LayoutBuilder {
    /// Creates a new layout builder with the specified grid dimensions.
    ///
    /// The resulting layout will be:
    /// - `width` x `height` tiles total
    /// - Interior filled with `TileType::Floor`
    /// - 1-tile `TileType::Wall` border around the edges
    ///
    /// # Example
    ///
    /// ```ignore
    /// LayoutBuilder::new(40, 21)  // 40x21 grid, 38x19 walkable interior
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            entrance: None,
            exit: None,
            door: None,
            spawn_table: None,
            torch_count: None,
        }
    }

    /// Sets the player spawn point (entrance) for the layout.
    ///
    /// This tile becomes `TileType::PlayerSpawn`.
    ///
    /// # Panics
    ///
    /// Panics if the position is not in the interior (must not be on any wall).
    ///
    /// # Example
    ///
    /// ```ignore
    /// .entrance(20, 19)  // Interior position, one tile above bottom wall
    /// ```
    pub fn entrance(mut self, x: usize, y: usize) -> Self {
        self.validate_interior_position(x, y, "entrance");
        self.entrance = Some((x, y));
        self
    }

    /// Sets the exit point (stairs/door to next floor) for the layout.
    ///
    /// This tile becomes `TileType::Exit`.
    ///
    /// # Panics
    ///
    /// Panics if the position is not on the top or bottom wall.
    ///
    /// # Example
    ///
    /// ```ignore
    /// .exit(20, 20)  // Bottom wall exit
    /// .exit(20, 0)   // Top wall exit
    /// ```
    pub fn exit(mut self, x: usize, y: usize) -> Self {
        self.validate_wall_position(x, y, "exit");
        self.exit = Some((x, y));
        self
    }

    /// Sets a decorative door on the back (top) wall.
    ///
    /// This tile becomes `TileType::Door` (impassable).
    ///
    /// # Panics
    ///
    /// Panics if the position is not on the back wall (y=0).
    pub fn door(mut self, x: usize, y: usize) -> Self {
        assert!(y == 0, "door must be on back wall (y=0), got y={}", y);
        self.door = Some((x, y));
        self
    }

    /// Sets a spawn table to apply when building the layout.
    ///
    /// The spawn table defines what entities should spawn on valid spawn points.
    /// It is applied automatically during `build()`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// LayoutBuilder::new(30, 20)
    ///     .entrance(15, 18)
    ///     .exit(15, 19)
    ///     .spawn(SpawnTable::new()
    ///         .mob(MobId::Goblin, 3)
    ///         .mob(MobId::Slime, 2)
    ///         .chest(1..=2))
    ///     .build()
    /// ```
    pub fn spawn(mut self, spawn_table: SpawnTable) -> Self {
        self.spawn_table = Some(spawn_table);
        self
    }

    /// Sets the number of torch walls to place randomly on the back (top) wall.
    ///
    /// Torches are placed on the top wall (y=0), avoiding corners and the exit.
    ///
    /// # Example
    ///
    /// ```ignore
    /// .torches(2..=4)  // 2-4 torches on the back wall
    /// ```
    pub fn torches(mut self, count: RangeInclusive<u8>) -> Self {
        self.torch_count = Some(count);
        self
    }

    /// Consumes the builder and produces the final `DungeonLayout`.
    ///
    /// If a spawn table was set via `spawn()`, it is applied automatically
    /// using `rand::thread_rng()`.
    ///
    /// # Panics
    ///
    /// Panics if `entrance` was not set.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let layout = LayoutBuilder::new(40, 21)
    ///     .entrance(20, 19)
    ///     .exit(20, 20)
    ///     .build();
    /// ```
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

                // Randomize floor variant for visual variety (75% Slice_73, 25% others)
                let variant = if tile_type == TileType::Floor {
                    if rng.gen_range(0u8..4) < 3 {
                        0 // FloorTileAlt1 (Slice_73)
                    } else {
                        rng.gen_range(1u8..5) // Other floor tiles
                    }
                } else {
                    0
                };

                layout.set_tile(x, y, Tile::new(tile_type).with_variant(variant));
            }
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

            // Collect valid back-wall positions (not corners, not exit, not door)
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
