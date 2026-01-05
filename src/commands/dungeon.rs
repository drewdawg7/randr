//! Dungeon-related game commands.
//!
//! Handles room interactions, navigation, resting, and boss fights.

use crate::combat::{self, Combatant, DealsDamage, IsKillable};
use crate::dungeon::{Direction, Explorable, RoomType};
use crate::mob::MobId;
use crate::loot::collect_loot_drops;
use crate::stats::HasStats;
use crate::system::{game_state, CombatSource};
use crate::ui::Id;

use super::CommandResult;

/// Enter/interact with the current dungeon room.
pub fn enter_room() -> CommandResult {
    let gs = game_state();

    // Get room info
    let (is_cleared, room_type) = {
        if let Some(dungeon) = gs.dungeon() {
            if let Some(room) = dungeon.current_room() {
                (room.is_cleared, Some(room.room_type))
            } else {
                (false, None)
            }
        } else {
            return CommandResult::error("Not in a dungeon");
        }
    };

    if is_cleared {
        return CommandResult::ok();
    }

    let Some(room_type) = room_type else {
        return CommandResult::error("Invalid room");
    };

    match room_type {
        RoomType::Monster => {
            // Spawn a mob and start combat
            let mob_result = gs.dungeon().and_then(|d| d.spawn_mob().ok());

            match mob_result {
                Some(mob) => {
                    gs.combat_source = CombatSource::Dungeon;
                    gs.start_combat(mob);
                    CommandResult::ok().with_screen(Id::Fight)
                }
                None => CommandResult::error("No enemies to fight!"),
            }
        }
        RoomType::Boss => {
            // Spawn boss if needed
            let needs_spawn = gs.dungeon().map(|d| d.boss.is_none()).unwrap_or(false);
            if needs_spawn {
                let dragon = MobId::Dragon.spawn();
                if let Some(dungeon) = gs.dungeon_mut() {
                    dungeon.boss = Some(dragon);
                }
            }
            CommandResult::ok()
        }
        RoomType::Chest => {
            // Open the chest and get loot
            let magic_find = gs.player.effective_magicfind();
            let loot_drops = {
                if let Some(dungeon) = gs.dungeon_mut() {
                    if let Some(room) = dungeon.current_room_mut() {
                        let drops = room.open_chest(magic_find);
                        room.clear();
                        drops
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            };

            // Add items to inventory
            let total = collect_loot_drops(&mut gs.player, &loot_drops, Some(&mut gs.ui.toasts));

            if loot_drops.is_empty() {
                CommandResult::info("The chest was empty.")
            } else {
                CommandResult::success(format!("Found {} items!", total))
            }
        }
        _ => {
            // Other room types - just clear
            if let Some(dungeon) = gs.dungeon_mut() {
                if let Some(room) = dungeon.current_room_mut() {
                    room.clear();
                }
            }
            CommandResult::ok()
        }
    }
}

/// Move in a direction in the dungeon.
pub fn move_dungeon(direction: Direction) -> CommandResult {
    let gs = game_state();

    if let Some(dungeon) = gs.dungeon_mut() {
        if let Ok(room) = dungeon.move_player(direction) {
            // Check if entering a boss room
            let is_boss_room = room.room_type == RoomType::Boss && !room.is_cleared;

            if is_boss_room {
                // Spawn boss if needed
                if dungeon.boss.is_none() {
                    let dragon = MobId::Dragon.spawn();
                    if let Some(d) = gs.dungeon_mut() {
                        d.boss = Some(dragon);
                    }
                }
            }

            CommandResult::ok()
        } else {
            CommandResult::error("Cannot move in that direction")
        }
    } else {
        CommandResult::error("Not in a dungeon")
    }
}

/// Leave the dungeon and return to town.
pub fn leave_dungeon() -> CommandResult {
    let gs = game_state();
    gs.leave_dungeon();
    CommandResult::ok().with_screen(Id::Town)
}

/// Rest at a rest room to heal.
pub fn rest() -> CommandResult {
    let gs = game_state();

    let max_hp = gs.player.max_hp();
    let current_hp = gs.player.hp();

    if current_hp >= max_hp {
        return CommandResult::info("Already at full health!");
    }

    let heal_amount = (max_hp as f32 * 0.5).round() as i32;
    let actual_heal = heal_amount.min(max_hp - current_hp);
    gs.player.increase_health(actual_heal);

    CommandResult::success(format!("Rested and recovered {} HP!", actual_heal))
}

/// Attack the boss in a boss room.
///
/// Returns combat log messages for display.
pub fn attack_boss() -> CommandResult {
    let gs = game_state();

    // Check if boss is alive
    let boss_alive = gs
        .dungeon()
        .and_then(|d| d.boss.as_ref())
        .map(|b| b.is_alive())
        .unwrap_or(false);

    if !boss_alive {
        return CommandResult::ok();
    }

    // Player attacks boss
    let player_attack = gs.player.get_attack();
    let (player_damage, boss_died) = {
        if let Some(dungeon) = gs.dungeon_mut() {
            if let Some(boss) = dungeon.boss.as_mut() {
                let raw_damage = player_attack.roll_damage();
                let defense = boss.effective_defense();
                let damage = combat::apply_defense(raw_damage, defense);
                boss.take_damage(damage);
                let died = !boss.is_alive();
                (damage, died)
            } else {
                (0, true)
            }
        } else {
            (0, true)
        }
    };

    if boss_died {
        // Victory! Get death rewards
        let magic_find = gs.player.effective_magicfind();
        let death_result = {
            if let Some(dungeon) = gs.dungeon_mut() {
                dungeon.boss.as_mut().map(|boss| boss.on_death(magic_find))
            } else {
                None
            }
        };

        if let Some(death_result) = death_result {
            // Apply gold and XP rewards using shared helper (includes XP multiplier from tomes)
            let rewards = combat::apply_victory_rewards(
                &mut gs.player,
                death_result.gold_dropped,
                death_result.xp_dropped,
            );

            // Add loot to inventory
            collect_loot_drops(&mut gs.player, &death_result.loot_drops, Some(&mut gs.ui.toasts));

            gs.ui.toasts.success(format!(
                "Dragon defeated! +{} gold, +{} XP",
                rewards.gold_gained, rewards.xp_gained
            ));
        }

        // Clear the boss room
        if let Some(dungeon) = gs.dungeon_mut() {
            if let Some(room) = dungeon.current_room_mut() {
                room.clear();
            }
            dungeon.boss = None;
        }

        return CommandResult::success("Dragon has been slain!");
    }

    // Boss counter-attacks
    let (boss_damage, player_died) = {
        if let Some(dungeon) = gs.dungeon() {
            if let Some(boss) = &dungeon.boss {
                let attack_range = boss.get_attack();
                let raw_damage = attack_range.roll_damage();
                let defense = gs.player.effective_defense();
                let damage = combat::apply_defense(raw_damage, defense);
                gs.player.take_damage(damage);
                let died = !gs.player.is_alive();
                (damage, died)
            } else {
                (0, false)
            }
        } else {
            (0, false)
        }
    };

    if player_died {
        gs.player.on_death(0);
        gs.reset_dungeon();
        gs.leave_dungeon();
        return CommandResult::error("You were slain by the Dragon!");
    }

    // Combat continues
    CommandResult::info(format!(
        "You dealt {} damage. Dragon dealt {} damage.",
        player_damage, boss_damage
    ))
}
