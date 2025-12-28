use std::{collections::HashMap, fmt::Display};

use uuid::Uuid;

use crate::{
    registry::{RegistryDefaults, SpawnFromSpec},
    stats::{HasStats, StatInstance, StatSheet, StatType},
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

impl SpawnFromSpec<ItemKind> for ItemSpec {
    type Output = Item;

    fn spawn_from_spec(kind: ItemKind, spec: &Self) -> Self::Output {
        Item {
            item_uuid: Uuid::new_v4(),
            kind,
            item_type: spec.item_type,
            name: spec.name,
            is_equipped: false,
            num_upgrades: 0,
            max_upgrades: spec.max_upgrades,
            max_stack_quantity: 1,
            stats: {
                let mut stats: HashMap<StatType, StatInstance> = HashMap::new();
                stats.insert(
                    StatType::Attack,
                    StatInstance {
                        stat_type: StatType::Attack,
                        current_value: spec.attack,
                        max_value: spec.attack,
                    },
                );
                stats.insert(
                    StatType::Defense,
                    StatInstance {
                        stat_type: StatType::Defense,
                        current_value: spec.defense,
                        max_value: spec.defense,
                    },
                );
                StatSheet { stats }
            },
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
                    max_upgrades: 5
                }
            ),
            (
                ItemKind::Dagger,
                ItemSpec {
                    name: "Dagger",
                    item_type: ItemType::Weapon,
                    attack: 6,
                    defense: 0,
                    max_upgrades: 5
                }
            ),
            (
                ItemKind::BasicShield,
                ItemSpec {
                    name: "Basic Shield",
                    item_type: ItemType::Shield,
                    attack: 0,
                    defense: 4,
                    max_upgrades: 5
                }
            )
        ]
    }
}
