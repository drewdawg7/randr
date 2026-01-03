use std::collections::HashMap;
use std::time::Duration;

use crate::{
    entities::Player,
    location::{Location, LocationEntryError, LocationId},
};

use super::definition::Mine;
use super::rock::RockId;

impl Default for Mine {
    fn default() -> Self {
        let mut rock_weights = HashMap::new();
        rock_weights.insert(RockId::Tin, 2);
        rock_weights.insert(RockId::Coal, 2);
        rock_weights.insert(RockId::Copper, 2);
        rock_weights.insert(RockId::Mixed, 1);
        Self {
            location_id: LocationId::VillageMine,
            name: "Village Mine".to_string(),
            description: String::new(),
            rock_weights,
            current_rock: None,
        }
    }
}

impl Location for Mine {
    fn id(&self) -> LocationId {
        self.location_id()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        Mine::description(self)
    }

    fn tick(&mut self, _elapsed: Duration) {
        // No time-based updates for mine
    }

    fn refresh(&mut self) {
        // No refresh mechanic for mine
    }

    fn time_until_refresh(&self) -> Option<Duration> {
        None
    }

    fn can_enter(&self, _player: &Player) -> Result<(), LocationEntryError> {
        Ok(())
    }

    fn on_enter(&mut self, player: &mut Player) {
        self.ensure_rock_exists(player);
    }

    fn on_exit(&mut self, _player: &mut Player) {
        // No special action on exit
    }
}
