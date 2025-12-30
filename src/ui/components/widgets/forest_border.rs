use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme as colors;

/// Generate the top forest border line with cycling green colors
/// Pattern: *^*v (flowers, peaks, flowers, valleys)
pub fn generate_top_border(width: u16) -> Line<'static> {
    const TOP_PATTERN: &str = "*^*v";
    let forest_colors = [
        colors::DARK_FOREST,
        colors::FOREST_GREEN,
        colors::GREEN,
        colors::LIME_GREEN,
        colors::PALE_GREEN,
    ];

    let spans: Vec<Span> = TOP_PATTERN
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = forest_colors[i % forest_colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

/// Generate the bottom grass border line with cycling green colors
/// Pattern: vV (short grass, tall grass)
pub fn generate_bottom_border(width: u16) -> Line<'static> {
    const BOTTOM_PATTERN: &str = "vV";
    let forest_colors = [
        colors::DARK_FOREST,
        colors::FOREST_GREEN,
        colors::GREEN,
        colors::LIME_GREEN,
        colors::PALE_GREEN,
    ];

    let spans: Vec<Span> = BOTTOM_PATTERN
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = forest_colors[i % forest_colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

/// Generate a single character for the left border at a given row
/// Pattern: }{|: (vines and tree trunk)
pub fn generate_left_border_char(row: u16) -> Span<'static> {
    const LEFT_PATTERN: &[char] = &['}', '{', '|', ':'];
    let forest_colors = [
        colors::DARK_FOREST,
        colors::FOREST_GREEN,
        colors::GREEN,
        colors::LIME_GREEN,
        colors::PALE_GREEN,
    ];

    let ch = LEFT_PATTERN[row as usize % LEFT_PATTERN.len()];
    let color = forest_colors[row as usize % forest_colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}

/// Generate a single character for the right border at a given row
/// Pattern: {|}; (vines and tree trunk, mirrored)
pub fn generate_right_border_char(row: u16) -> Span<'static> {
    const RIGHT_PATTERN: &[char] = &['{', '}', '|', ';'];
    let forest_colors = [
        colors::DARK_FOREST,
        colors::FOREST_GREEN,
        colors::GREEN,
        colors::LIME_GREEN,
        colors::PALE_GREEN,
    ];

    let ch = RIGHT_PATTERN[row as usize % RIGHT_PATTERN.len()];
    let color = forest_colors[row as usize % forest_colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}
