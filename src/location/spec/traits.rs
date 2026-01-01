use crate::{
    location::enums::LocationId,
    registry::RegistryDefaults,
};

use super::definition::LocationSpec;
use super::specs::{VILLAGE_BLACKSMITH, VILLAGE_FIELD, VILLAGE_MINE, VILLAGE_STORE};

impl RegistryDefaults<LocationId> for LocationSpec {
    fn defaults() -> impl IntoIterator<Item = (LocationId, Self)> {
        [
            (LocationId::VillageStore, VILLAGE_STORE.clone()),
            (LocationId::VillageBlacksmith, VILLAGE_BLACKSMITH.clone()),
            (LocationId::VillageField, VILLAGE_FIELD.clone()),
            (LocationId::VillageMine, VILLAGE_MINE.clone()),
        ]
    }
}
