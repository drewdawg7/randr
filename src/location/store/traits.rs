use crate::item::ItemId;
use crate::location::{Location, LocationId};

use super::definition::Store;

impl Default for Store {
    fn default() -> Self {
        Store::new("The Shop", vec![
            (ItemId::Sword, 1),
            (ItemId::Dagger, 1),
            (ItemId::CopperPickaxe, 1),
            (ItemId::BasicHPPotion, 7),
        ])
    }
}

impl Location for Store {
    fn id(&self) -> LocationId {
        self.location_id()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        Store::description(self)
    }
}
