use std::time::Duration;

use super::enums::{LocationId, LocationType};

/// Core trait that ALL locations implement
pub trait Location {
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
}

/// Trait for locations that have time-based refresh mechanics.
/// Only locations with actual refresh behavior should implement this.
pub trait Refreshable: Location {
    /// Called each game tick with elapsed time
    fn tick(&mut self, elapsed: Duration);

    /// Force an immediate refresh/restock
    fn refresh(&mut self);

    /// Time until next automatic refresh
    fn time_until_refresh(&self) -> Duration;
}
