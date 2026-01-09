use bevy::prelude::*;

use crate::game::Player;
use crate::inventory::{EquipmentSlot, ManagesEquipment, ManagesItems};
use crate::item::ItemType;

use super::state::ItemInfo;

/// Get all items for display (equipped first, then backpack).
pub fn get_all_inventory_items(player: &Player) -> Vec<ItemInfo> {
    let mut items = Vec::new();

    // Add equipped items first
    for slot in EquipmentSlot::all() {
        if let Some(inv_item) = player.get_equipped_item(*slot) {
            items.push(ItemInfo::Equipped(*slot, inv_item.item.clone()));
        }
    }

    // Add backpack items
    for inv_item in player.get_inventory_items() {
        items.push(ItemInfo::Backpack(inv_item.item.item_uuid, inv_item.clone()));
    }

    items
}

/// Get the display color for an item quality.
pub fn get_quality_color(quality: &crate::item::enums::ItemQuality) -> Color {
    use crate::item::enums::ItemQuality;
    match quality {
        ItemQuality::Poor => Color::srgb(0.6, 0.6, 0.6),
        ItemQuality::Normal => Color::srgb(1.0, 1.0, 1.0),
        ItemQuality::Improved => Color::srgb(0.3, 1.0, 0.3),
        ItemQuality::WellForged => Color::srgb(0.3, 0.5, 1.0),
        ItemQuality::Masterworked => Color::srgb(0.8, 0.3, 1.0),
        ItemQuality::Mythic => Color::srgb(1.0, 0.5, 0.0),
    }
}

/// Format an item type for display.
pub fn format_item_type(item_type: &ItemType) -> String {
    match item_type {
        ItemType::Equipment(eq) => format!("Equipment ({:?})", eq),
        ItemType::Material(mat) => format!("Material ({:?})", mat),
        ItemType::Consumable(con) => format!("Consumable ({:?})", con),
        ItemType::QuestItem => "Quest Item".to_string(),
    }
}
