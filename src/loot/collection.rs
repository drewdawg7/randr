use crate::inventory::{HasInventory, ManagesItems};

use super::LootDrop;

/// Collect loot drops into a player's inventory.
///
/// Adds each item from the loot drops to the player's inventory.
/// Returns the total number of individual items collected.
pub fn collect_loot_drops(player: &mut impl HasInventory, loot_drops: &[LootDrop]) -> i32 {
    let mut total = 0;
    for loot_drop in loot_drops {
        for _ in 0..loot_drop.quantity {
            let _ = player.add_to_inv(loot_drop.item.clone());
        }
        total += loot_drop.quantity;
    }
    total
}
