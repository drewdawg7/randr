use crate::item::ItemId;
use crate::registry::RegistryDefaults;

use super::definition::ConsumableEffect;

impl RegistryDefaults<ItemId> for ConsumableEffect {
    fn defaults() -> impl IntoIterator<Item = (ItemId, Self)> {
        [(ItemId::BasicHPPotion, ConsumableEffect::RestoreHealth(50))]
    }
}
