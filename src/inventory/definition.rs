use std::collections::HashMap;

use uuid::Uuid;

use crate::item::{Item, ItemType};
use crate::item::enums::EquipmentType;
use crate::magic::tome::Tome;
use crate::stats::StatType;

use super::EquipmentSlot;

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub item: Item,
    pub quantity: u32,
}

impl InventoryItem {
    pub fn new(item: Item) -> Self {
        Self { item, quantity: 1 }
    }

    pub fn uuid(&self) -> Uuid {
        self.item.item_uuid
    }

    pub fn decrease_quantity(&mut self, amount: u32) {
        self.quantity = self.quantity.saturating_sub(amount);
    }

    pub fn increase_quantity(&mut self, amount: u32) {
        self.quantity += amount;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    max_slots: usize,
    equipment: HashMap<EquipmentSlot, InventoryItem>,
}


impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: Vec::new(),
            max_slots: 15,
            equipment: HashMap::new(),
        }
    }

    /// Create an inventory with unlimited slots (for storage).
    pub fn new_unlimited() -> Self {
        Inventory {
            items: Vec::new(),
            max_slots: usize::MAX,
            equipment: HashMap::new(),
        }
    }

    pub fn equipment(&self) -> &HashMap<EquipmentSlot, InventoryItem> {
        &self.equipment
    }

    pub fn equipment_mut(&mut self) -> &mut HashMap<EquipmentSlot, InventoryItem> {
        &mut self.equipment
    }

    pub fn max_slots(&self) -> usize {
        self.max_slots
    }

    pub fn sum_equipment_stats(&self, stat_type: StatType) -> i32 {
        self.equipment
            .values()
            .map(|inv_item| inv_item.item.stats.value(stat_type))
            .sum()
    }

    /// Get the equipped tome's data, if a tome is equipped in the off-hand slot
    pub fn equipped_tome(&self) -> Option<&Tome> {
        self.equipment
            .get(&EquipmentSlot::OffHand)
            .and_then(|inv_item| {
                // Check if the equipped item is a tome
                if let ItemType::Equipment(EquipmentType::Tome) = inv_item.item.item_type {
                    inv_item.item.tome_data.as_ref()
                } else {
                    None
                }
            })
    }

    /// Get mutable access to the equipped tome's data
    pub fn equipped_tome_mut(&mut self) -> Option<&mut Tome> {
        self.equipment
            .get_mut(&EquipmentSlot::OffHand)
            .and_then(|inv_item| {
                // Check if the equipped item is a tome
                if let ItemType::Equipment(EquipmentType::Tome) = inv_item.item.item_type {
                    inv_item.item.tome_data.as_mut()
                } else {
                    None
                }
            })
    }
}
