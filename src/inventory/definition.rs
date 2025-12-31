use std::collections::HashMap;

use uuid::Uuid;

use crate::item::Item;
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
        self.quantity = (self.quantity - amount).max(0);
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
            max_slots: 10,
            equipment: HashMap::new()
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
}
