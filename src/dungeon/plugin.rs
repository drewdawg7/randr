//! DungeonPlugin for registering dungeon locations and their floors.
//!
//! Provides a fluent builder API for declaratively registering which floors
//! belong to which dungeon locations.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::dungeon::{DungeonState, FloorId};
use crate::location::LocationId;

/// Resource providing runtime access to dungeon floor configurations.
#[derive(Resource, Clone, Debug)]
pub struct DungeonRegistry {
    dungeons: HashMap<LocationId, Vec<FloorId>>,
}

impl DungeonRegistry {
    /// Get all floors for a location in order.
    pub fn floors(&self, location: LocationId) -> &[FloorId] {
        self.dungeons
            .get(&location)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get the next floor after the given floor (if any).
    pub fn next_floor(&self, location: LocationId, current: FloorId) -> Option<FloorId> {
        let floors = self.floors(location);
        floors
            .iter()
            .position(|&f| f == current)
            .and_then(|idx| floors.get(idx + 1))
            .copied()
    }

    /// Check if a floor is the final floor for its location.
    pub fn is_final_floor(&self, location: LocationId, floor: FloorId) -> bool {
        let floors = self.floors(location);
        floors.last() == Some(&floor)
    }
}

/// Plugin that provides declarative dungeon location/floor registration.
pub struct DungeonPlugin {
    registry: DungeonRegistry,
}

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.registry.clone())
            .init_resource::<DungeonState>();
    }
}

impl DungeonPlugin {
    /// Create a new dungeon builder.
    pub fn new() -> DungeonBuilder {
        DungeonBuilder {
            dungeons: HashMap::new(),
            current_location: None,
        }
    }
}

impl Default for DungeonPlugin {
    fn default() -> Self {
        Self {
            registry: DungeonRegistry {
                dungeons: HashMap::new(),
            },
        }
    }
}

/// Builder for configuring dungeon locations and their floors.
pub struct DungeonBuilder {
    dungeons: HashMap<LocationId, Vec<FloorId>>,
    current_location: Option<LocationId>,
}

impl DungeonBuilder {
    /// Set the current location context for subsequent `.floor()` calls.
    ///
    /// All floors added after this call (until the next `.location()`) are
    /// registered under this location.
    pub fn location(mut self, id: LocationId) -> Self {
        self.current_location = Some(id);
        // Ensure the location has an entry in the map
        self.dungeons.entry(id).or_default();
        self
    }

    /// Add a floor to the current location.
    ///
    /// Floors are added in sequence order (first floor added = floor 1, etc.).
    ///
    /// # Panics
    ///
    /// Panics if called before any `.location()` call.
    pub fn floor(mut self, floor: FloorId) -> Self {
        let location = self
            .current_location
            .expect("floor() called before location() - no location context set");
        self.dungeons.get_mut(&location).unwrap().push(floor);
        self
    }

    /// Build the dungeon plugin.
    ///
    /// # Panics
    ///
    /// Panics if no locations were registered.
    pub fn build(self) -> DungeonPlugin {
        assert!(
            !self.dungeons.is_empty(),
            "DungeonPlugin requires at least one location to be registered"
        );

        DungeonPlugin {
            registry: DungeonRegistry {
                dungeons: self.dungeons,
            },
        }
    }
}
