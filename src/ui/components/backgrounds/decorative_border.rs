use ratatui::{
    layout::Rect,
    style::Style,
    Frame,
};

use crate::ui::theme as colors;

/// Corner and edge patterns (all 7 chars wide)
const CORNER_TOP: &str = ".-=~=-.";
const CORNER_BOTTOM: &str = "`-._.-'";

/// Side patterns that cycle vertically (each 7 chars)
const SIDE_PATTERNS: &[&str] = &[
    "(__  _)",
    "( _ __)",
    "(__  _)",
    "(_ ___)",
];

/// Horizontal bar pattern components
const HORIZ_PREFIX: &str = "-.";
const HORIZ_PATTERN: &str = "_.-=-.";
const HORIZ_SUFFIX: &str = "_.-";

const BORDER_WIDTH: u16 = 7;

/// Generates the horizontal bar pattern to fill a given width
fn generate_horizontal_bar(width: usize) -> String {
    if width < HORIZ_PREFIX.len() + HORIZ_SUFFIX.len() {
        return "-".repeat(width);
    }

    let middle_width = width - HORIZ_PREFIX.len() - HORIZ_SUFFIX.len();
    let pattern_count = middle_width / HORIZ_PATTERN.len();
    let remainder = middle_width % HORIZ_PATTERN.len();

    let mut result = String::from(HORIZ_PREFIX);
    for _ in 0..pattern_count {
        result.push_str(HORIZ_PATTERN);
    }
    // Add partial pattern for remaining chars
    if remainder > 0 {
        result.push_str(&HORIZ_PATTERN[..remainder]);
    }
    result.push_str(HORIZ_SUFFIX);

    result
}

/// Renders a decorative ASCII border that stretches with the screen.
/// The border is transparent where there are spaces (to show background through).
/// Uses direct buffer rendering to preserve the background.
pub fn render_decorative_border(frame: &mut Frame, area: Rect) {
    if area.height < 4 || area.width < (BORDER_WIDTH * 2) {
        return; // Too small for border
    }

    let width = area.width as usize;
    let height = area.height as usize;
    let inner_width = width.saturating_sub(BORDER_WIDTH as usize * 2);

    let border_style = Style::default().fg(colors::WHITE);
    let buf = frame.buffer_mut();

    // Row 0: Top corners
    render_line_to_buffer(buf, area.x, area.y, CORNER_TOP, &" ".repeat(inner_width), CORNER_TOP, border_style);

    // Row 1: Top horizontal bar with side patterns
    let horiz_bar = generate_horizontal_bar(inner_width);
    render_line_to_buffer(buf, area.x, area.y + 1, SIDE_PATTERNS[0], &horiz_bar, SIDE_PATTERNS[0], border_style);

    // Middle rows: Side patterns only (spaces in between, transparent to background)
    for row in 2..height.saturating_sub(2) {
        let pattern_idx = (row - 1) % SIDE_PATTERNS.len();
        let side = SIDE_PATTERNS[pattern_idx];
        render_sides_only(buf, area.x, area.y + row as u16, side, inner_width, side, border_style);
    }

    // Second to last row: Bottom horizontal bar
    if height > 3 {
        let bottom_bar_row = height - 2;
        let pattern_idx = (bottom_bar_row - 1) % SIDE_PATTERNS.len();
        let side = SIDE_PATTERNS[pattern_idx];
        render_line_to_buffer(buf, area.x, area.y + bottom_bar_row as u16, side, &horiz_bar, side, border_style);
    }

    // Last row: Bottom corners
    render_line_to_buffer(buf, area.x, area.y + (height - 1) as u16, CORNER_BOTTOM, &" ".repeat(inner_width), CORNER_BOTTOM, border_style);
}

/// Renders a full line with left, middle, and right sections
fn render_line_to_buffer(
    buf: &mut ratatui::buffer::Buffer,
    start_x: u16,
    y: u16,
    left: &str,
    middle: &str,
    right: &str,
    style: Style,
) {
    let mut x = start_x;

    // Render left section
    for ch in left.chars() {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(ch);
            if let Some(fg) = style.fg {
                cell.set_fg(fg);
            }
        }
        x += 1;
    }

    // Render middle section (skip spaces to preserve background)
    for ch in middle.chars() {
        if ch != ' ' {
            if let Some(cell) = buf.cell_mut((x, y)) {
                cell.set_char(ch);
                if let Some(fg) = style.fg {
                    cell.set_fg(fg);
                }
            }
        }
        x += 1;
    }

    // Render right section
    for ch in right.chars() {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(ch);
            if let Some(fg) = style.fg {
                cell.set_fg(fg);
            }
        }
        x += 1;
    }
}

/// Renders only the side patterns (left and right), leaving middle transparent
fn render_sides_only(
    buf: &mut ratatui::buffer::Buffer,
    start_x: u16,
    y: u16,
    left: &str,
    middle_width: usize,
    right: &str,
    style: Style,
) {
    let mut x = start_x;

    // Render left side
    for ch in left.chars() {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(ch);
            if let Some(fg) = style.fg {
                cell.set_fg(fg);
            }
        }
        x += 1;
    }

    // Skip middle (leave background visible)
    x += middle_width as u16;

    // Render right side
    for ch in right.chars() {
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_char(ch);
            if let Some(fg) = style.fg {
                cell.set_fg(fg);
            }
        }
        x += 1;
    }
}
