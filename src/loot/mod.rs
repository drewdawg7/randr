pub(crate) mod definition;
pub(crate) mod enums;
pub(crate) mod traits;

pub(crate) use definition::{LootDrop, LootItem, LootTable};
pub(crate) use traits::HasLoot;

use crate::inventory::HasInventory;
use crate::toast::ToastQueue;

/// Collect loot drops into a player's inventory.
///
/// Adds each item from the loot drops to the player's inventory.
/// If `toasts` is provided, shows a success toast for each item type collected.
///
/// Returns the total number of individual items collected.
pub fn collect_loot_drops(
    player: &mut impl HasInventory,
    loot_drops: &[LootDrop],
    mut toasts: Option<&mut ToastQueue>,
) -> i32 {
    let mut total = 0;
    for loot_drop in loot_drops {
        for _ in 0..loot_drop.quantity {
            let _ = player.add_to_inv(loot_drop.item.clone());
        }
        total += loot_drop.quantity;

        if let Some(t) = toasts.as_mut() {
            t.success(format!("Found: {} x{}", loot_drop.item.name, loot_drop.quantity));
        }
    }
    total
}
