use std::collections::HashMap;

use uuid::Uuid;

use crate::item::Item;

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
}

#[derive(Default, Debug, Clone)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    max_slots: usize,
    equipment: HashMap<EquipmentSlot, Item>,
}


impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: Vec::new(),
            max_slots: 5,
            equipment: HashMap::new()
        }
    }

    pub fn equipment(&self) -> &HashMap<EquipmentSlot, Item> {
        &self.equipment
    }

    pub fn equipment_mut(&mut self) -> &mut HashMap<EquipmentSlot, Item> {
        &mut self.equipment
    }

    pub fn max_slots(&self) -> usize {
        self.max_slots
    }
}
