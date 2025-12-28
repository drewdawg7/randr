use std::{collections::HashMap};

use crate::item::{Item, definition::ItemKind};

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

    fn get_inventory_items(&self) -> &[Item] {
        &self.inventory().items
    }

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
            Some(mut item) => {
                item.set_is_equipped(false);
                let _ = self.add_to_inv(item);
                Ok(())
            }
            None => { Ok(()) }
        }
    }

    fn equip_item(&mut self, item: &mut Item, slot: EquipmentSlot) {
        let _ = self.unequip_item(slot);
        item.set_is_equipped(true);
        self.inventory_mut().equipment.insert(slot, *item);
    }

    fn equip_from_inventory(&mut self, kind: ItemKind, slot: EquipmentSlot) {
        let index = self.inventory().items.iter().position(|i| i.kind == kind);
        if let Some(index) = index {
            let mut item = self.inventory_mut().items.remove(index);
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
    Weapon
}
