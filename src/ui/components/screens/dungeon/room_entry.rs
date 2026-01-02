//! Room entry state for dungeon screen.
//!
//! Handles the initial interaction when entering a new room.

use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{
    commands::{apply_result, execute, GameCommand},
    dungeon::RoomType,
    system::game_state,
    ui::{
        components::utilities::selection_prefix,
        theme as colors,
    },
};

use super::{centered_rect, DungeonState};

/// Render the room entry UI.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
) {
    let gs = game_state();
    let text_style = Style::default().fg(colors::WHITE);

    let (room_type_name, action_text) = if let Some(dungeon) = gs.dungeon() {
        if let Some(room) = dungeon.current_room() {
            let type_name = match room.room_type {
                RoomType::Monster => "Monster Room",
                RoomType::Boss => "Boss Room",
                RoomType::Chest => "Treasure Chest",
                RoomType::Rest => "Rest Area",
                RoomType::Trap => "Trap Room",
                RoomType::Treasure => "Treasure Room",
            };
            let action = match room.room_type {
                RoomType::Monster | RoomType::Boss => "Fight",
                RoomType::Chest | RoomType::Treasure => "Open",
                _ => "Proceed",
            };
            (type_name, action)
        } else {
            ("Unknown", "Proceed")
        }
    } else {
        ("Unknown", "Proceed")
    };

    // Center the content
    let content_area = centered_rect(30, 8, area);

    // Room type display
    let room_info = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            room_type_name,
            Style::default().fg(colors::YELLOW),
        )]),
        Line::from(""),
    ])
    .centered();

    let info_area = Rect {
        height: 2,
        ..content_area
    };
    frame.render_widget(room_info, info_area);

    // Action menu
    let selected = list_state.selected().unwrap_or(0);
    let menu_items: Vec<ListItem> = vec![ListItem::new(Line::from(vec![
        selection_prefix(selected == 0),
        Span::styled(action_text, text_style),
    ]))];

    let menu = List::new(menu_items);
    let menu_area = Rect {
        y: content_area.y + 3,
        height: 1,
        ..content_area
    };
    frame.render_stateful_widget(menu, menu_area, list_state);
}

/// Handle room entry submit action.
/// Returns the new state to transition to, or None if screen changed.
pub fn handle_submit() -> Option<DungeonState> {
    let gs = game_state();

    // First, check if room is cleared and get room type
    let (is_cleared, room_type) = {
        if let Some(dungeon) = gs.dungeon() {
            if let Some(room) = dungeon.current_room() {
                (room.is_cleared, Some(room.room_type))
            } else {
                (false, None)
            }
        } else {
            (false, None)
        }
    };

    if is_cleared {
        // Rest rooms go to RestRoom state, others go to Navigation
        if room_type == Some(RoomType::Rest) {
            return Some(DungeonState::RestRoom);
        } else {
            return Some(DungeonState::Navigation);
        }
    }

    let Some(room_type) = room_type else {
        return None;
    };

    // Execute the enter room command (handles combat, chest, etc.)
    let result = execute(GameCommand::EnterRoom);
    apply_result(&result);

    // If screen changed (e.g., to Fight), return None
    if result.screen_change.is_some() {
        return None;
    }

    // Determine UI state based on room type
    match room_type {
        RoomType::Monster => {
            // Monster room starts combat - screen change handled by command
            None
        }
        RoomType::Boss => {
            // Boss room - transition to BossRoom state
            Some(DungeonState::BossRoom)
        }
        _ => {
            // Other room types go to Navigation after being cleared
            Some(DungeonState::Navigation)
        }
    }
}
