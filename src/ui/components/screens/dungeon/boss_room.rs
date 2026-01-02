//! Boss room state for dungeon screen.
//!
//! Handles the boss fight room where the player is trapped until they win or die.

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

use crate::{
    combat::{self, Combatant, DealsDamage, HasGold, IsKillable, Named},
    entities::progression::HasProgression,
    inventory::HasInventory,
    stats::HasStats,
    system::game_state,
    ui::{
        components::{
            dungeon::dragon_art::{self, DRAGON_HEIGHT, DRAGON_WIDTH},
            utilities::{selection_prefix, HEART},
        },
        theme as colors,
    },
};

use super::DungeonState;

/// Render the boss room UI.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    boss_combat_log: &[String],
) {
    let gs = game_state();

    // Get dragon art
    let dragon_lines = dragon_art::render_dragon_art();
    let dragon_height = DRAGON_HEIGHT;
    let dragon_w = DRAGON_WIDTH;

    // HP bar constants
    const HP_BAR_WIDTH: u16 = 30;

    // Get boss HP from stored boss (or show 0 if dead/missing)
    let (hp, max_hp) = if let Some(dungeon) = gs.dungeon() {
        if let Some(boss) = &dungeon.boss {
            (boss.hp(), boss.max_hp())
        } else {
            (0, 1)
        }
    } else {
        (0, 1)
    };

    // Calculate total content height
    let combat_log_height = boss_combat_log.len().min(3) as u16;
    let total_height = dragon_height + 1 + 1 + 1 + 1 + combat_log_height + 2;

    // Center everything vertically
    let y_offset = area.y + area.height.saturating_sub(total_height) / 2;

    // Center dragon horizontally
    let dragon_x = area.x + area.width.saturating_sub(dragon_w) / 2;

    // Render dragon lines directly to buffer, skipping spaces to preserve background
    let buf = frame.buffer_mut();
    for (i, line) in dragon_lines.into_iter().enumerate() {
        let y = y_offset + i as u16;
        if y >= area.y + area.height {
            break;
        }
        let mut x = dragon_x;
        for span in line.spans {
            for ch in span.content.chars() {
                if x >= area.x + area.width {
                    break;
                }
                if ch != ' ' {
                    buf.set_string(x, y, ch.to_string(), span.style);
                }
                x += 1;
            }
        }
    }

    // Title "DRAGON" centered below dragon
    let title_y = y_offset + dragon_height + 1;
    let title = Paragraph::new(Line::from(vec![Span::styled(
        "DRAGON",
        Style::default().fg(colors::EMBER_RED),
    )]))
    .centered();
    let title_area = Rect {
        x: area.x,
        y: title_y,
        width: area.width,
        height: 1,
    };
    frame.render_widget(title, title_area);

    // Boss HP bar
    let hp_percent = if max_hp > 0 {
        (hp as f32 / max_hp as f32 * 100.0) as u16
    } else {
        0
    };
    let hp_color = if hp_percent > 60 {
        colors::EMBER_RED
    } else if hp_percent > 30 {
        colors::FLAME_ORANGE
    } else {
        colors::BRIGHT_YELLOW
    };

    let filled_chars = ((HP_BAR_WIDTH as f32) * (hp as f32 / max_hp as f32)).round() as u16;
    let empty_chars = HP_BAR_WIDTH.saturating_sub(filled_chars);
    let filled_bar = "█".repeat(filled_chars as usize);
    let empty_bar = "░".repeat(empty_chars as usize);

    let hp_bar_line = Line::from(vec![
        Span::styled(format!("{} ", HEART), Style::default().fg(colors::EMBER_RED)),
        Span::styled("[", Style::default().fg(colors::WHITE)),
        Span::styled(filled_bar, Style::default().fg(hp_color)),
        Span::styled(empty_bar, Style::default().fg(colors::DARK_STONE)),
        Span::styled("] ", Style::default().fg(colors::WHITE)),
        Span::styled(format!("{}/{}", hp, max_hp), Style::default().fg(hp_color)),
    ]);

    let hp_y = title_y + 1;
    let hp_area = Rect {
        x: area.x,
        y: hp_y,
        width: area.width,
        height: 1,
    };
    frame.render_widget(Paragraph::new(hp_bar_line).centered(), hp_area);

    // Combat log (last few messages)
    let log_y = hp_y + 2;
    for (i, msg) in boss_combat_log.iter().rev().take(3).rev().enumerate() {
        let log_line = Paragraph::new(Line::from(vec![Span::styled(
            msg.clone(),
            Style::default().fg(colors::WHITE),
        )]))
        .centered();
        let log_area = Rect {
            x: area.x,
            y: log_y + i as u16,
            width: area.width,
            height: 1,
        };
        frame.render_widget(log_line, log_area);
    }

    // Attack menu below combat log
    let attack_style = Style::default().fg(colors::YELLOW);
    let menu_y = log_y + combat_log_height.max(1) + 1;
    let menu_width: u16 = 20;
    let menu_x = area.x + area.width.saturating_sub(menu_width) / 2;

    let menu_items: Vec<ListItem> = vec![ListItem::new(Line::from(vec![
        selection_prefix(true),
        Span::styled("Attack", attack_style),
    ]))];

    let menu_area = Rect {
        x: menu_x,
        y: menu_y,
        width: menu_width,
        height: 1,
    };
    let menu = List::new(menu_items);
    frame.render_widget(menu, menu_area);
}

