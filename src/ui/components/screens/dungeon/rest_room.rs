//! Rest room state for dungeon screen.
//!
//! Handles rest areas where the player can heal.

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

use crate::{
    combat::Combatant,
    stats::HasStats,
    system::game_state,
    ui::{
        components::{
            dungeon::campfire_art::{campfire_width, render_campfire_art},
            utilities::{selection_prefix, HEART, RETURN_ARROW},
        },
        theme as colors,
    },
};

use super::DungeonState;

/// Render the rest room UI.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    rest_selection: usize,
) {
    let gs = game_state();
    let player = &gs.player;

    // Get campfire art
    let campfire_lines = render_campfire_art();
    let campfire_height = campfire_lines.len() as u16;
    let campfire_w = campfire_width();

    // HP bar constants
    const HP_BAR_WIDTH: u16 = 20;

    // Calculate total content height: campfire + spacing + title + HP bar + spacing + menu
    let total_height = campfire_height + 1 + 1 + 1 + 1 + 2; // campfire + gap + title + hp + gap + 2 menu items

    // Center everything vertically
    let y_offset = area.y + area.height.saturating_sub(total_height) / 2;

    // Center campfire horizontally
    let campfire_x = area.x + area.width.saturating_sub(campfire_w) / 2;

    // Render campfire lines directly to buffer, skipping spaces to preserve background
    let buf = frame.buffer_mut();
    for (i, line) in campfire_lines.into_iter().enumerate() {
        let y = y_offset + i as u16;
        if y >= area.y + area.height {
            break;
        }
        // Render each span, skipping space characters to preserve background
        let mut x = campfire_x;
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

    // Title "Rest Area" centered below campfire
    let title_y = y_offset + campfire_height + 1;
    let title = Paragraph::new(Line::from(vec![Span::styled(
        "Rest Area",
        Style::default().fg(colors::CYAN),
    )]))
    .centered();
    let title_area = Rect {
        x: area.x,
        y: title_y,
        width: area.width,
        height: 1,
    };
    frame.render_widget(title, title_area);

    // HP bar below title
    let hp = player.hp();
    let max_hp = player.max_hp();
    let hp_percent = if max_hp > 0 {
        (hp as f32 / max_hp as f32 * 100.0) as u16
    } else {
        100
    };

    let hp_color = if hp_percent > 60 {
        colors::GREEN
    } else if hp_percent > 30 {
        colors::YELLOW
    } else {
        colors::RED
    };

    // Create HP bar: [████████░░░░░░░░░░░░] 81/100
    let filled_chars = ((HP_BAR_WIDTH as f32) * (hp as f32 / max_hp as f32)).round() as u16;
    let empty_chars = HP_BAR_WIDTH.saturating_sub(filled_chars);
    let filled_bar = "█".repeat(filled_chars as usize);
    let empty_bar = "░".repeat(empty_chars as usize);

    let hp_bar_line = Line::from(vec![
        Span::styled(format!("{} ", HEART), Style::default().fg(colors::RED)),
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

    // Menu below HP bar
    let heal_amount = (max_hp as f32 * 0.5).round() as i32;
    let can_heal = hp < max_hp;

    let rest_style = if rest_selection == 0 {
        Style::default().fg(colors::YELLOW)
    } else if can_heal {
        Style::default().fg(colors::GREEN)
    } else {
        Style::default().fg(colors::DARK_STONE)
    };

    let leave_style = if rest_selection == 1 {
        Style::default().fg(colors::YELLOW)
    } else {
        Style::default().fg(colors::WHITE)
    };

    let rest_text = if can_heal {
        format!("Rest (+{} HP)", heal_amount.min(max_hp - hp))
    } else {
        "Rest (Full HP)".to_string()
    };

    let menu_y = hp_y + 2;
    let menu_width: u16 = 20;
    let menu_x = area.x + area.width.saturating_sub(menu_width) / 2;

    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            selection_prefix(rest_selection == 0),
            Span::styled(rest_text, rest_style),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(rest_selection == 1),
            Span::styled(format!("{} Continue", RETURN_ARROW), leave_style),
        ])),
    ];

    let menu_area = Rect {
        x: menu_x,
        y: menu_y,
        width: menu_width,
        height: 2,
    };
    let menu = List::new(menu_items);
    frame.render_widget(menu, menu_area);
}

/// Handle rest room submit action.
/// Returns the new state to transition to, or None to stay in current state.
pub fn handle_submit(rest_selection: usize) -> Option<DungeonState> {
    let gs = game_state();

    match rest_selection {
        0 => {
            // Rest/Heal: restore 50% of max HP
            let player = &mut gs.player;
            let max_hp = player.max_hp();
            let current_hp = player.hp();

            if current_hp < max_hp {
                let heal_amount = (max_hp as f32 * 0.5).round() as i32;
                let actual_heal = heal_amount.min(max_hp - current_hp);
                player.increase_health(actual_heal);
                gs.toasts.success(format!("Rested and recovered {} HP!", actual_heal));
            } else {
                gs.toasts.info("Already at full health!");
            }
            None // Stay in RestRoom state
        }
        1 => {
            // Continue: go to navigation
            Some(DungeonState::Navigation)
        }
        _ => None,
    }
}
