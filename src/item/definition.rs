
use std::{collections::HashMap, fmt::Display};

use crate::{registry::{Registry, RegistryDefaults, SpawnFromSpec}, stats::{HasStats, StatInstance, StatSheet, StatType}};


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub item_type: ItemType,
    pub name: &'static str,
    pub is_equipped: bool,
    pub stats: StatSheet 
}
impl Item {
    pub fn set_is_equipped(&mut self, is_equipped: bool) {
        self.is_equipped = is_equipped
    }
}

impl HasStats for Item {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

#[derive(Debug, Clone)]
pub struct ItemSpec {
    pub name: &'static str,
    pub item_type: ItemType,
    pub attack: i32,
    pub defense: i32,

}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemKind {
    Sword,
    Dagger,
    BasicShield
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemType {
    Weapon,
    Shield 
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.attack())
    }
}

pub type ItemRegistry = Registry<ItemKind, ItemSpec>;


impl SpawnFromSpec<ItemKind> for ItemSpec {
    type Output = Item;

    fn spawn_from_spec(kind: ItemKind, spec: &Self) -> Self::Output {
        Item {
            kind,
            item_type: spec.item_type,
            name: spec.name,
            is_equipped: false,
  
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
                }
            ),
            (
                ItemKind::Dagger,
                ItemSpec {
                    name: "Dagger",
                    item_type: ItemType::Weapon,
                    attack: 6,
                    defense: 0
                }
            ),
            (
                ItemKind::BasicShield,
                ItemSpec {
                    name: "Basic Shield",
                    item_type: ItemType::Shield,
                    attack: 0,
                    defense: 4
                }
            )
        ]
    }
}
