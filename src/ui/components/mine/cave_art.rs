use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::location::mine::{CaveLayout, RockType, CAVE_HEIGHT, CAVE_WIDTH};
use crate::ui::theme as colors;

/// Rock symbol (Nerd Font)
const ROCK_SYMBOL: char = '\u{e88a}';

/// Player symbol (Nerd Font)
const PLAYER_SYMBOL: char = '\u{f183}';

/// Pickaxe symbol (Nerd Font) - shown when adjacent to rock
const PICKAXE_SYMBOL: char = '\u{F08B7}';

/// Exit ladder symbol (Nerd Font)
const LADDER_SYMBOL: char = '\u{F15A2}';

/// Exit indicator arrow (Nerd Font) - shown when on ladder
const EXIT_ARROW_SYMBOL: char = '\u{F062}';

impl RockType {
    /// Get the color for this rock type
    fn color(&self) -> Color {
        match self {
            RockType::Copper => colors::COPPER_ORE,
            RockType::Coal => colors::COAL_ORE,
            RockType::Tin => colors::TIN_ORE,
        }
    }
}

/// Get style for wall character based on depth
fn char_style(ch: char) -> Style {
    match ch {
        '#' => Style::default().fg(colors::CAVE_WALL_DARK),
        '@' => Style::default().fg(colors::CAVE_WALL_MID),
        '%' => Style::default().fg(colors::CAVE_WALL_LIGHT),
        ';' => Style::default().fg(colors::CAVE_FLOOR_EDGE),
        _ => Style::default(),
    }
}

/// Convert cave layout to renderable lines
fn cave_to_lines(cave: &CaveLayout) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    for y in 0..CAVE_HEIGHT {
        let mut spans = Vec::new();
        let current_char = cave.cell_to_char(0, y);
        let mut current_style = char_style(current_char);
        let mut current_text = String::new();
        current_text.push(current_char);

        for x in 1..CAVE_WIDTH {
            let ch = cave.cell_to_char(x, y);
            let style = char_style(ch);

            if style != current_style {
                spans.push(Span::styled(current_text.clone(), current_style));
                current_text.clear();
                current_style = style;
            }
            current_text.push(ch);
        }

        if !current_text.is_empty() {
            spans.push(Span::styled(current_text, current_style));
        }

        lines.push(Line::from(spans));
    }

    lines
}

/// Renders a cave layout centered in the given area
pub fn render_cave(frame: &mut Frame, area: Rect, cave: &CaveLayout) {
    // Calculate centering offsets
    let x_offset = area.x + area.width.saturating_sub(CAVE_WIDTH as u16) / 2;
    let y_offset = area.y + area.height.saturating_sub(CAVE_HEIGHT as u16) / 2;

    let cave_area = Rect {
        x: x_offset,
        y: y_offset,
        width: CAVE_WIDTH as u16,
        height: CAVE_HEIGHT as u16,
    };

    let bg_style = Style::default().bg(colors::CAVE_FLOOR_BG);
    let lines = cave_to_lines(cave);

    frame.render_widget(Paragraph::new(lines).style(bg_style), cave_area);

    // Render exit ladder
    let buf = frame.buffer_mut();
    let exit_screen_x = x_offset + cave.exit_x as u16;
    let exit_screen_y = y_offset + cave.exit_y as u16;
    if let Some(cell) = buf.cell_mut((exit_screen_x, exit_screen_y)) {
        cell.set_char(LADDER_SYMBOL);
        cell.set_fg(colors::WOOD_BROWN);
    }

    // Render rocks on top
    for rock in &cave.rocks {
        let screen_x = x_offset + rock.x as u16;
        let screen_y = y_offset + rock.y as u16;

        if let Some(cell) = buf.cell_mut((screen_x, screen_y)) {
            cell.set_char(ROCK_SYMBOL);
            cell.set_fg(rock.rock_type.color());
        }
    }

    // Render player
    let player_screen_x = x_offset + cave.player_x as u16;
    let player_screen_y = y_offset + cave.player_y as u16;
    if let Some(cell) = buf.cell_mut((player_screen_x, player_screen_y)) {
        cell.set_char(PLAYER_SYMBOL);
        cell.set_fg(colors::WHITE);
    }

    // Render indicator above player
    if player_screen_y > 0 {
        let indicator_y = player_screen_y - 1;
        if cave.is_on_exit() {
            // Exit arrow when on ladder
            if let Some(cell) = buf.cell_mut((player_screen_x, indicator_y)) {
                cell.set_char(EXIT_ARROW_SYMBOL);
                cell.set_fg(colors::YELLOW);
            }
        } else if cave.is_adjacent_to_rock() {
            // Pickaxe when adjacent to rock
            if let Some(cell) = buf.cell_mut((player_screen_x, indicator_y)) {
                cell.set_char(PICKAXE_SYMBOL);
                cell.set_fg(colors::YELLOW);
            }
        }
    }
}
