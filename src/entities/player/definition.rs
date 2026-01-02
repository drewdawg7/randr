use crate::HasInventory;
use crate::{entities::Progression, inventory::Inventory, stats::StatSheet};
use crate::stats::{HasStats, StatType};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: &'static str,
    pub gold: i32,
    pub prog: Progression,
    pub inventory: Inventory,
    pub stats: StatSheet,
}


impl Player {
    pub fn effective_mining(&self) -> i32 {
        let mining = self.inventory().sum_equipment_stats(StatType::Mining);
        self.mining() + mining
    }

    pub fn effective_goldfind(&self) -> i32 {
        let gf = self.inventory().sum_equipment_stats(StatType::GoldFind);
        self.goldfind() + gf
    }
}
