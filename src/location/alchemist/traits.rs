use std::time::Duration;

use crate::{
    player::Player,
    location::{Location, LocationEntryError, LocationId},
};

use super::definition::Alchemist;

impl Default for Alchemist {
    fn default() -> Self {
        Self {
            location_id: LocationId::VillageAlchemist,
            name: "Alchemist".to_string(),
            description: String::new(),
        }
    }
}

impl Location for Alchemist {
    fn id(&self) -> LocationId {
        self.location_id()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        Alchemist::description(self)
    }

    fn tick(&mut self, _elapsed: Duration) {
        // No time-based updates for alchemist
    }

    fn refresh(&mut self) {
        // No refresh mechanic for alchemist
    }

    fn time_until_refresh(&self) -> Option<Duration> {
        None
    }

    fn can_enter(&self, _player: &Player) -> Result<(), LocationEntryError> {
        Ok(())
    }

    fn on_enter(&mut self, _player: &mut Player) {
        // No special action on enter
    }

    fn on_exit(&mut self, _player: &mut Player) {
        // No special action on exit
    }
}
