use std::time::Duration;

use crate::item::ItemId;
use crate::location::{Location, LocationId, Refreshable};

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

impl Refreshable for Store {
    fn tick(&mut self, elapsed: Duration) {
        self.tick_timer(elapsed);
        self.check_and_restock();
    }

    fn refresh(&mut self) {
        self.restock();
    }

    fn time_until_refresh(&self) -> Duration {
        Duration::from_secs(self.time_until_restock())
    }
}
