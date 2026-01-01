use std::time::Duration;

use crate::{
    entities::Player,
    location::{ActivityId, Location, LocationEntryError, LocationId},
};

use super::definition::Field;

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

    fn available_activities(&self) -> &[ActivityId] {
        &[ActivityId::Fight]
    }
}
