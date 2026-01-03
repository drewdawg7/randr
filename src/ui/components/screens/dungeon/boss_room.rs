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
    combat::IsKillable,
    commands::{apply_result, execute, CommandMessage, GameCommand},
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

    // Player HP bar below boss HP
    let player = &gs.player;
    let player_hp = player.hp();
    let player_max_hp = player.max_hp();
    let player_hp_percent = if player_max_hp > 0 {
        (player_hp as f32 / player_max_hp as f32 * 100.0) as u16
    } else {
        100
    };

    let player_hp_color = if player_hp_percent > 60 {
        colors::GREEN
    } else if player_hp_percent > 30 {
        colors::YELLOW
    } else {
        colors::RED
    };

    let player_filled_chars =
        ((HP_BAR_WIDTH as f32) * (player_hp as f32 / player_max_hp as f32)).round() as u16;
    let player_empty_chars = HP_BAR_WIDTH.saturating_sub(player_filled_chars);
    let player_filled_bar = "█".repeat(player_filled_chars as usize);
    let player_empty_bar = "░".repeat(player_empty_chars as usize);

    let player_hp_line = Line::from(vec![
        Span::styled(format!("{} ", HEART), Style::default().fg(colors::RED)),
        Span::styled("[", Style::default().fg(colors::WHITE)),
        Span::styled(player_filled_bar, Style::default().fg(player_hp_color)),
        Span::styled(player_empty_bar, Style::default().fg(colors::DARK_STONE)),
        Span::styled("] ", Style::default().fg(colors::WHITE)),
        Span::styled(
            format!("{}/{}", player_hp, player_max_hp),
            Style::default().fg(player_hp_color),
        ),
        Span::styled("  YOU", Style::default().fg(colors::CYAN)),
    ]);

    let player_hp_y = hp_y + 1;
    let player_hp_area = Rect {
        x: area.x,
        y: player_hp_y,
        width: area.width,
        height: 1,
    };
    frame.render_widget(Paragraph::new(player_hp_line).centered(), player_hp_area);

    // Combat log (last few messages)
    let log_y = player_hp_y + 2;
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

    // Execute the attack boss command
    let result = execute(GameCommand::AttackBoss);

    // Extract message for combat log before applying result
    if let Some(ref msg) = result.message {
        let text = match msg {
            CommandMessage::Success(s) => s.clone(),
            CommandMessage::Info(s) => s.clone(),
            CommandMessage::Error(s) => s.clone(),
        };
        boss_combat_log.push(text);
    }

    apply_result(&result);

    // Check if boss was defeated (room should be cleared)
    let boss_defeated = gs
        .dungeon()
        .and_then(|d| d.current_room())
        .map(|r| r.is_cleared)
        .unwrap_or(false);

    if boss_defeated {
        return Some(DungeonState::Navigation);
    }

    None // Stay in BossRoom state
}
