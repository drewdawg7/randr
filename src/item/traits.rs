use std::{collections::HashMap, fmt::Display};

use uuid::Uuid;

use crate::{
    item::enums::ItemQuality, loot::traits::WorthGold, registry::{RegistryDefaults, SpawnFromSpec}, stats::{HasStats, StatInstance, StatSheet, StatType}
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
    fn gold_value(&self) -> i32 { self.gold_value }
}

impl SpawnFromSpec<ItemKind> for ItemSpec {
    type Output = Item;

    fn spawn_from_spec(kind: ItemKind, spec: &Self) -> Self::Output {
        let quality = ItemQuality::roll();
        let mut sheet = StatSheet { stats: HashMap::new() };
        sheet.insert(StatType::Attack.instance(spec.attack));
        sheet.insert(StatType::Defense.instance(spec.defense));
        let q_sheet = quality.multiply_stats(sheet);
        Item {
            item_uuid: Uuid::new_v4(),
            kind,
            item_type: spec.item_type,
            name: spec.name,
            is_equipped: false,
            num_upgrades: 0,
            max_upgrades: spec.max_upgrades,
            max_stack_quantity: 1,
            gold_value: spec.gold_value,
            stats: q_sheet,
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
                    attack: 10,
                    defense: 0,
                    max_upgrades: 5,
                    gold_value: 15
                }
            ),
            (
                ItemKind::Dagger,
                ItemSpec {
                    name: "Dagger",
                    item_type: ItemType::Weapon,
                    attack: 6,
                    defense: 0,
                    max_upgrades: 5,
                    gold_value: 10,
                }
            ),
            (
                ItemKind::BasicShield,
                ItemSpec {
                    name: "Basic Shield",
                    item_type: ItemType::Shield,
                    attack: 0,
                    defense: 4,
                    max_upgrades: 5,
                    gold_value: 15
                }
            )
        ]
    }
}
