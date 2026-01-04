use std::time::Duration;

use crate::{
    player::Player,
    location::{Location, LocationEntryError, LocationId},
};

use super::definition::Blacksmith;

impl Default for Blacksmith {
    fn default() -> Self {
        Self {
            location_id: LocationId::VillageBlacksmith,
            name: "Blacksmith".to_string(),
            description: String::new(),
            max_upgrades: 4,
            base_upgrade_cost: 5,
            fuel_amount: 0,
            last_fuel_regen: None,
        }
    }
}

impl Location for Blacksmith {
    fn id(&self) -> LocationId {
        self.location_id()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        Blacksmith::description(self)
    }

    fn tick(&mut self, _elapsed: Duration) {
        // No time-based updates for blacksmith
    }

    fn refresh(&mut self) {
        // No refresh mechanic for blacksmith
    }

    fn time_until_refresh(&self) -> Option<Duration> {
        None
    }

    fn can_enter(&self, _player: &Player) -> Result<(), LocationEntryError> {
        Ok(())
    }

    fn on_enter(&mut self, player: &mut Player) {
        // Apply fuel regeneration from passive effects
        self.apply_fuel_regen(player);
    }

    fn on_exit(&mut self, _player: &mut Player) {
        // No special action on exit
    }
}
