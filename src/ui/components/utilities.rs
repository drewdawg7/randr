#![allow(dead_code)]
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::item::Item;
use crate::ui::theme::{self as colors, upgrade_color, ColorExt};

use crate::{blacksmith::Blacksmith, store::Store, system::game_state, ui::Id};
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

/// Returns a styled prefix Span for list items. Selected items get a yellow ">", unselected get "  ".
pub fn selection_prefix(is_selected: bool) -> Span<'static> {
    if is_selected {
        Span::styled("> ", Style::default().color(colors::YELLOW))
    } else {
        Span::raw("  ")
    }
}

/// Returns a styled Span for an item name with upgrade count, colored by upgrade level.
/// Format: "{name} (+{num_upgrades})"
pub fn item_display(item: &Item) -> Span<'static> {
    let color = upgrade_color(item.num_upgrades);
    Span::styled(
        format!("{} (+{})", item.name, item.num_upgrades),
        Style::default().color(color),
    )
}

pub fn blacksmith_header(blacksmith: &Blacksmith, gold: i32) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled(blacksmith.name.to_string(), Style::default().color(colors::ORANGE)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
            Span::raw(format!("{}", gold)),
            Span::raw("  |  "),
            Span::styled(format!("{} ", HAMMER), Style::default().color(colors::BLACK)),
            Span::raw(format!("{}", blacksmith.max_upgrades)),
        ]),
    ]
}

pub fn store_header(store: &Store, gold: i32) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled(store.name.to_string(), Style::default().color(colors::WHITE)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
            Span::raw(format!("{}", gold)),
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
