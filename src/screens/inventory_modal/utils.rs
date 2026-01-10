use crate::game::Player;
use crate::inventory::{EquipmentSlot, ManagesEquipment, ManagesItems};

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
