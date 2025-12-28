use uuid::Uuid;

use crate::{item::enums::ItemError, registry::Registry, stats::{HasStats, StatSheet}};

pub use super::enums::{ItemKind, ItemType};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item {
    pub item_uuid: Uuid,
    pub kind: ItemKind,
    pub item_type: ItemType,
    pub name: &'static str,
    pub is_equipped: bool,
    pub num_upgrades: i32,
    pub max_upgrades: i32,
    pub max_stack_quantity: u32,
    pub stats: StatSheet,
}

impl Item {
    pub fn set_is_equipped(&mut self, is_equipped: bool) {
        self.is_equipped = is_equipped
    }

    pub fn upgrade(&mut self) -> Result<(), ItemError> {
        if self.num_upgrades >= self.max_upgrades {
            return Err(ItemError::MaxUpgradesReached)
        }
        self.num_upgrades += 1;
        match self.item_type {
            ItemType::Weapon => self.inc_attack(3),
            ItemType::Shield => self.inc_def(1),
        };
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemSpec {
    pub name: &'static str,
    pub item_type: ItemType,
    pub max_upgrades: i32,
    pub attack: i32,
    pub defense: i32,
}

pub type ItemRegistry = Registry<ItemKind, ItemSpec>;
