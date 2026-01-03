use crate::stats::StatType;
use super::HasInventory;

/// Trait for entities that can wear equipment
pub trait HasEquipment: HasInventory {
    fn equipment_stat_bonus(&self, stat_type: StatType) -> i32 {
        self.inventory().sum_equipment_stats(stat_type)
    }

    fn equipment_attack(&self) -> i32 {
        self.equipment_stat_bonus(StatType::Attack)
    }

    fn equipment_defense(&self) -> i32 {
        self.equipment_stat_bonus(StatType::Defense)
    }
}
