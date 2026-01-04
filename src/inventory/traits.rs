use uuid::Uuid;

use crate::{item::Item, ItemId};

use super::{AddItemResult, EquipmentSlot, Inventory, InventoryError, InventoryItem};

pub trait HasInventory {
    fn inventory(&self) -> &Inventory;
    fn inventory_mut(&mut self) -> &mut Inventory;

    fn get_inventory_items(&self) -> &[InventoryItem] {
        &self.inventory().items
    }

    fn find_item_by_uuid(&self, uuid: Uuid) -> Option<&InventoryItem> {
        self.inventory().items.iter().find(|inv_item| inv_item.uuid() == uuid)
    }

    fn find_item_by_id(&self, item_id: ItemId) -> Option<&InventoryItem> {
        // Check inventory items first
        if let Some(inv_item) = self.inventory().items.iter().find(|inv_item| inv_item.item.item_id == item_id) {
            return Some(inv_item);
        }
        // Check equipment
        self.inventory().equipment().values().find(|inv_item| inv_item.item.item_id == item_id)
    }

    fn decrease_item_quantity(&mut self, inv_item: &InventoryItem, amount: u32) {
        let item_id = inv_item.item.item_id;

        // Check inventory items
        if let Some(index) = self.inventory().items.iter().position(|i| i.item.item_id == item_id) {
            self.inventory_mut().items[index].decrease_quantity(amount);
            if self.inventory().items[index].quantity == 0 {
                self.inventory_mut().items.remove(index);
            }
            return;
        }

        // Check equipment
        let slot_to_remove = self.inventory().equipment().iter()
            .find(|(_, inv)| inv.item.item_id == item_id)
            .map(|(slot, _)| *slot);

        if let Some(slot) = slot_to_remove {
            let equipment = self.inventory_mut().equipment_mut();
            if let Some(inv) = equipment.get_mut(&slot) {
                inv.decrease_quantity(amount);
                if inv.quantity == 0 {
                    equipment.remove(&slot);
                }
            }
        }
    }
    fn remove_item_from_inventory(&mut self, item: &InventoryItem) {
        let uuid = item.uuid();

        self.inventory_mut()
            .items
            .retain(|inv_item| inv_item.uuid() != uuid);
    }
    
    fn find_item_by_id_mut(&mut self, item_id: ItemId) -> Option<&mut InventoryItem> {
        // Check if in inventory items first
        let in_inventory = self.inventory().items.iter().any(|inv_item| inv_item.item.item_id == item_id);
        if in_inventory {
            return self.inventory_mut().items.iter_mut().find(|inv_item| inv_item.item.item_id == item_id);
        }
        // Check equipment
        self.inventory_mut().equipment_mut().values_mut().find(|inv_item| inv_item.item.item_id == item_id)
    }

    fn find_item_by_uuid_mut(&mut self, uuid: Uuid) -> Option<&mut InventoryItem> {
        // Check if in inventory items first
        let in_inventory = self.inventory().items.iter().any(|inv_item| inv_item.uuid() == uuid);
        if in_inventory {
            return self.inventory_mut().items.iter_mut().find(|inv_item| inv_item.uuid() == uuid);
        }
        // Check equipment
        self.inventory_mut().equipment_mut().values_mut().find(|inv_item| inv_item.uuid() == uuid)
    }
    fn find_item_index_by_uuid(&self, uuid: Uuid) -> Option<usize> {
        self.inventory().items.iter().position(|inv_item| inv_item.uuid() == uuid)
    }

    fn add_to_inv(&mut self, item: Item) -> Result<AddItemResult, InventoryError> {
        let inv = self.inventory_mut();

        // Try to stack with existing item of same kind (only for non-equipment)
        if !item.item_type.is_equipment() {
            if let Some((index, existing)) = inv.items.iter_mut()
                .enumerate()
                .find(|(_, i)| i.item.item_id == item.item_id && i.quantity < i.item.max_stack_quantity)
            {
                existing.quantity += 1;
                return Ok(AddItemResult {
                    was_stacked: true,
                    total_quantity: existing.quantity,
                    slot_index: index,
                });
            }
        }

        // Otherwise add new slot
        if inv.items.len() >= inv.max_slots() {
            return Err(InventoryError::Full);
        }
        let slot_index = inv.items.len();
        inv.items.push(InventoryItem::new(item));
        Ok(AddItemResult {
            was_stacked: false,
            total_quantity: 1,
            slot_index,
        })
    }

    fn get_equipped_item(&self, slot: EquipmentSlot) -> Option<&InventoryItem> {
        self.inventory().equipment().get(&slot)
    }

    fn unequip_item(&mut self, slot: EquipmentSlot) -> Result<Option<Item>, InventoryError> {
        // Check if inventory has room before removing from equipment
        if self.inventory().equipment().contains_key(&slot)
            && self.inventory().items.len() >= self.inventory().max_slots()
        {
            return Err(InventoryError::Full);
        }

        let inv_item = self.inventory_mut().equipment_mut().remove(&slot);

        match inv_item {
            Some(mut inv_item) => {
                inv_item.item.set_is_equipped(false);
                let item_clone = inv_item.item.clone();
                self.add_to_inv(inv_item.item)?;
                Ok(Some(item_clone))
            }
            None => Ok(None)
        }
    }

    fn equip_item(&mut self, item: &mut Item, slot: EquipmentSlot) {
        let _ = self.unequip_item(slot);
        item.set_is_equipped(true);
        self.inventory_mut().equipment_mut().insert(slot, InventoryItem::new(item.clone()));
    }

    fn equip_from_inventory(&mut self, item_uuid: Uuid, slot: EquipmentSlot) {
        let index = self.find_item_index_by_uuid(item_uuid);
        if let Some(index) = index {
            let mut inv_item = self.inventory_mut().items.remove(index);
            inv_item.item.set_is_equipped(true);
            let _ = self.unequip_item(slot);
            self.inventory_mut().equipment_mut().insert(slot, inv_item);
        }
    }

    fn remove_item(&mut self, item_uuid: Uuid) -> Option<InventoryItem> {
        // Check equipment slots first
        let equipment = self.inventory_mut().equipment_mut();
        for slot in EquipmentSlot::all() {
            if equipment.get(slot).is_some_and(|inv| inv.item.item_uuid == item_uuid) {
                return equipment.remove(slot);
            }
        }

        // Check inventory items
        if let Some(index) = self.find_item_index_by_uuid(item_uuid) {
            return Some(self.inventory_mut().items.remove(index));
        }

        None
    }
}
