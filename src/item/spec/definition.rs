use crate::{registry::Registry, stats::StatSheet};

use super::super::enums::{ItemId, ItemQuality, ItemType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemSpec {
    pub name: &'static str,
    pub item_type: ItemType,
    pub quality: Option<ItemQuality>,
    pub max_upgrades: i32,
    pub max_stack_quantity: u32,
    pub stats: StatSheet,
    pub gold_value: i32,
}

pub type ItemRegistry = Registry<ItemId, ItemSpec>;
