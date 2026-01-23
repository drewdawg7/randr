use bevy::prelude::*;

use crate::inventory::{EquipmentSlot, Inventory, ManagesEquipment, ManagesItems};
use crate::ui::screens::modal::spawn_modal_overlay;
use crate::ui::widgets::{ItemGrid, ItemGridEntry};

use super::state::InventoryModalRoot;

/// Spawn the inventory modal UI with just an ItemGrid showing all inventory items.
pub fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    let mut items: Vec<ItemGridEntry> = Vec::new();

    // Add equipped items first
    for slot in EquipmentSlot::all() {
        if let Some(inv_item) = inventory.get_equipped_item(*slot) {
            items.push(ItemGridEntry {
                sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            });
        }
    }

    // Add backpack items
    for inv_item in inventory.get_inventory_items() {
        items.push(ItemGridEntry {
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
        });
    }

    let overlay = spawn_modal_overlay(commands);
    commands
        .entity(overlay)
        .insert(InventoryModalRoot)
        .with_children(|parent| {
            parent.spawn(ItemGrid {
                items,
                selected_index: 0,
                is_focused: false,
            });
        });
}
