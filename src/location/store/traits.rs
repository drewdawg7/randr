use std::time::Duration;

use crate::{
    player::Player,
    item::ItemId,
    location::{Location, LocationEntryError, LocationId},
};

use super::definition::Store;

impl Default for Store {
    fn default() -> Self {
        let mut store = Store::new("The Shop");
        store.add_stock(ItemId::Sword, 1);
        store.add_stock(ItemId::Dagger, 1);
        store.add_stock(ItemId::BronzePickaxe, 1);
        store.add_stock(ItemId::BasicHPPotion, 7);
        store
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

    fn tick(&mut self, _elapsed: Duration) {
        self.check_and_restock();
    }

    fn refresh(&mut self) {
        self.restock();
    }

    fn time_until_refresh(&self) -> Option<Duration> {
        let secs = self.time_until_restock();
        Some(Duration::from_secs(secs))
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
