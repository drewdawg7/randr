use uuid::Uuid;

use crate::{item::enums::{ItemError, ItemQuality}, magic::tome::Tome, stats::{StatSheet, StatType}};

// ItemId now comes from definitions (macro-generated)
pub use super::definitions::ItemId;
pub use super::enums::ItemType;

#[derive(Debug, Clone)]
pub struct Item {
    pub item_uuid: Uuid,
    pub item_id: ItemId,
    pub item_type: ItemType,
    pub name: &'static str,
    pub is_equipped: bool,
    pub is_locked: bool,
    pub num_upgrades: i32,
    pub max_upgrades: i32,
    pub max_stack_quantity: u32,
    pub base_stats: StatSheet,
    pub stats: StatSheet,
    pub gold_value: i32,
    pub quality: ItemQuality,
    /// Tome-specific data (pages with inscribed words). Only used for tome items.
    pub tome_data: Option<Tome>,
}

impl Item {
    pub fn set_is_equipped(&mut self, is_equipped: bool) {
        self.is_equipped = is_equipped
    }

    pub fn toggle_lock(&mut self) {
        self.is_locked = !self.is_locked;
    }

    pub fn upgrade(&mut self) -> Result<(), ItemError> {
        if !self.item_type.is_equipment() {
            return Err(ItemError::NotEquipment);
        }
        if self.num_upgrades >= self.max_upgrades {
            return Err(ItemError::MaxUpgradesReached);
        }
        self.num_upgrades += 1;
        let multiplier = 1.1;

        // Upgrade all stats that have a base value > 0
        for stat_type in StatType::all() {
            let base_value = self.base_stats.value(*stat_type);
            if base_value > 0 {
                let increase = ((base_value as f64) * (multiplier - 1.0)).round().max(1.0) as i32;
                self.base_stats.increase_stat(*stat_type, increase);
            }
        }

        self.recalculate_stats();
        Ok(())
    }

    fn recalculate_stats(&mut self) {
        self.stats = self.quality.multiply_stats(self.base_stats.clone());
    }

    pub fn upgrade_quality(&mut self) -> Result<(), ItemError> {
        if self.quality == ItemQuality::Mythic {
            return Err(ItemError::MaxQualityReached)
        }
        match self.quality.next_quality() {
            Some(next) => {
                self.quality = next;
                self.recalculate_stats();
                Ok(())
            }
            None => Ok(())
        }
    }

}
