//! Minimap rendering for dungeon exploration
//!
//! Displays a fog-of-war minimap showing:
//! - Current room (highlighted with player icon)
//! - Visited rooms (with room type icons)
//! - Adjacent rooms (revealed but not visited)
//! - Empty spaces (no room exists)

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    dungeon::{Dungeon, RoomType, DUNGEON_SIZE},
    ui::theme as colors,
};

// Nerdfont icons for room types
const ICON_MONSTER: char = '\u{f0787}';   // Crossed swords
const ICON_BOSS: char = '\u{F0544}';      // Skull
const ICON_CHEST: char = '\u{f0726}';     // Treasure chest
const ICON_REST: char = '\u{F023E}';      // Campfire/bed
const ICON_TRAP: char = '\u{F0236}';      // Warning/spike
const ICON_TREASURE: char = '\u{F19D1}';  // Gem/diamond
const ICON_PLAYER: char = '\u{F415}';     // Person marker
const ICON_UNKNOWN: char = '?';           // Revealed but not visited room

// Colors for different room states
const COLOR_CURRENT: Color = colors::YELLOW;
const COLOR_CLEARED: Color = colors::GREEN;
const COLOR_VISITED: Color = colors::LIGHT_STONE;
const COLOR_ADJACENT: Color = colors::GRANITE;
const COLOR_EMPTY: Color = colors::DARK_STONE;
const COLOR_BORDER: Color = colors::GRANITE;

/// Cell width for each room in the minimap (wider for better icon display)
const CELL_WIDTH: u16 = 5;
/// Cell height for each room
const CELL_HEIGHT: u16 = 1;

/// Renders the minimap in the given area
pub fn render_minimap(frame: &mut Frame, area: Rect, dungeon: &Dungeon) {
    let (px, py) = dungeon.player_position;

    // Build lines for the minimap
    let mut lines: Vec<Line> = Vec::new();

    for y in 0..DUNGEON_SIZE {
        let mut spans: Vec<Span> = Vec::new();

        for x in 0..DUNGEON_SIZE {
            let cell = render_cell(dungeon, x as i32, y as i32, px, py);
            spans.push(cell);
        }

        lines.push(Line::from(spans));
    }

    let minimap = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(COLOR_BORDER))
                .title(" Map ")
                .title_style(Style::default().fg(colors::LIGHT_STONE))
        );

    frame.render_widget(minimap, area);
}

/// Render a single cell of the minimap
fn render_cell(dungeon: &Dungeon, x: i32, y: i32, player_x: i32, player_y: i32) -> Span<'static> {
    let is_current = x == player_x && y == player_y;

    match dungeon.get_room(x, y) {
        Some(room) => {
            // Room exists at this position
            if is_current {
                // Current room - show player icon with highlight
                Span::styled(
                    format!("[ {} ]", ICON_PLAYER),
                    Style::default().fg(COLOR_CURRENT),
                )
            } else if room.is_visited {
                // Visited room - show room type icon
                let (icon, color) = if room.is_cleared {
                    (room_type_icon(room.room_type), COLOR_CLEARED)
                } else {
                    (room_type_icon(room.room_type), COLOR_VISITED)
                };
                Span::styled(
                    format!("[ {} ]", icon),
                    Style::default().fg(color),
                )
            } else if room.is_revealed {
                // Revealed but not visited - show as unknown
                Span::styled(
                    format!("[ {} ]", ICON_UNKNOWN),
                    Style::default().fg(COLOR_ADJACENT),
                )
            } else {
                // Not revealed yet - show as fog
                Span::styled(
                    "  ·  ".to_string(),
                    Style::default().fg(COLOR_EMPTY),
                )
            }
        }
        None => {
            // No room at this position
            if has_adjacent_revealed_room(dungeon, x, y) {
                // Show empty space near revealed areas
                Span::styled(
                    "  ·  ".to_string(),
                    Style::default().fg(COLOR_EMPTY),
                )
            } else {
                // Fog of war - completely hidden
                Span::styled(
                    "     ".to_string(),
                    Style::default(),
                )
            }
        }
    }
}

/// Check if there's a revealed room adjacent to position (x, y)
fn has_adjacent_revealed_room(dungeon: &Dungeon, x: i32, y: i32) -> bool {
    let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for (dx, dy) in offsets {
        if let Some(room) = dungeon.get_room(x + dx, y + dy) {
            if room.is_revealed {
                return true;
            }
        }
    }
    false
}

/// Get the icon for a room type
fn room_type_icon(room_type: RoomType) -> char {
    match room_type {
        RoomType::Monster => ICON_MONSTER,
        RoomType::Boss => ICON_BOSS,
        RoomType::Chest => ICON_CHEST,
        RoomType::Rest => ICON_REST,
        RoomType::Trap => ICON_TRAP,
        RoomType::Treasure => ICON_TREASURE,
    }
}

/// Calculate the required size for the minimap
pub fn minimap_size() -> (u16, u16) {
    // Width: cells + border (2 chars)
    let width = (DUNGEON_SIZE as u16 * CELL_WIDTH) + 2;
    // Height: cells + border (2 chars)
    let height = (DUNGEON_SIZE as u16 * CELL_HEIGHT) + 2;
    (width, height)
}
