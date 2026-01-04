//! Mining-related game commands.
//!
//! Handles rock mining and loot collection.

use crate::loot::{collect_loot_drops, LootDrop};
use crate::magic::effect::PassiveEffect;
use crate::system::game_state;

use super::CommandResult;

/// Result of a mining operation - extends CommandResult with drop info.
pub struct MiningResult {
    /// Standard command result.
    pub result: CommandResult,
    /// Loot drops if rock was destroyed.
    pub drops: Vec<LootDrop>,
}

/// Mine the current rock.
///
/// Deals mining damage to the rock. If destroyed, collects loot and spawns a new rock.
/// Returns mining result with drop information for UI display.
pub fn mine_rock() -> MiningResult {
    let gs = game_state();
    let base_mining = gs.player.effective_mining();

    // Apply BonusMining passive effects
    let mining_bonus: i32 = gs
        .player
        .tome_passive_effects()
        .iter()
        .filter_map(|e| {
            if let PassiveEffect::BonusMining(amt) = e {
                Some(*amt)
            } else {
                None
            }
        })
        .sum();

    let mining_damage = base_mining + mining_bonus;
    let magic_find = gs.player.effective_magicfind();

    let Some(mut rock) = gs.town.mine.current_rock.take() else {
        return MiningResult {
            result: CommandResult::error("No rock to mine"),
            drops: vec![],
        };
    };

    if let Some(drops) = rock.mine(mining_damage, magic_find) {
        // Rock was destroyed - add loot to inventory
        collect_loot_drops(&mut gs.player, &drops, None);

        // Spawn a new rock
        gs.town.mine.spawn_rock(&gs.player);

        MiningResult {
            result: CommandResult::ok(),
            drops,
        }
    } else {
        // Rock still alive - put it back
        gs.town.mine.current_rock = Some(rock);
        MiningResult {
            result: CommandResult::ok(),
            drops: vec![],
        }
    }
}
