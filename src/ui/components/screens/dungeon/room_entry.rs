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
    dungeon::{Explorable, RoomType},
    magic::effect::PassiveEffect,
    system::game_state,
    ui::{
        components::utilities::selection_prefix,
        theme as colors,
    },
};

use super::{centered_rect, DungeonState};

/// Check if player has DungeonBypass passive effect
fn has_dungeon_bypass() -> bool {
    let gs = game_state();
    gs.player
        .tome_passive_effects()
        .iter()
        .any(|e| matches!(e, PassiveEffect::DungeonBypass))
}

/// Check if the current room can be bypassed (Monster room + has DungeonBypass)
pub fn can_bypass_current_room() -> bool {
    let gs = game_state();
    if let Some(dungeon) = gs.dungeon() {
        if let Some(room) = dungeon.current_room() {
            return matches!(room.room_type, RoomType::Monster) && has_dungeon_bypass();
        }
    }
    false
}

/// Render the room entry UI.
pub fn render(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
) {
    let gs = game_state();
    let text_style = Style::default().fg(colors::WHITE);

    let (room_type_name, room_type, action_text) = if let Some(dungeon) = gs.dungeon() {
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
            (type_name, Some(room.room_type), action)
        } else {
            ("Unknown", None, "Proceed")
        }
    } else {
        ("Unknown", None, "Proceed")
    };

    // Check if bypass is available (Monster room + has DungeonBypass)
    let can_bypass = matches!(room_type, Some(RoomType::Monster)) && has_dungeon_bypass();

    // Center the content
    let menu_height = if can_bypass { 10 } else { 8 };
    let content_area = centered_rect(30, menu_height, area);

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
    let mut menu_items: Vec<ListItem> = vec![ListItem::new(Line::from(vec![
        selection_prefix(selected == 0),
        Span::styled(action_text, text_style),
    ]))];

    // Add bypass option if available
    if can_bypass {
        menu_items.push(ListItem::new(Line::from(vec![
            selection_prefix(selected == 1),
            Span::styled("Bypass (Shadow Step)", Style::default().fg(colors::MYSTIC_PURPLE)),
        ])));
    }

    let menu = List::new(menu_items);
    let menu_area = Rect {
        y: content_area.y + 3,
        height: if can_bypass { 2 } else { 1 },
        ..content_area
    };
    frame.render_stateful_widget(menu, menu_area, list_state);
}

/// Handle room entry submit action.
/// Returns the new state to transition to, or None if screen changed.
/// `selected` is the menu option selected (0 = primary action, 1 = bypass if available)
pub fn handle_submit(selected: usize) -> Option<DungeonState> {
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

    // Check if bypass was selected (option 1) and is available
    let can_bypass = matches!(room_type, RoomType::Monster) && has_dungeon_bypass();
    if selected == 1 && can_bypass {
        // Bypass the room - mark it as cleared without combat
        if let Some(dungeon) = gs.dungeon_mut() {
            if let Some(room) = dungeon.current_room_mut() {
                room.clear();
            }
        }
        return Some(DungeonState::Navigation);
    }

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
