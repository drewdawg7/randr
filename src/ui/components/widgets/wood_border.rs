use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme as colors;

/// Generate the top wooden plank border line with cycling wood colors
/// Pattern: =#=[=]= (wooden beam with nails and joints)
pub fn generate_top_border(width: u16) -> Line<'static> {
    const TOP_PATTERN: &str = "=#=[=]=";
    let wood_colors = [
        colors::DARK_WALNUT,
        colors::WOOD_BROWN,
        colors::OAK_BROWN,
        colors::TAN_WOOD,
        colors::LIGHT_BEIGE,
        colors::CREAM_WOOD,
        colors::LIGHT_BEIGE,
        colors::TAN_WOOD,
        colors::OAK_BROWN,
    ];

    let spans: Vec<Span> = TOP_PATTERN
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = wood_colors[i % wood_colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

/// Generate the bottom wooden plank border line with cycling wood colors
/// Pattern: ]_|_[_|_ (wooden floor planks with gaps)
pub fn generate_bottom_border(width: u16) -> Line<'static> {
    const BOTTOM_PATTERN: &str = "]_|_[_|_";
    let wood_colors = [
        colors::WOOD_BROWN,
        colors::DARK_WALNUT,
        colors::OAK_BROWN,
        colors::WOOD_BROWN,
        colors::TAN_WOOD,
        colors::OAK_BROWN,
        colors::DARK_WALNUT,
    ];

    let spans: Vec<Span> = BOTTOM_PATTERN
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = wood_colors[i % wood_colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

/// Generate a single character for the left border at a given row
/// Pattern: [|#| (wood grain and planks)
pub fn generate_left_border_char(row: u16) -> Span<'static> {
    const LEFT_PATTERN: &[char] = &['[', '|', '#', '|'];
    let wood_colors = [
        colors::DARK_WALNUT,
        colors::WOOD_BROWN,
        colors::OAK_BROWN,
        colors::TAN_WOOD,
        colors::LIGHT_BEIGE,
        colors::OAK_BROWN,
    ];

    let ch = LEFT_PATTERN[row as usize % LEFT_PATTERN.len()];
    let color = wood_colors[row as usize % wood_colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}

/// Generate a single character for the right border at a given row
/// Pattern: ]|#| (wood grain and planks, mirrored)
pub fn generate_right_border_char(row: u16) -> Span<'static> {
    const RIGHT_PATTERN: &[char] = &[']', '|', '#', '|'];
    let wood_colors = [
        colors::DARK_WALNUT,
        colors::WOOD_BROWN,
        colors::OAK_BROWN,
        colors::TAN_WOOD,
        colors::LIGHT_BEIGE,
        colors::OAK_BROWN,
    ];

    let ch = RIGHT_PATTERN[row as usize % RIGHT_PATTERN.len()];
    let color = wood_colors[row as usize % wood_colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}
