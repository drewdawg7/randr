use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::theme as colors;

/// Stone wall pattern rows - these tile horizontally and vertically
/// Using the user's provided dense braille pattern
const WALL_PATTERN: &[&str] = &[
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

/// Generates a single line of tiled stone wall pattern with color variation
fn generate_wall_line(row_in_pattern: usize, width: usize) -> Line<'static> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    let pattern_row = WALL_PATTERN[row_in_pattern % PATTERN_HEIGHT];
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

/// Renders stone wall pattern filling the entire dead space area.
/// The pattern tiles to fill whatever space is available.
pub fn render_stone_patches(frame: &mut Frame, area: Rect, menu_item_count: u16) {
    let dead_space_start_y = area.y + menu_item_count;
    let dead_space_height = area.height.saturating_sub(menu_item_count);

    // Only render if we have dead space
    if dead_space_height == 0 || area.width == 0 {
        return;
    }

    // Generate lines to fill the entire dead space
    let mut lines: Vec<Line<'static>> = Vec::new();

    for row in 0..dead_space_height {
        let line = generate_wall_line(row as usize, area.width as usize);
        lines.push(line);
    }

    // Render the tiled wall pattern in the dead space
    let wall_rect = Rect {
        x: area.x,
        y: dead_space_start_y,
        width: area.width,
        height: dead_space_height,
    };

    frame.render_widget(Paragraph::new(lines), wall_rect);
}
