use uuid::Uuid;

use crate::item::{Item, ItemId};

use super::{AddItemResult, EquipmentSlot, Inventory, InventoryError, InventoryItem};

// =============================================================================
// Core Trait - Required methods only
// =============================================================================

/// Core trait for entities that have an inventory.
/// Only requires implementing the accessor methods.
pub trait HasInventory {
    fn inventory(&self) -> &Inventory;
    fn inventory_mut(&mut self) -> &mut Inventory;
}

// =============================================================================
// ManagesItems - Item vector management
// =============================================================================

/// Extension trait for managing inventory items (the items vec).
/// All methods have default implementations.
pub trait ManagesItems: HasInventory {
    fn get_inventory_items(&self) -> &[InventoryItem] {
        &self.inventory().items
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

    /// Remove an item from inventory items only (not equipment).
    fn remove_item_from_inventory(&mut self, item: &InventoryItem) {
        let uuid = item.uuid();
        self.inventory_mut()
            .items
            .retain(|inv_item| inv_item.uuid() != uuid);
    }

    /// Find an item's index in the inventory items vec by UUID.
    fn find_item_index_by_uuid(&self, uuid: Uuid) -> Option<usize> {
        self.inventory().items.iter().position(|inv_item| inv_item.uuid() == uuid)
    }

    /// Count total quantity of an item across inventory (not equipment).
    fn count_item(&self, item_id: ItemId) -> u32 {
        self.inventory()
            .items
            .iter()
            .filter(|i| i.item.item_id == item_id)
            .map(|i| i.quantity)
            .sum()
    }

    /// Decrease item quantity, removing if it reaches zero.
    /// Searches both inventory items and equipment.
    fn decrease_item_quantity(&mut self, item_id: ItemId, amount: u32) {

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
}

// Blanket implementation for all types with HasInventory
impl<T: HasInventory> ManagesItems for T {}

// =============================================================================
// ManagesEquipment - Equipment hashmap management
// =============================================================================

/// Extension trait for managing equipped items.
/// All methods have default implementations.
pub trait ManagesEquipment: HasInventory + ManagesItems {
    fn get_equipped_item(&self, slot: EquipmentSlot) -> Option<&InventoryItem> {
        self.inventory().equipment().get(&slot)
    }

    fn unequip_item(&mut self, slot: EquipmentSlot) -> Result<(), InventoryError> {
        // Check if inventory has room before removing from equipment
        if self.inventory().equipment().contains_key(&slot)
            && self.inventory().items.len() >= self.inventory().max_slots()
        {
            return Err(InventoryError::Full);
        }

        if let Some(mut inv_item) = self.inventory_mut().equipment_mut().remove(&slot) {
            inv_item.item.set_is_equipped(false);
            self.add_to_inv(inv_item.item)?;
        }
        Ok(())
    }

    fn equip_item(&mut self, mut item: Item, slot: EquipmentSlot) {
        let _ = self.unequip_item(slot);
        item.set_is_equipped(true);
        self.inventory_mut().equipment_mut().insert(slot, InventoryItem::new(item));
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
}

// Blanket implementation for all types with HasInventory + ManagesItems
impl<T: HasInventory + ManagesItems> ManagesEquipment for T {}

// =============================================================================
// FindsItems - Search operations across both storages
// =============================================================================

/// Extension trait for finding items in both inventory and equipment.
/// All methods have default implementations.
pub trait FindsItems: HasInventory {
    /// Find item by UUID in inventory items only.
    fn find_item_by_uuid(&self, uuid: Uuid) -> Option<&InventoryItem> {
        self.inventory().items.iter().find(|inv_item| inv_item.uuid() == uuid)
    }

    /// Find item by ItemId, searching inventory items first, then equipment.
    fn find_item_by_id(&self, item_id: ItemId) -> Option<&InventoryItem> {
        // Check inventory items first
        if let Some(inv_item) = self.inventory().items.iter().find(|inv_item| inv_item.item.item_id == item_id) {
            return Some(inv_item);
        }
        // Check equipment
        self.inventory().equipment().values().find(|inv_item| inv_item.item.item_id == item_id)
    }

    /// Find mutable item by ItemId, searching inventory items first, then equipment.
    fn find_item_by_id_mut(&mut self, item_id: ItemId) -> Option<&mut InventoryItem> {
        // Check if in inventory items first
        let in_inventory = self.inventory().items.iter().any(|inv_item| inv_item.item.item_id == item_id);
        if in_inventory {
            return self.inventory_mut().items.iter_mut().find(|inv_item| inv_item.item.item_id == item_id);
        }
        // Check equipment
        self.inventory_mut().equipment_mut().values_mut().find(|inv_item| inv_item.item.item_id == item_id)
    }

    /// Find mutable item by UUID, searching inventory items first, then equipment.
    fn find_item_by_uuid_mut(&mut self, uuid: Uuid) -> Option<&mut InventoryItem> {
        // Check if in inventory items first
        let in_inventory = self.inventory().items.iter().any(|inv_item| inv_item.uuid() == uuid);
        if in_inventory {
            return self.inventory_mut().items.iter_mut().find(|inv_item| inv_item.uuid() == uuid);
        }
        // Check equipment
        self.inventory_mut().equipment_mut().values_mut().find(|inv_item| inv_item.uuid() == uuid)
    }

    /// Remove item by UUID from either inventory or equipment.
    fn remove_item(&mut self, item_uuid: Uuid) -> Option<InventoryItem> {
        // Check equipment slots first
        let equipment = self.inventory_mut().equipment_mut();
        for slot in EquipmentSlot::all() {
            if equipment.get(slot).is_some_and(|inv| inv.item.item_uuid == item_uuid) {
                return equipment.remove(slot);
            }
        }

        // Check inventory items
        if let Some(index) = self.inventory().items.iter().position(|inv_item| inv_item.uuid() == item_uuid) {
            return Some(self.inventory_mut().items.remove(index));
        }

        None
    }
}

// Blanket implementation for all types with HasInventory
impl<T: HasInventory> FindsItems for T {}

// =============================================================================
// HasInventory impl for Inventory itself
// =============================================================================

/// Inventory implements HasInventory so it can use all the trait methods directly.
impl HasInventory for Inventory {
    fn inventory(&self) -> &Inventory {
        self
    }
    fn inventory_mut(&mut self) -> &mut Inventory {
        self
    }
}
