use std::fmt::Display;

use uuid::Uuid;

use crate::{
    item::{enums::ItemQuality, ToolKind}, loot::traits::WorthGold, registry::{RegistryDefaults, SpawnFromSpec}, stats::{HasStats, StatSheet, StatType}
};

use super::{Item, ItemSpec, ItemId, ItemType, EquipmentType, MaterialType};

impl HasStats for Item {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.attack())
    }
}

impl WorthGold for Item {
    fn gold_value(&self) -> i32 {
        let base = self.gold_value;
        let quality_multiplier = self.quality.value_multiplier();
        ((base as f64) * quality_multiplier).round() as i32
    }
}

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
            (
                ItemId::Sword,
                ItemSpec {
                    name: "Sword",
                    item_type: ItemType::Equipment(EquipmentType::Weapon),
                    quality: None,
                    stats: StatSheet::new().with(StatType::Attack, 10),
                    max_upgrades: 5,
                    max_stack_quantity: 1,
                    gold_value: 15
                }
            ),
            (
                ItemId::BronzePickaxe,
                ItemSpec {
                    name: "Bronze Pickaxe",
                    item_type: ItemType::Equipment(EquipmentType::Tool(ToolKind::Pickaxe)),
                    quality: None,
                    stats: StatSheet::new()
                        .with(StatType::Attack, 10)
                        .with(StatType::Mining, 10),
                    max_upgrades: 5,
                    max_stack_quantity: 1,
                    gold_value: 50
                }
            ),
            (
                ItemId::Dagger,
                ItemSpec {
                    name: "Dagger",
                    item_type: ItemType::Equipment(EquipmentType::Weapon),
                    quality: None,
                    stats: StatSheet::new().with(StatType::Attack, 6),
                    max_upgrades: 5,
                    max_stack_quantity: 1,
                    gold_value: 10,
                }
            ),
            (
                ItemId::BasicShield,
                ItemSpec {
                    name: "Basic Shield",
                    item_type: ItemType::Equipment(EquipmentType::Shield),
                    quality: None,
                    stats: StatSheet::new().with(StatType::Defense, 4),
                    max_upgrades: 5,
                    max_stack_quantity: 1,
                    gold_value: 15
                }
            ),
            (
                ItemId::GoldRing,
                ItemSpec {
                    name: "Midas' Touch",
                    item_type: ItemType::Equipment(EquipmentType::Ring),
                    quality: None,
                    stats: StatSheet::new().with(StatType::GoldFind, 10),
                    max_upgrades: 7,
                    max_stack_quantity: 1,
                    gold_value: 50
                }
            ),
            (
                ItemId::QualityUpgradeStone,
                ItemSpec {
                    name: "Magic Rock",
                    item_type: ItemType::Material(MaterialType::UpgradeStone),
                    quality: Some(ItemQuality::Mythic),
                    stats: StatSheet::new(),
                    max_upgrades: 0,
                    max_stack_quantity: 99,
                    gold_value: 500,
                }
            ),
            (
                ItemId::Coal,
                ItemSpec {
                    name: "Coal",
                    item_type: ItemType::Material(MaterialType::Ore),
                    quality: Some(ItemQuality::Normal),
                    stats: StatSheet::new(),
                    max_upgrades: 0,
                    max_stack_quantity: 99,
                    gold_value: 4,
                }
            ),
            (
                ItemId::CopperOre,
                ItemSpec {
                    name: "Copper Ore",
                    item_type: ItemType::Material(MaterialType::Ore),
                    quality: Some(ItemQuality::Normal),
                    stats: StatSheet::new(),
                    max_upgrades: 0,
                    max_stack_quantity: 99,
                    gold_value: 5,
                }
            ),
            (
                ItemId::TinOre,
                ItemSpec {
                    name: "Tin Ore",
                    item_type: ItemType::Material(MaterialType::Ore),
                    quality: Some(ItemQuality::Normal),
                    stats: StatSheet::new(),
                    max_upgrades: 0,
                    max_stack_quantity: 99,
                    gold_value: 5,
                }
            )
        ]
    }
}
