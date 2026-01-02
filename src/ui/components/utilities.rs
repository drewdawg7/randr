#![allow(dead_code)]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, ListState},
    Frame,
};

use crate::inventory::{EquipmentSlot, HasInventory, InventoryItem};
use crate::item::Item;
use crate::ui::theme::{self as colors, quality_color, ColorExt};

use crate::{location::{Blacksmith, Store}, system::game_state, ui::Id};
use super::widgets::menu::{Menu, MenuItem};

pub const HEART: char           = '\u{F004}';
pub const COIN: char            = '\u{EDE8}';
pub const CROSSED_SWORDS: char  = '\u{f0787}';
pub const CHECKED: char         = '\u{F14A}';
pub const UNCHECKED: char       = '\u{F0C8}';
pub const STORE: char           = '\u{ee17}';
pub const PERSON: char          = '\u{F415}';
pub const SHIRT: char           = '\u{EE1C}';
pub const OPEN_DOOR: char       = '\u{F081C}';
pub const SHIELD: char          = '\u{F132}';
pub const ANVIL: char           = '\u{F089B}';
pub const DOUBLE_ARROW_UP: char = '\u{F102}';
pub const HOUSE: char           = '\u{F015}';
pub const RETURN_ARROW: char    = '\u{F17B1}';
pub const HAMMER: char          = '\u{EEFF}';
pub const LOCK: char            = '\u{F023}';
pub const PICKAXE: char         = '\u{F08B7}';
pub const HOURGLASS: char       = '\u{F252}';
pub const FIRE: char            = '\u{F0238}';
pub const FLASK: char           = '\u{F0093}';

/// Returns a styled prefix Span for list items. Selected items get a yellow ">", unselected get "  ".
pub fn selection_prefix(is_selected: bool) -> Span<'static> {
    if is_selected {
        Span::styled("> ", Style::default().color(colors::YELLOW))
    } else {
        Span::raw("  ")
    }
}

/// Returns a lock icon Span if the item is locked, otherwise an empty span.
pub fn lock_prefix(item: &Item) -> Span<'static> {
    if item.is_locked {
        Span::styled(format!("{} ", LOCK), Style::default().color(colors::BRONZE))
    } else {
        Span::raw("")
    }
}

/// Returns a styled Span for an item, colored by quality.
/// - Equipment: "{name} (+{num_upgrades})"
/// - Materials: "{name} (x{quantity})"
pub fn item_display(item: &Item, quantity: Option<u32>) -> Span<'static> {
    let color = quality_color(item.quality);
    let text = if item.item_type.is_equipment() {
        format!("{} (+{})", item.name, item.num_upgrades)
    } else {
        match quantity {
            Some(q) => format!("{} (x{})", item.name, q),
            None => item.name.to_string(),
        }
    };
    Span::styled(text, Style::default().color(color))
}

pub fn blacksmith_header(blacksmith: &Blacksmith, gold: i32, stones: u32) -> Vec<Line<'static>> {
    let text_style = Style::default().fg(colors::WHITE);
    vec![
        Line::from(vec![
            Span::styled(blacksmith.name.to_string(), Style::default().color(colors::ORANGE)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
            Span::styled(format!("{}", gold), text_style),
            Span::styled("  |  ", text_style),
            Span::styled(format!("{} ", HAMMER), Style::default().color(colors::BLACK)),
            Span::styled(format!("{}", blacksmith.max_upgrades), text_style),
            Span::styled("  |  ", text_style),
            Span::styled(format!("{} ", DOUBLE_ARROW_UP), Style::default().color(colors::PURPLE)),
            Span::styled(format!("{}", stones), text_style),
        ]),
    ]
}

