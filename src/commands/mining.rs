//! Mining-related game commands.
//!
//! Handles rock mining and loot collection.

use crate::inventory::HasInventory;
use crate::system::game_state;

use super::CommandResult;

/// Mine the current rock.
///
/// Deals mining damage to the rock. If destroyed, collects loot and spawns a new rock.
/// Returns loot information for UI display.
pub fn mine_rock() -> CommandResult {
    let gs = game_state();
    let mining_damage = gs.player.effective_mining();

    let Some(mut rock) = gs.town.mine.current_rock.take() else {
        return CommandResult::error("No rock to mine");
    };

    if let Some(drops) = rock.mine(mining_damage) {
        // Rock was destroyed - add loot to inventory
        let mut total_items = 0;
        for loot_drop in &drops {
            for _ in 0..loot_drop.quantity {
                let _ = gs.player.add_to_inv(loot_drop.item.clone());
            }
            total_items += loot_drop.quantity;
        }

        // Spawn a new rock
        gs.town.mine.spawn_rock();

        CommandResult::success(format!("Rock destroyed! Got {} items", total_items))
    } else {
        // Rock still alive - put it back
        gs.town.mine.current_rock = Some(rock);
        CommandResult::ok()
    }
}
