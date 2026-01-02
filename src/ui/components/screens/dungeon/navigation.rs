//! Navigation state for dungeon screen.
//!
//! Handles compass-based directional navigation between rooms.

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use tuirealm::command::Direction as CmdDirection;

use crate::{
    commands::{apply_result, execute, GameCommand},
    dungeon::Direction,
    system::game_state,
    ui::{
        components::utilities::{selection_prefix, RETURN_ARROW},
        theme as colors,
    },
};

use super::{centered_rect, CompassPosition, DungeonState};

/// Render the navigation compass UI.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    compass_selection: CompassPosition,
) {
    let gs = game_state();

    let available = if let Some(dungeon) = gs.dungeon() {
        dungeon.available_directions()
    } else {
        vec![]
    };

    let has_north = available.contains(&Direction::North);
    let has_south = available.contains(&Direction::South);
    let has_east = available.contains(&Direction::East);
    let has_west = available.contains(&Direction::West);

    // Compass layout dimensions
    const BUTTON_WIDTH: u16 = 16;
    const BUTTON_HEIGHT: u16 = 1;
    const COMPASS_WIDTH: u16 = BUTTON_WIDTH * 3 + 4; // 3 columns + spacing
    const COMPASS_HEIGHT: u16 = 7; // title + north + middle row + south + padding

    // Center the compass
    let content_area = centered_rect(COMPASS_WIDTH, COMPASS_HEIGHT, area);

    // Navigation title
    let title = Paragraph::new(Line::from(vec![Span::styled(
        "Choose Direction",
        Style::default().fg(colors::CYAN),
    )]))
    .centered();

    let title_area = Rect {
        height: 1,
        ..content_area
    };
    frame.render_widget(title, title_area);

    // Compass grid: 3 rows (North, West-Center-East, South)
    let compass_y = content_area.y + 2;

    // Row 1: North (centered)
    if has_north {
        let north_area = Rect {
            x: content_area.x + BUTTON_WIDTH + 2,
            y: compass_y,
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
        };
        render_compass_button(frame, north_area, "North", CompassPosition::North, compass_selection);
    }

    // Row 2: West - Center - East
    let middle_y = compass_y + 2;

    if has_west {
        let west_area = Rect {
            x: content_area.x,
            y: middle_y,
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
        };
        render_compass_button(frame, west_area, "West", CompassPosition::West, compass_selection);
    }

    // Center (Leave Dungeon) - always available
    let center_area = Rect {
        x: content_area.x + BUTTON_WIDTH + 2,
        y: middle_y,
        width: BUTTON_WIDTH,
        height: BUTTON_HEIGHT,
    };
    render_leave_button(frame, center_area, compass_selection);

    if has_east {
        let east_area = Rect {
            x: content_area.x + (BUTTON_WIDTH + 2) * 2,
            y: middle_y,
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
        };
        render_compass_button(frame, east_area, "East", CompassPosition::East, compass_selection);
    }

    // Row 3: South (centered)
    if has_south {
        let south_area = Rect {
            x: content_area.x + BUTTON_WIDTH + 2,
            y: middle_y + 2,
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
        };
        render_compass_button(frame, south_area, "South", CompassPosition::South, compass_selection);
    }
}

fn render_compass_button(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    position: CompassPosition,
    selected: CompassPosition,
) {
    let gs = game_state();
    let is_selected = selected == position;

    // Check if room is cleared
    let direction = match position {
        CompassPosition::North => Direction::North,
        CompassPosition::East => Direction::East,
        CompassPosition::South => Direction::South,
        CompassPosition::West => Direction::West,
        CompassPosition::Center => return, // Should not happen
    };

    let is_cleared = if let Some(dungeon) = gs.dungeon() {
        let (dx, dy) = direction.offset();
        let (px, py) = dungeon.player_position;
        dungeon
            .get_room(px + dx, py + dy)
            .map(|r| r.is_cleared)
            .unwrap_or(false)
    } else {
        false
    };

    let text = if is_cleared {
        format!("{} ✓", label)
    } else {
        label.to_string()
    };

    let style = if is_selected {
        Style::default().fg(colors::YELLOW)
    } else {
        Style::default().fg(colors::WHITE)
    };

    let line = Line::from(vec![selection_prefix(is_selected), Span::styled(text, style)]);
    let paragraph = Paragraph::new(line).centered();
    frame.render_widget(paragraph, area);
}

