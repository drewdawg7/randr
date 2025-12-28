use std::collections::HashMap;

use uuid::Uuid;

use crate::item::Item;

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
}

pub trait HasInventory {
    fn inventory(&self) -> &Inventory;
    fn inventory_mut(&mut self) -> &mut Inventory;

    fn get_inventory_items(&self) -> &[InventoryItem] {
        &self.inventory().items
    }

    fn find_item_by_uuid(&self, uuid: Uuid) -> Option<&InventoryItem> {
        self.inventory().items.iter().find(|inv_item| inv_item.uuid() == uuid)
    }

    fn find_item_index_by_uuid(&self, uuid: Uuid) -> Option<usize> {
        self.inventory().items.iter().position(|inv_item| inv_item.uuid() == uuid)
    }

    fn add_to_inv(&mut self, item: Item) -> Result<(), InventoryError> {
        let inv = self.inventory_mut();
        if inv.items.len() >= inv.max_slots {
            return Err(InventoryError::Full);
        }
        inv.items.push(InventoryItem::new(item));
        Ok(())
    }

    fn get_equipped_item(&self, slot: EquipmentSlot) -> Option<&Item> {
        self.inventory().equipment.get(&slot)
    }

    fn unequip_item(&mut self, slot: EquipmentSlot) -> Result<(), InventoryError> {
        let item = self.inventory_mut().equipment.remove(&slot);

        match item {
            Some(mut item) => {
                item.set_is_equipped(false);
                let _ = self.add_to_inv(item);
                Ok(())
            }
            None => Ok(())
        }
    }

    fn equip_item(&mut self, item: &mut Item, slot: EquipmentSlot) {
        let _ = self.unequip_item(slot);
        item.set_is_equipped(true);
        self.inventory_mut().equipment.insert(slot, item.clone());
    }

    fn equip_from_inventory(&mut self, item_uuid: Uuid, slot: EquipmentSlot) {
        let index = self.find_item_index_by_uuid(item_uuid);
        if let Some(index) = index {
            let inv_item = self.inventory_mut().items.remove(index);
            let mut item = inv_item.item;
            item.set_is_equipped(true);
            let _ = self.unequip_item(slot);
            self.inventory_mut().equipment.insert(slot, item);
        }
    }
}

pub enum InventoryError{
    Full
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EquipmentSlot {
    Weapon,
    OffHand
}
