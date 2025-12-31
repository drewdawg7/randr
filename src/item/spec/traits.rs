use uuid::Uuid;

use crate::{
    item::{enums::ItemQuality, spec::specs::BASIC_HP_POTION},
    registry::{RegistryDefaults, SpawnFromSpec},
};

use super::super::{definition::Item, enums::ItemId};
use super::definition::ItemSpec;
use super::specs::{
    BASIC_SHIELD, BRONZE_PICKAXE, COAL, COPPER_ORE, DAGGER, GOLD_RING, QUALITY_UPGRADE_STONE,
    SWORD, TIN_ORE,
};

impl SpawnFromSpec<ItemId> for ItemSpec {
    type Output = Item;

    fn spawn_from_spec(kind: ItemId, spec: &Self) -> Self::Output {
        // Use fixed quality from spec, or roll if None
        let quality = spec.quality.unwrap_or_else(ItemQuality::roll);
        let base_stats = spec.stats.clone();
        let stats = quality.multiply_stats(base_stats.clone());
        Item {
            item_uuid: Uuid::new_v4(),
            kind,
            item_type: spec.item_type,
            name: spec.name,
            is_equipped: false,
            is_locked: false,
            num_upgrades: 0,
            max_upgrades: spec.max_upgrades,
            max_stack_quantity: spec.max_stack_quantity,
            gold_value: spec.gold_value,
            base_stats,
            stats,
            quality,
        }
    }
}

impl RegistryDefaults<ItemId> for ItemSpec {
    fn defaults() -> impl IntoIterator<Item = (ItemId, Self)> {
        [
            (ItemId::BasicHPPotion, BASIC_HP_POTION.clone()),
            (ItemId::Sword, SWORD.clone()),
            (ItemId::BronzePickaxe, BRONZE_PICKAXE.clone()),
            (ItemId::Dagger, DAGGER.clone()),
            (ItemId::BasicShield, BASIC_SHIELD.clone()),
            (ItemId::GoldRing, GOLD_RING.clone()),
            (ItemId::QualityUpgradeStone, QUALITY_UPGRADE_STONE.clone()),
            (ItemId::Coal, COAL.clone()),
            (ItemId::CopperOre, COPPER_ORE.clone()),
            (ItemId::TinOre, TIN_ORE.clone()),
        ]
    }
}