/// Handle boss room submit action (attack).
/// Returns the new state if transitioning, and combat log messages to add.
pub fn handle_submit(boss_combat_log: &mut Vec<String>) -> Option<DungeonState> {
    let gs = game_state();

    // Check if boss is alive
    let boss_alive = gs
        .dungeon()
        .and_then(|d| d.boss.as_ref())
        .map(|b| b.is_alive())
        .unwrap_or(false);

    if !boss_alive {
        return None;
    }

    // Get player attack stats
    let player_attack = gs.player.get_attack();

    // Player attacks boss
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

    boss_combat_log.push(format!("You deal {} damage to Dragon!", player_damage));

    if boss_died {
        // Victory! Get death rewards from boss
        let death_result = {
            if let Some(dungeon) = gs.dungeon_mut() {
                dungeon.boss.as_mut().map(|boss| boss.on_death())
            } else {
                None
            }
        };

        if let Some(death_result) = death_result {
            // Apply gold with goldfind bonus
            let gf = gs.player.effective_goldfind();
            let multiplier = 1.0 + (gf as f64 / 100.0);
            let gold_with_bonus =
                ((death_result.gold_dropped as f64) * multiplier).round() as i32;
            gs.player.add_gold(gold_with_bonus);

            // Award XP
            gs.player.gain_xp(death_result.xp_dropped);

            // Add loot to inventory
            for loot in &death_result.loot_drops {
                for _ in 0..loot.quantity {
                    let _ = gs.player.add_to_inv(loot.item.clone());
                }
                gs.toasts
                    .success(format!("Obtained: {} x{}", loot.item.name, loot.quantity));
            }

            gs.toasts.success(format!(
                "Dragon defeated! +{} gold, +{} XP",
                gold_with_bonus, death_result.xp_dropped
            ));
        }

        // Clear the boss room
        if let Some(dungeon) = gs.dungeon_mut() {
            if let Some(room) = dungeon.current_room_mut() {
                room.clear();
            }
            dungeon.boss = None;
        }

        boss_combat_log.push("Dragon has been slain!".to_string());
        return Some(DungeonState::Navigation);
    }

    // Boss attacks player (if boss still alive)
    let (boss_attack_range, boss_name) = {
        if let Some(dungeon) = gs.dungeon() {
            if let Some(boss) = &dungeon.boss {
                (Some(boss.get_attack()), boss.name().to_string())
            } else {
                (None, String::new())
            }
        } else {
            (None, String::new())
        }
    };

    if let Some(attack_range) = boss_attack_range {
        let raw_damage = attack_range.roll_damage();
        let defense = gs.player.effective_defense();
        let damage = combat::apply_defense(raw_damage, defense);
        gs.player.take_damage(damage);

        boss_combat_log.push(format!("{} deals {} damage to you!", boss_name, damage));

        // Check if player died
        if !gs.player.is_alive() {
            // Player died - process death and kick out of dungeon
            gs.player.on_death();
            gs.toasts.error("You were slain by the Dragon!");
            gs.reset_dungeon();
            gs.leave_dungeon();
        }
    }

    None // Stay in BossRoom state
}
