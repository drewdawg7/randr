use bevy::prelude::*;

use crate::inventory::{EquipmentSlot, InventoryItem};
use crate::item::Item;
use crate::ui::SelectionState;

/// Component marker for the inventory modal UI.
#[derive(Component)]
pub struct InventoryModalRoot;

/// Component for individual inventory item UI elements.
#[derive(Component)]
pub struct InventoryItemUI {
    pub index: usize,
}

/// Resource for tracking which item is selected in the inventory.
#[derive(Resource, Default)]
pub struct InventorySelection {
    pub index: usize,
    pub count: usize,
}

impl SelectionState for InventorySelection {
    fn selected(&self) -> usize {
        self.index
    }

    fn count(&self) -> usize {
        self.count
    }

    fn set_selected(&mut self, index: usize) {
        self.index = index;
    }
}

impl InventorySelection {
    pub fn set_count(&mut self, count: usize) {
        self.count = count;
        self.clamp_to_bounds();
    }
}

/// Information about an item in the inventory display.
#[derive(Clone)]
pub enum ItemInfo {
    Equipped(EquipmentSlot, Item),
    Backpack(uuid::Uuid, InventoryItem),
}

impl ItemInfo {
    pub fn item(&self) -> &Item {
        match self {
            ItemInfo::Equipped(_, item) => item,
            ItemInfo::Backpack(_, inv_item) => &inv_item.item,
        }
    }

    pub fn quantity(&self) -> u32 {
        match self {
            ItemInfo::Equipped(_, _) => 1,
            ItemInfo::Backpack(_, inv_item) => inv_item.quantity,
        }
    }

    pub fn is_equipped(&self) -> bool {
        matches!(self, ItemInfo::Equipped(_, _))
    }
}
