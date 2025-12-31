use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme as colors;

/// Generate the top stone border line with cycling grey colors
/// Pattern: #^.~# (rocky ceiling with stalactites)
pub fn generate_top_border(width: u16) -> Line<'static> {
    const TOP_PATTERN: &str = "#^.~#";
    let stone_colors = [
        colors::DEEP_SLATE,
        colors::DARK_STONE,
        colors::GRANITE,
        colors::LIGHT_STONE,
        colors::PALE_ROCK,
        colors::LIGHT_STONE,
        colors::GRANITE,
        colors::DARK_STONE,
    ];

    let spans: Vec<Span> = TOP_PATTERN
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = stone_colors[i % stone_colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

/// Generate the bottom stone border line with cycling grey colors
/// Pattern: _.-~. (cave floor with rubble)
pub fn generate_bottom_border(width: u16) -> Line<'static> {
    const BOTTOM_PATTERN: &str = "_.-~.";
    let stone_colors = [
        colors::DARK_STONE,
        colors::DEEP_SLATE,
        colors::GRANITE,
        colors::DARK_STONE,
        colors::LIGHT_STONE,
        colors::GRANITE,
        colors::DEEP_SLATE,
    ];

    let spans: Vec<Span> = BOTTOM_PATTERN
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = stone_colors[i % stone_colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

/// Generate a single character for the left border at a given row
/// Pattern: [{|#: (rough stone wall)
pub fn generate_left_border_char(row: u16) -> Span<'static> {
    const LEFT_PATTERN: &[char] = &['[', '{', '|', '#', ':'];
    let stone_colors = [
        colors::DEEP_SLATE,
        colors::DARK_STONE,
        colors::GRANITE,
        colors::LIGHT_STONE,
        colors::PALE_ROCK,
        colors::LIGHT_STONE,
        colors::GRANITE,
        colors::DARK_STONE,
    ];

    let ch = LEFT_PATTERN[row as usize % LEFT_PATTERN.len()];
    let color = stone_colors[row as usize % stone_colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}

/// Generate a single character for the right border at a given row
/// Pattern: ]}|#: (rough stone wall, mirrored)
pub fn generate_right_border_char(row: u16) -> Span<'static> {
    const RIGHT_PATTERN: &[char] = &[']', '}', '|', '#', ':'];
    let stone_colors = [
        colors::DEEP_SLATE,
        colors::DARK_STONE,
        colors::GRANITE,
        colors::LIGHT_STONE,
        colors::PALE_ROCK,
        colors::LIGHT_STONE,
        colors::GRANITE,
        colors::DARK_STONE,
    ];

    let ch = RIGHT_PATTERN[row as usize % RIGHT_PATTERN.len()];
    let color = stone_colors[row as usize % stone_colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}
