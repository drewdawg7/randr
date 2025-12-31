use crate::{loot::LootTable, registry::Registry};

use super::super::RockId;

#[derive(Clone)]
pub struct RockSpec {
    pub rock_id: RockId,
    pub name: &'static str,
    pub health: i32,
    pub loot: LootTable,
}

pub type RockRegistry = Registry<super::super::RockId, RockSpec>;
