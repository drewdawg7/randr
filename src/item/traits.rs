use std::{collections::HashMap, fmt::Display};

use uuid::Uuid;

use crate::{
    item::enums::ItemQuality, loot::traits::WorthGold, registry::{RegistryDefaults, SpawnFromSpec}, stats::{HasStats, StatSheet, StatType}
};

use super::{Item, ItemSpec, ItemKind, ItemType};

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

impl SpawnFromSpec<ItemKind> for ItemSpec {
    type Output = Item;

    fn spawn_from_spec(kind: ItemKind, spec: &Self) -> Self::Output {
        // Use fixed quality from spec, or roll if None
        let quality = spec.quality.unwrap_or_else(ItemQuality::roll);
        let mut base_stats = StatSheet { stats: HashMap::new() };
        base_stats.insert(StatType::Attack.instance(spec.attack));
        base_stats.insert(StatType::Defense.instance(spec.defense));
        base_stats.insert(StatType::GoldFind.instance(spec.gold_find));
        let stats = quality.multiply_stats(base_stats.clone());
        Item {
            item_uuid: Uuid::new_v4(),
            kind,
            item_type: spec.item_type,
            name: spec.name,
            is_equipped: false,
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

impl RegistryDefaults<ItemKind> for ItemSpec {
    fn defaults() -> impl IntoIterator<Item = (ItemKind, Self)> {
        [
            (
                ItemKind::Sword,
                ItemSpec {
                    name: "Sword",
                    item_type: ItemType::Weapon,
                    quality: None,
                    attack: 10,
                    defense: 0,
                    gold_find: 0,
                    max_upgrades: 5,
                    max_stack_quantity: 1,
                    gold_value: 15
                }
            ),
            (
                ItemKind::Dagger,
                ItemSpec {
                    name: "Dagger",
                    item_type: ItemType::Weapon,
                    quality: None,
                    attack: 6,
                    defense: 0,
                    gold_find: 0,
                    max_upgrades: 5,
                    max_stack_quantity: 1,
                    gold_value: 10,
                }
            ),
            (
                ItemKind::BasicShield,
                ItemSpec {
                    name: "Basic Shield",
                    item_type: ItemType::Shield,
                    quality: None,
                    attack: 0,
                    defense: 4,
                    gold_find: 0,
                    max_upgrades: 5,
                    max_stack_quantity: 1,
                    gold_value: 15
                }
            ),

            (
                ItemKind::GoldRing,
                ItemSpec {
                    name: "Midas' Touch",
                    item_type: ItemType::Ring,
                    quality: None,
                    attack: 0,
                    defense: 0,
                    gold_find: 10,
                    max_upgrades: 7,
                    max_stack_quantity: 1,
                    gold_value: 50
                }
            ),
            (
                ItemKind::QualityUpgradeStone,
                ItemSpec {
                    name: "Magic Rock",
                    item_type: ItemType::Material,
                    quality: Some(ItemQuality::Mythic),
                    attack: 0,
                    defense: 0,
                    gold_find: 0,
                    max_upgrades: 0,
                    max_stack_quantity: 99,
                    gold_value: 500,
                }
            )
        ]
    }
}
