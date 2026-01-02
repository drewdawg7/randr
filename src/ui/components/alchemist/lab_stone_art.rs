use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::theme as colors;

/// Laboratory stone pattern - dark stone with mystical purple tint
/// Similar structure to blacksmith stone wall but with alchemist colors
const LAB_PATTERN: &[&str] = &[
    "⠒⠂⠤⠀⠀⠂⠀⠀⠤⠀⠐⠚⠂⠒⢲⠒⠒⠒⠓⠒⠒⡖⠐⠒⠒⠀⠀⠠⠀⠀",
    "⠒⠒⠒⠒⠒⡖⠒⠒⠓⠀⠀⣤⠄⠀⠘⠒⠒⢶⡖⠖⠖⠓⠒⠒⢶⠒⠒⠒⠒⠒",
    "⠤⠤⡤⠤⠤⠷⠤⢤⡤⠤⠤⠼⠤⠤⢬⡤⠮⠴⠃⠁⠰⡤⠄⠐⠚⠂⠐⠦⠄⠀",
    "⠤⠤⡧⡤⠤⡤⠠⠤⠧⠤⠤⢤⠤⠤⠼⠧⠤⠤⡦⠤⠤⠳⠰⠰⠤⠤⠤⡐⠂⠀",
    "⣀⣠⣄⣱⣀⣇⣀⣀⣀⣀⣀⣼⣀⣀⣀⣠⠀⢠⡇⠀⠠⣤⠢⠤⠬⠤⠤⢧⡄⠀",
    "⣀⣀⣅⣀⣀⣀⣐⣀⣋⣀⣀⣀⣀⣀⣀⣇⣀⣀⣰⣀⣀⣷⣄⣀⣀⣀⠤⢼⠧⠤",
    "⠀⠀⢀⢀⢀⣏⠀⠀⠀⠀⢀⢸⢀⡀⢀⣄⠀⣀⣏⣰⣈⣀⡀⠀⣸⣀⠀⠀⡀⠀",
    "⠀⠘⡏⠁⠀⠀⠀⠀⢹⠉⠀⠀⠙⠈⢙⡧⠀⢀⡀⠀⠀⢸⢀⣀⣀⣀⣀⣀⣁⠀",
    "⠀⠈⠉⠉⠉⡟⠁⠋⠙⠁⠉⢻⠙⠉⠛⠛⠉⠩⡏⠉⠉⠉⠭⠉⠉⡇⠀⠀⠀⠀",
    "⠀⠐⡖⠒⠒⠛⠒⠒⢲⠒⠒⠚⠒⠒⠒⠶⠒⠒⠛⠂⠂⢰⠂⠚⠙⠓⠂⠐⡖⠐",
];

/// Pattern width in characters (each braille char is 1 terminal column)
const PATTERN_WIDTH: usize = 30;
const PATTERN_HEIGHT: usize = 10;

/// Generates a single line of tiled laboratory stone pattern with color variation
fn generate_lab_line(row_in_pattern: usize, width: usize) -> Line<'static> {
    let dark = Style::default().fg(colors::DEEP_VIOLET);
    let mid = Style::default().fg(colors::DARK_PURPLE);
    let light = Style::default().fg(colors::MYSTIC_PURPLE);

    let pattern_row = LAB_PATTERN[row_in_pattern % PATTERN_HEIGHT];
    let pattern_chars: Vec<char> = pattern_row.chars().collect();

    let mut spans = Vec::new();
    let mut current_style = dark;
    let mut current_text = String::new();

    for col in 0..width {
        // Get character from tiled pattern
        let pattern_col = col % PATTERN_WIDTH;
        let ch = pattern_chars.get(pattern_col).copied().unwrap_or(' ');

        // Vary color based on position for depth effect
        let new_style = match ((col / 5) + (row_in_pattern / 2)) % 3 {
            0 => dark,
            1 => mid,
            _ => light,
        };

        // If style changed, push current span and start new one
        if new_style != current_style && !current_text.is_empty() {
            spans.push(Span::styled(current_text.clone(), current_style));
            current_text.clear();
        }
        current_style = new_style;
        current_text.push(ch);
    }

    // Push final span
    if !current_text.is_empty() {
        spans.push(Span::styled(current_text, current_style));
    }

    Line::from(spans)
}

/// Renders laboratory stone pattern filling the entire area.
/// The pattern tiles to fill whatever space is available.
pub fn render_lab_stone(frame: &mut Frame, area: Rect) {
    if area.height == 0 || area.width == 0 {
        return;
    }

    // Generate lines to fill the entire area
    let mut lines: Vec<Line<'static>> = Vec::new();

    for row in 0..area.height {
        let line = generate_lab_line(row as usize, area.width as usize);
        lines.push(line);
    }

    frame.render_widget(Paragraph::new(lines), area);
}
