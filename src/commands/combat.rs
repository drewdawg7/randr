//! Combat-related game commands.
//!
//! Handles player attacks, running, and combat result processing.

use crate::combat::{
    enemy_attack_step, player_attack_step, process_defeat, process_victory, CombatPhase,
};
use crate::dungeon::Explorable;
use crate::loot::collect_loot_drops;
use crate::system::{game_state, CombatSource};
use crate::ui::Id;

use super::CommandResult;

/// Execute player attack against the current enemy.
///
/// Returns information about what happened (damage dealt, victory, defeat).
pub fn player_attack() -> CommandResult {
    let gs = game_state();

    let Some(mut combat) = gs.active_combat.take() else {
        return CommandResult::error("No active combat");
    };

    if combat.phase != CombatPhase::PlayerTurn {
        gs.active_combat = Some(combat);
        return CommandResult::error("Not player's turn");
    }

    // Player attacks
    player_attack_step(&gs.player, &mut combat);

    // Check for victory
    if combat.phase == CombatPhase::Victory {
        // Process victory rewards
        process_victory(&mut gs.player, &mut combat);

        // Add loot drops to player inventory
        collect_loot_drops(&mut gs.player, &combat.loot_drops, None);

        // If in dungeon, mark the current room as cleared
        if gs.combat_source == CombatSource::Dungeon {
            if let Some(dungeon) = gs.dungeon_mut() {
                if let Some(room) = dungeon.current_room_mut() {
                    room.clear();
                }
            }
        }

        gs.active_combat = Some(combat);
        return CommandResult::ok();
    }

    // Enemy counter-attacks
    enemy_attack_step(&mut combat, &mut gs.player);

    // Check for defeat
    if combat.phase == CombatPhase::Defeat {
        process_defeat(&mut gs.player);
        gs.active_combat = Some(combat);
        return CommandResult::ok();
    }

    // Back to player turn
    combat.phase = CombatPhase::PlayerTurn;
    gs.active_combat = Some(combat);
    CommandResult::ok()
}

/// Player runs from combat.
pub fn player_run() -> CommandResult {
    let gs = game_state();

    // Determine return screen based on combat source
    let screen = match gs.combat_source {
        CombatSource::Dungeon => Id::Dungeon,
        CombatSource::Field => Id::Town,
    };

    CommandResult::ok().with_screen(screen)
}

/// Return from combat after victory or defeat.
pub fn return_from_combat() -> CommandResult {
    let gs = game_state();

    let Some(combat) = gs.active_combat() else {
        return CommandResult::error("No active combat");
    };

    let was_defeat = combat.phase == CombatPhase::Defeat;

    match gs.combat_source {
        CombatSource::Dungeon => {
            if was_defeat {
                // Player died - kick out of dungeon
                gs.reset_dungeon();
                gs.leave_dungeon();
                return CommandResult::error("You were defeated! Retreating from dungeon...");
            } else {
                // Victory - mark room as cleared and stay in dungeon
                if let Some(dungeon) = gs.dungeon_mut() {
                    if let Some(room) = dungeon.current_room_mut() {
                        room.clear();
                    }
                }
                return CommandResult::ok().with_screen(Id::Dungeon);
            }
        }
        CombatSource::Field => {
            // Reset combat source to default
            gs.combat_source = CombatSource::default();
            return CommandResult::ok().with_screen(Id::Town);
        }
    }
}

/// Start a new fight from the field.
pub fn start_new_fight() -> CommandResult {
    let gs = game_state();

    // Only allow from field
    if gs.combat_source != CombatSource::Field {
        return return_from_combat();
    }

    let field = &gs.town.field;
    match field.spawn_mob(&gs.player) {
        Ok(mob) => {
            gs.start_combat(mob);
            CommandResult::ok().with_screen(Id::Fight)
        }
        Err(_) => CommandResult::error("No enemies to fight"),
    }
}
