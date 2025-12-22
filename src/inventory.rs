use std::{collections::HashMap};

use crate::item::{Item};

#[derive(Default, Debug, Clone)]
pub struct Inventory {
    items: Vec<Item>,
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

    fn add_to_inv(&mut self, item: Item) -> Result<(), InventoryError> {
        let inv = self.inventory_mut();
        if inv.items.len() >= inv.max_slots {
            return Err(InventoryError::Full);
        }
        inv.items.push(item);
        Ok(())
    }

    fn get_equipped_item(&self, slot: EquipmentSlot) -> Option<&Item>{
        self.inventory().equipment.get(&slot)
    }

    fn unequip_item(&mut self, slot: EquipmentSlot) -> Result<(), InventoryError> {
        let item = self.inventory_mut().equipment.remove(&slot);

        match item {
            Some(item) => {
                self.add_to_inv(item);
                Ok(())
            }
            None => { Ok(()) }
        }
    }

    fn equip_item(&mut self, item: Item, slot: EquipmentSlot) {
        self.unequip_item(slot);
        self.inventory_mut().equipment.insert(slot, item);
    }

}

pub enum InventoryError{
    Full
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EquipmentSlot {
    Weapon
}
