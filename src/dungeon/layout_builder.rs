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

use super::layout::DungeonLayout;
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

    /// Consumes the builder and produces the final `DungeonLayout`.
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

                // Apply floor variant pattern for visual variety
                let variant = if tile_type == TileType::Floor {
                    ((x + y) % 3) as u8
                } else {
                    0
                };

                layout.set_tile(x, y, Tile::new(tile_type).with_variant(variant));
            }
        }

        // Set entrance (PlayerSpawn)
        let (ex, ey) = entrance;
        let variant = ((ex + ey) % 3) as u8;
        layout.set_tile(ex, ey, Tile::new(TileType::PlayerSpawn).with_variant(variant));
        layout.entrance = entrance;

        // Set exit if provided
        if let Some((x, y)) = self.exit {
            layout.set_tile(x, y, Tile::new(TileType::Exit));
            layout.exit = Some((x, y));
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
