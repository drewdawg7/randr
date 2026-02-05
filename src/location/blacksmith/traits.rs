use crate::location::{Location, LocationId};

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
}
