use std::time::Duration;

use crate::player::Player;
use crate::item::ItemId;

use super::enums::{LocationId, LocationType};

/// Error returned when player cannot enter a location
#[derive(Debug)]
pub enum LocationEntryError {
    LevelTooLow { required: i32, current: i32 },
    MissingRequiredItem(ItemId),
    LocationClosed,
    Custom(String),
}

/// Core trait that ALL locations implement
pub trait Location {
    // === Identity ===

    /// Get the unique identifier for this location
    fn id(&self) -> LocationId;

    /// Get the display name of this location
    fn name(&self) -> &str;

    /// Get the description of this location
    fn description(&self) -> &str;

    /// Get the category type for this location
    fn location_type(&self) -> LocationType {
        self.id().location_type()
    }

    // === Entry/Exit Hooks ===

    /// Check if player can enter this location
    fn can_enter(&self, _player: &Player) -> Result<(), LocationEntryError> {
        Ok(()) // Default: always allowed
    }

    /// Called when player enters the location
    fn on_enter(&mut self, _player: &mut Player) {}

    /// Called when player exits the location
    fn on_exit(&mut self, _player: &mut Player) {}
}

/// Trait for locations that have time-based refresh mechanics.
/// Only locations with actual refresh behavior should implement this.
#[allow(dead_code)]
pub trait Refreshable: Location {
    /// Called each game tick with elapsed time
    fn tick(&mut self, elapsed: Duration);

    /// Force an immediate refresh/restock
    fn refresh(&mut self);

    /// Time until next automatic refresh
    fn time_until_refresh(&self) -> Duration;
}
