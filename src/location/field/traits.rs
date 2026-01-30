use std::collections::HashMap;

use crate::location::{Location, LocationId};
use crate::mob::MobId;

use super::definition::Field;

impl Default for Field {
    fn default() -> Self {
        let mut mob_weights = HashMap::new();
        mob_weights.insert(MobId::Slime, 5);
        mob_weights.insert(MobId::Cow, 5);
        mob_weights.insert(MobId::Goblin, 3);
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
}
