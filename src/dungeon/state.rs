//! DungeonState resource for tracking runtime dungeon state and player progression.

use std::collections::HashSet;

use bevy::prelude::*;

use crate::dungeon::{DungeonLayout, DungeonRegistry, FloorId, GridPosition, GridSize};
use crate::location::LocationId;

/// Resource tracking runtime dungeon state and player progression.
///
/// Combines progression tracking (which dungeon/floor the player is on, which floors
/// have been cleared) with runtime state (current layout and player position).
#[derive(Resource, Default)]
pub struct DungeonState {
    // === Progression tracking ===
    /// Currently active dungeon location (None if not in dungeon)
    pub current_location: Option<LocationId>,

    /// Currently active floor (None if not in dungeon)
    pub current_floor: Option<FloorId>,

    /// 0-indexed position in the location's floor sequence
    pub floor_index: usize,

    /// Set of floors the player has cleared
    pub cleared_floors: HashSet<FloorId>,

    // === Runtime state ===
    /// Current floor layout (None if not actively in dungeon)
    pub layout: Option<DungeonLayout>,

    /// Player's current position on the grid
    pub player_pos: GridPosition,

    /// Player's grid size (supports future multi-cell player)
    pub player_size: GridSize,
}

impl DungeonState {
    /// Begin a dungeon run at the first floor.
    ///
    /// Sets current location and floor, but does not generate the layout.
    /// Call `load_floor_layout` to generate the layout for the current floor.
    pub fn enter_dungeon(&mut self, location: LocationId, registry: &DungeonRegistry) {
        let floors = registry.floors(location);
        self.current_location = Some(location);
        self.current_floor = floors.first().copied();
        self.floor_index = 0;
    }

    /// Move to the next floor after clearing the current one.
    ///
    /// Returns the new floor ID if there is a next floor, or None if the dungeon is complete.
    pub fn advance_floor(&mut self, registry: &DungeonRegistry) -> Option<FloorId> {
        let location = self.current_location?;
        let floors = registry.floors(location);

        // Mark current floor as cleared
        if let Some(current) = self.current_floor {
            self.cleared_floors.insert(current);
        }

        // Move to next floor
        self.floor_index += 1;
        self.current_floor = floors.get(self.floor_index).copied();

        // Clear runtime state when advancing (new layout will be loaded)
        self.layout = None;

        self.current_floor
    }

    /// Check if the player is on the last floor of the current dungeon.
    pub fn is_current_floor_final(&self, registry: &DungeonRegistry) -> bool {
        let Some(location) = self.current_location else {
            return false;
        };
        let floors = registry.floors(location);
        self.floor_index == floors.len().saturating_sub(1)
    }

    /// Leave the dungeon (return to town, flee, etc.).
    ///
    /// Clears location, floor, and runtime state but preserves cleared_floors for progress tracking.
    pub fn exit_dungeon(&mut self) {
        self.current_location = None;
        self.current_floor = None;
        self.floor_index = 0;
        self.layout = None;
        self.player_pos = GridPosition::default();
        self.player_size = GridSize::default();
    }

    /// Check if a specific floor has been cleared.
    pub fn is_floor_cleared(&self, floor: FloorId) -> bool {
        self.cleared_floors.contains(&floor)
    }

    /// Load the layout for the current floor and set player spawn position.
    ///
    /// Returns the layout if successfully loaded, or None if not in a dungeon.
    pub fn load_floor_layout(&mut self) -> Option<&DungeonLayout> {
        let floor = self.current_floor?;
        let spec = floor.spec();
        let layout = spec.layout_id.layout();

        // Find player spawn position
        self.player_pos = layout
            .iter()
            .find(|(_, _, tile)| tile.tile_type == crate::dungeon::TileType::PlayerSpawn)
            .map_or(GridPosition::default(), |(x, y, _)| GridPosition::new(x, y));

        self.player_size = GridSize::single();

        self.layout = Some(layout);
        self.layout.as_ref()
    }

    /// Check if currently in a dungeon.
    pub fn is_in_dungeon(&self) -> bool {
        self.current_location.is_some()
    }
}
