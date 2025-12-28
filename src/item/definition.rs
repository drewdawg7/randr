
use std::fmt::Display;

use crate::registry::{Registry, RegistryDefaults, SpawnFromSpec};


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub item_type: ItemType,
    pub name: &'static str,
    pub attack: i32,
    pub is_equipped: bool,
}
impl Item {
    pub fn set_is_equipped(&mut self, is_equipped: bool) {
        self.is_equipped = is_equipped
    }
}

#[derive(Debug, Clone)]
pub struct ItemSpec {
    pub name: &'static str,
    pub item_type: ItemType,
    pub attack: i32,

}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemKind {
    Sword,
    Dagger
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemType {
    Weapon
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.attack)
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
            attack: spec.attack,
            is_equipped: false
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
                }
            ),
            (
                ItemKind::Dagger,
                ItemSpec {
                    name: "Dagger",
                    item_type: ItemType::Weapon,
                    attack: 6
                }
            )
        ]
    }
}
