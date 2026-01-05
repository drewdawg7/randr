//! Location header rendering functions.
//!
//! These functions create header content for location screens (blacksmith, store, etc.)
//! and handle the common header layout rendering.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::location::{Blacksmith, Store};
use crate::ui::theme::{self as colors, icons, ColorExt};

/// Creates header lines for the blacksmith screen.
pub fn blacksmith_header(blacksmith: &Blacksmith, gold: i32, stones: u32) -> Vec<Line<'static>> {
    let text_style = Style::default().fg(colors::WHITE);
    vec![
        Line::from(vec![
            Span::styled(blacksmith.name.to_string(), Style::default().color(colors::ORANGE)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", icons::COIN), Style::default().color(colors::YELLOW)),
            Span::styled(format!("{}", gold), text_style),
            Span::styled("  |  ", text_style),
            Span::styled(format!("{} ", icons::HAMMER), Style::default().color(colors::BLACK)),
            Span::styled(format!("{}", blacksmith.max_upgrades), text_style),
            Span::styled("  |  ", text_style),
            Span::styled(format!("{} ", icons::DOUBLE_ARROW_UP), Style::default().color(colors::PURPLE)),
            Span::styled(format!("{}", stones), text_style),
        ]),
    ]
}

/// Creates header lines for the store screen.
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
            Span::styled(format!("{} ", icons::COIN), Style::default().color(colors::YELLOW)),
            Span::styled(format!("{}", gold), text_style),
            Span::styled("  |  ", text_style),
            Span::styled(format!("{} ", icons::HOURGLASS), Style::default().color(colors::CYAN)),
            Span::styled(timer_text, text_style),
        ]),
    ]
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
