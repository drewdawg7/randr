use crate::location::{Location, LocationId};

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
}