fn render_leave_button(frame: &mut Frame, area: Rect, compass_selection: CompassPosition) {
    let is_selected = compass_selection == CompassPosition::Center;

    let style = if is_selected {
        Style::default().fg(colors::YELLOW)
    } else {
        Style::default().fg(colors::WHITE)
    };

    let line = Line::from(vec![
        selection_prefix(is_selected),
        Span::styled(format!("{} Leave", RETURN_ARROW), style),
    ]);
    let paragraph = Paragraph::new(line).centered();
    frame.render_widget(paragraph, area);
}

/// Handle navigation submit action.
pub fn handle_submit(compass_selection: CompassPosition) -> Option<DungeonState> {
    match compass_selection {
        CompassPosition::Center => {
            let result = execute(GameCommand::LeaveDungeon);
            apply_result(&result);
            None // Screen will change
        }
        CompassPosition::North => try_move(Direction::North),
        CompassPosition::East => try_move(Direction::East),
        CompassPosition::South => try_move(Direction::South),
        CompassPosition::West => try_move(Direction::West),
    }
}

/// Try to move in a direction, returns new state if successful.
fn try_move(direction: Direction) -> Option<DungeonState> {
    let result = execute(GameCommand::MoveDungeon { direction });
    apply_result(&result);

    // After moving, check if we entered a boss room
    let gs = game_state();
    if let Some(dungeon) = gs.dungeon() {
        let is_boss_room = dungeon
            .current_room()
            .map(|r| r.room_type == crate::dungeon::RoomType::Boss && !r.is_cleared)
            .unwrap_or(false);

        if is_boss_room {
            return Some(DungeonState::BossRoom);
        } else {
            return Some(DungeonState::RoomEntry);
        }
    }
    None
}

/// Navigate compass selection based on arrow key.
pub fn compass_move(current: CompassPosition, cmd_dir: CmdDirection) -> CompassPosition {
    let gs = game_state();
    let available = if let Some(dungeon) = gs.dungeon() {
        dungeon.available_directions()
    } else {
        vec![]
    };

    let has_north = available.contains(&Direction::North);
    let has_south = available.contains(&Direction::South);
    let has_east = available.contains(&Direction::East);
    let has_west = available.contains(&Direction::West);

    match (current, cmd_dir) {
        // From Center
        (CompassPosition::Center, CmdDirection::Up) if has_north => CompassPosition::North,
        (CompassPosition::Center, CmdDirection::Down) if has_south => CompassPosition::South,
        (CompassPosition::Center, CmdDirection::Left) if has_west => CompassPosition::West,
        (CompassPosition::Center, CmdDirection::Right) if has_east => CompassPosition::East,

        // From North
        (CompassPosition::North, CmdDirection::Down) => CompassPosition::Center,
        (CompassPosition::North, CmdDirection::Left) if has_west => CompassPosition::West,
        (CompassPosition::North, CmdDirection::Right) if has_east => CompassPosition::East,

        // From South
        (CompassPosition::South, CmdDirection::Up) => CompassPosition::Center,
        (CompassPosition::South, CmdDirection::Left) if has_west => CompassPosition::West,
        (CompassPosition::South, CmdDirection::Right) if has_east => CompassPosition::East,

        // From West
        (CompassPosition::West, CmdDirection::Right) => CompassPosition::Center,
        (CompassPosition::West, CmdDirection::Up) if has_north => CompassPosition::North,
        (CompassPosition::West, CmdDirection::Down) if has_south => CompassPosition::South,

        // From East
        (CompassPosition::East, CmdDirection::Left) => CompassPosition::Center,
        (CompassPosition::East, CmdDirection::Up) if has_north => CompassPosition::North,
        (CompassPosition::East, CmdDirection::Down) if has_south => CompassPosition::South,

        // No change for invalid moves
        (current, _) => current,
    }
}
