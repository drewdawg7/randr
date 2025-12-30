use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme as colors;

/// Generate the top flame border line with cycling ember colors
/// Pattern: ~'^"~ (flame flickering upward)
pub fn generate_top_border(width: u16) -> Line<'static> {
    const TOP_PATTERN: &str = "~'^\"~";
    let ember_colors = [
        colors::COAL_BLACK,
        colors::EMBER_RED,
        colors::DEEP_ORANGE,
        colors::FLAME_ORANGE,
        colors::BRIGHT_YELLOW,
        colors::HOT_WHITE,
        colors::BRIGHT_YELLOW,
        colors::FLAME_ORANGE,
        colors::DEEP_ORANGE,
        colors::EMBER_RED,
    ];

    let spans: Vec<Span> = TOP_PATTERN
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = ember_colors[i % ember_colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

/// Generate the bottom ember/coal border line with cycling ember colors
/// Pattern: .o@O# (glowing coals and embers)
pub fn generate_bottom_border(width: u16) -> Line<'static> {
    const BOTTOM_PATTERN: &str = ".o@O#";
    let ember_colors = [
        colors::ASH_GRAY,
        colors::COAL_BLACK,
        colors::EMBER_RED,
        colors::DEEP_ORANGE,
        colors::FLAME_ORANGE,
        colors::EMBER_RED,
        colors::ASH_GRAY,
    ];

    let spans: Vec<Span> = BOTTOM_PATTERN
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = ember_colors[i % ember_colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

/// Generate a single character for the left border at a given row
/// Pattern: !|*:' (rising flames and sparks)
pub fn generate_left_border_char(row: u16) -> Span<'static> {
    const LEFT_PATTERN: &[char] = &['!', '|', '*', ':', '\''];
    let ember_colors = [
        colors::COAL_BLACK,
        colors::EMBER_RED,
        colors::DEEP_ORANGE,
        colors::FLAME_ORANGE,
        colors::BRIGHT_YELLOW,
        colors::HOT_WHITE,
        colors::BRIGHT_YELLOW,
        colors::FLAME_ORANGE,
    ];

    let ch = LEFT_PATTERN[row as usize % LEFT_PATTERN.len()];
    let color = ember_colors[row as usize % ember_colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}

/// Generate a single character for the right border at a given row
/// Pattern: ':|*! (rising flames and sparks, mirrored)
pub fn generate_right_border_char(row: u16) -> Span<'static> {
    const RIGHT_PATTERN: &[char] = &['\'', ':', '|', '*', '!'];
    let ember_colors = [
        colors::COAL_BLACK,
        colors::EMBER_RED,
        colors::DEEP_ORANGE,
        colors::FLAME_ORANGE,
        colors::BRIGHT_YELLOW,
        colors::HOT_WHITE,
        colors::BRIGHT_YELLOW,
        colors::FLAME_ORANGE,
    ];

    let ch = RIGHT_PATTERN[row as usize % RIGHT_PATTERN.len()];
    let color = ember_colors[row as usize % ember_colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}
