use std::collections::HashMap;
use std::time::Duration;

use crate::{
    entities::{mob::MobId, Player},
    location::{Location, LocationEntryError, LocationId},
};

use super::definition::Field;

impl Default for Field {
    fn default() -> Self {
        let mut mob_weights = HashMap::new();
        mob_weights.insert(MobId::Slime, 5);
        mob_weights.insert(MobId::Cow, 5);
        mob_weights.insert(MobId::Goblin, 3);
        mob_weights.insert(MobId::Dragon, 1);
        Self {
            location_id: LocationId::VillageField,
            name: "Village Field".to_string(),
            description: "Rolling fields outside the village where monsters roam".to_string(),
            mob_weights,
        }
    }
}

impl Location for Field {
    fn id(&self) -> LocationId {
        self.location_id()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        Field::description(self)
    }

    fn tick(&mut self, _elapsed: Duration) {
        // No time-based updates for field
    }

    fn refresh(&mut self) {
        // No refresh mechanic for field
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
