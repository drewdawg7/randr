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
    pub fn get_goldfind(&self) -> i32 {
        self.goldfind()
    }
    pub fn get_mining(&self) -> i32 {
        self.mining()
    }

    pub fn get_effective_mining(&self) -> i32 {
        let mining = self.inventory().sum_equipment_stats(StatType::Mining);
        self.get_mining() + mining
    }
    pub fn get_effective_goldfind(&self) -> i32 {
       let gf = self.inventory().sum_equipment_stats(StatType::GoldFind);
       self.get_goldfind() + gf
    }
    pub fn get_attack(&self) -> i32 {
        self.attack()
    }

    pub fn get_defense(&self) -> i32 {
        self.def()
    }

    pub fn get_health(&self) -> i32 {
        self.hp()
    }

    pub fn get_max_health(&self) -> i32 {
        self.max_hp()
    }

}