pub fn store_header(store: &Store, gold: i32) -> Vec<Line<'static>> {
    let secs = store.time_until_restock();
    let mins = secs / 60;
    let secs_remaining = secs % 60;
    let timer_text = if mins > 0 {
        format!("{}:{:02}", mins, secs_remaining)
    } else {
        format!("{}s", secs_remaining)
    };

    let text_style = Style::default().fg(colors::WHITE);
    vec![
        Line::from(vec![
            Span::styled(store.name.to_string(), text_style),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
            Span::styled(format!("{}", gold), text_style),
            Span::styled("  |  ", text_style),
            Span::styled(format!("{} ", HOURGLASS), Style::default().color(colors::CYAN)),
            Span::styled(timer_text, text_style),
        ]),
    ]
}

pub fn back_button(back_screen: Id) -> Menu {
    Menu::new(vec![
        MenuItem {
            label: format!("{} Back", RETURN_ARROW),
            action: Box::new(move || {
                game_state().current_screen = back_screen;
            }),
        },
    ])
}

/// Renders a location header with a themed background and colored separator line.
/// Returns the remaining area below the header for content rendering.
///
/// # Arguments
/// * `frame` - The frame to render to
/// * `area` - The area to render in
/// * `header_lines` - The header content (name, gold info, etc.)
/// * `header_bg` - The background color for the header area
/// * `accent_color` - The theme color for the separator line
pub fn render_location_header(
    frame: &mut Frame,
    area: Rect,
    header_lines: Vec<Line<'static>>,
    header_bg: Color,
    accent_color: Color,
) -> Rect {
    let header_height = header_lines.len() as u16;
    let separator_height = 1;
    let total_header_height = header_height + separator_height;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(total_header_height),
            Constraint::Min(0),
        ])
        .split(area);

    // Split header area into content and separator
    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(header_height),
            Constraint::Length(separator_height),
        ])
        .split(chunks[0]);

    // Render header with themed background
    let header_block = Block::default()
        .style(Style::default().bg(header_bg));
    let header_paragraph = Paragraph::new(header_lines).block(header_block);
    frame.render_widget(header_paragraph, header_chunks[0]);

    // Render separator line
    let separator_char = '─';
    let separator_line = Line::from(vec![
        Span::styled(
            separator_char.to_string().repeat(area.width as usize),
            Style::default().fg(accent_color),
        ),
    ]);
    frame.render_widget(Paragraph::new(separator_line), header_chunks[1]);

    // Return the remaining area for content
    chunks[1]
}

/// Collects all player items (equipped + inventory).
pub fn collect_player_items() -> Vec<InventoryItem> {
    let player = &game_state().player;
    let mut items = Vec::new();

    // Add equipped items first
    for slot in EquipmentSlot::all() {
        if let Some(inv_item) = player.get_equipped_item(*slot) {
            items.push(inv_item.clone());
        }
    }

    // Add inventory items
    for inv_item in player.get_inventory_items() {
        items.push(inv_item.clone());
    }

    items
}

/// Collects player equipment items (equipped + inventory equipment only).
pub fn collect_player_equipment() -> Vec<InventoryItem> {
    let player = &game_state().player;
    let mut items = Vec::new();

    // Add equipped items
    for slot in EquipmentSlot::all() {
        if let Some(inv_item) = player.get_equipped_item(*slot) {
            items.push(inv_item.clone());
        }
    }

    // Add inventory items (equipment only - materials can't be upgraded)
    for inv_item in player.get_inventory_items().iter() {
        if inv_item.item.item_type.is_equipment() {
            items.push(inv_item.clone());
        }
    }

    items
}

/// Move selection up in a list with wrapping.
pub fn list_move_up(list_state: &mut ListState, item_count: usize) {
    if item_count == 0 {
        return;
    }
    let current = list_state.selected().unwrap_or(0);
    let new_idx = if current == 0 { item_count - 1 } else { current - 1 };
    list_state.select(Some(new_idx));
}

/// Move selection down in a list with wrapping.
pub fn list_move_down(list_state: &mut ListState, item_count: usize) {
    if item_count == 0 {
        return;
    }
    let current = list_state.selected().unwrap_or(0);
    let new_idx = if current >= item_count - 1 { 0 } else { current + 1 };
    list_state.select(Some(new_idx));
}
