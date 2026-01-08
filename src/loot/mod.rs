pub(crate) mod definition;
pub(crate) mod enums;
pub(crate) mod traits;
#[cfg(test)]
mod tests;

pub(crate) use definition::{LootDrop, LootTable};
pub(crate) use traits::HasLoot;

use crate::inventory::{HasInventory, ManagesItems};

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
