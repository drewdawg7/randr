use uuid::Uuid;

use crate::{
    item::{enums::{EquipmentType, ItemQuality, ItemType}, spec::specs::{BASIC_HP_POTION, BONK_STICK, BRONZE_INGOT, BRONZE_SWORD, COPPER_INGOT, COPPER_SWORD, COWHIDE, IMBA_RING, SLIMEGEL, TIN_INGOT, TIN_SWORD}},
    magic::tome::Tome,
    registry::{RegistryDefaults, SpawnFromSpec},
};

use super::super::{definition::Item, enums::ItemId};
use super::definition::ItemSpec;
use super::specs::{
    BASIC_SHIELD, BRONZE_PICKAXE, COAL, COPPER_ORE, DAGGER, GOLD_RING, QUALITY_UPGRADE_STONE,
    SWORD, TIN_ORE, APPRENTICE_TOME,
    // Copper Armor
    COPPER_HELMET, COPPER_CHESTPLATE, COPPER_GAUNTLETS, COPPER_GREAVES, COPPER_LEGGINGS,
    // Tin Armor
    TIN_HELMET, TIN_CHESTPLATE, TIN_GAUNTLETS, TIN_GREAVES, TIN_LEGGINGS,
    // Bronze Armor
    BRONZE_HELMET, BRONZE_CHESTPLATE, BRONZE_GAUNTLETS, BRONZE_GREAVES, BRONZE_LEGGINGS,
};

impl SpawnFromSpec<ItemId> for ItemSpec {
    type Output = Item;

    fn spawn_from_spec(item_id: ItemId, spec: &Self) -> Self::Output {
        // Use fixed quality from spec, or roll if None
        let quality = spec.quality.unwrap_or_else(ItemQuality::roll);
        let base_stats = spec.stats.clone();
        let stats = quality.multiply_stats(base_stats.clone());

        // Initialize tome_data if this is a tome item
        let tome_data = match spec.item_type {
            ItemType::Equipment(EquipmentType::Tome) => Some(Tome::standard()),
            _ => None,
        };

        Item {
            item_uuid: Uuid::new_v4(),
            item_id,
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
            tome_data,
        }
    }
}

impl RegistryDefaults<ItemId> for ItemSpec {
    fn defaults() -> impl IntoIterator<Item = (ItemId, Self)> {
        [
            (ItemId::BasicHPPotion, BASIC_HP_POTION.clone()),
            (ItemId::ImbaRing, IMBA_RING.clone()),
            (ItemId::BonkStick, BONK_STICK.clone()),
            (ItemId::Sword, SWORD.clone()),
            (ItemId::BronzePickaxe, BRONZE_PICKAXE.clone()),
            (ItemId::Dagger, DAGGER.clone()),
            (ItemId::BasicShield, BASIC_SHIELD.clone()),
            (ItemId::ApprenticeTome, APPRENTICE_TOME.clone()),
            (ItemId::GoldRing, GOLD_RING.clone()),
            (ItemId::QualityUpgradeStone, QUALITY_UPGRADE_STONE.clone()),
            (ItemId::Coal, COAL.clone()),
            (ItemId::CopperOre, COPPER_ORE.clone()),
            (ItemId::TinOre, TIN_ORE.clone()),
            (ItemId::CopperIngot, COPPER_INGOT.clone()),
            (ItemId::TinIngot, TIN_INGOT.clone()),
            (ItemId::BronzeIngot, BRONZE_INGOT.clone()),
            (ItemId::BronzeSword, BRONZE_SWORD.clone()),
            (ItemId::TinSword, TIN_SWORD.clone()),
            (ItemId::CopperSword, COPPER_SWORD.clone()),
            (ItemId::SlimeGel, SLIMEGEL.clone()),
            (ItemId::Cowhide, COWHIDE.clone()),
            // Copper Armor
            (ItemId::CopperHelmet, COPPER_HELMET.clone()),
            (ItemId::CopperChestplate, COPPER_CHESTPLATE.clone()),
            (ItemId::CopperGauntlets, COPPER_GAUNTLETS.clone()),
            (ItemId::CopperGreaves, COPPER_GREAVES.clone()),
            (ItemId::CopperLeggings, COPPER_LEGGINGS.clone()),
            // Tin Armor
            (ItemId::TinHelmet, TIN_HELMET.clone()),
            (ItemId::TinChestplate, TIN_CHESTPLATE.clone()),
            (ItemId::TinGauntlets, TIN_GAUNTLETS.clone()),
            (ItemId::TinGreaves, TIN_GREAVES.clone()),
            (ItemId::TinLeggings, TIN_LEGGINGS.clone()),
            // Bronze Armor
            (ItemId::BronzeHelmet, BRONZE_HELMET.clone()),
            (ItemId::BronzeChestplate, BRONZE_CHESTPLATE.clone()),
            (ItemId::BronzeGauntlets, BRONZE_GAUNTLETS.clone()),
            (ItemId::BronzeGreaves, BRONZE_GREAVES.clone()),
            (ItemId::BronzeLeggings, BRONZE_LEGGINGS.clone()),
        ]
    }
}
