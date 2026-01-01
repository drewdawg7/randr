use std::time::Duration;

use crate::location::{Alchemist, Blacksmith, Field, Location, LocationId, Mine, Store};

pub struct Town {
    pub name: String,
    pub store: Store,
    pub blacksmith: Blacksmith,
    pub alchemist: Alchemist,
    pub field: Field,
    pub mine: Mine,
}

impl Town {
    pub fn new(
        name: String,
        store: Store,
        blacksmith: Blacksmith,
        alchemist: Alchemist,
        field: Field,
        mine: Mine,
    ) -> Self {
        Self {
            name,
            store,
            blacksmith,
            alchemist,
            field,
            mine,
        }
    }

    /// Get a reference to a location by its ID
    pub fn location(&self, id: LocationId) -> &dyn Location {
        match id {
            LocationId::VillageStore => &self.store,
            LocationId::VillageBlacksmith => &self.blacksmith,
            LocationId::VillageAlchemist => &self.alchemist,
            LocationId::VillageField => &self.field,
            LocationId::VillageMine => &self.mine,
        }
    }

    /// Get a mutable reference to a location by its ID
    pub fn location_mut(&mut self, id: LocationId) -> &mut dyn Location {
        match id {
            LocationId::VillageStore => &mut self.store,
            LocationId::VillageBlacksmith => &mut self.blacksmith,
            LocationId::VillageAlchemist => &mut self.alchemist,
            LocationId::VillageField => &mut self.field,
            LocationId::VillageMine => &mut self.mine,
        }
    }

    /// Tick all locations with elapsed time
    pub fn tick_all(&mut self, elapsed: Duration) {
        self.store.tick(elapsed);
        self.blacksmith.tick(elapsed);
        self.alchemist.tick(elapsed);
        self.field.tick(elapsed);
        self.mine.tick(elapsed);
    }
}
