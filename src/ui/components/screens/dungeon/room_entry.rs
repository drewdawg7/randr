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
    dungeon::RoomType,
    entities::mob::MobId,
    inventory::HasInventory,
    system::{game_state, CombatSource},
    ui::{
        components::utilities::selection_prefix,
        theme as colors,
        Id,
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

    match room_type {
        RoomType::Monster => {
            // Spawn a mob and start combat
            let mob_result = {
                if let Some(dungeon) = gs.dungeon() {
                    dungeon.spawn_mob()
                } else {
                    return None;
                }
            };

            match mob_result {
                Ok(mob) => {
                    gs.combat_source = CombatSource::Dungeon;
                    gs.start_combat(mob);
                    gs.current_screen = Id::Fight;
                    None // Screen changed
                }
                Err(_) => {
                    gs.toasts.error("No enemies to fight!");
                    None
                }
            }
        }
        RoomType::Boss => {
            // Boss room - spawn boss once and transition to BossRoom state (trapped!)
            let needs_spawn = gs.dungeon().map(|d| d.boss.is_none()).unwrap_or(false);
            if needs_spawn {
                // Spawn the dragon boss
                let dragon = gs.spawn_mob(MobId::Dragon);
                if let Some(dungeon) = gs.dungeon_mut() {
                    dungeon.boss = Some(dragon);
                }
            }
            Some(DungeonState::BossRoom)
        }
        RoomType::Chest => {
            // Open the chest and get loot
            let loot_drops = {
                if let Some(dungeon) = gs.dungeon_mut() {
                    if let Some(room) = dungeon.current_room_mut() {
                        let drops = room.open_chest();
                        room.clear();
                        drops
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            };

            // Now add items to inventory and show toasts
            for loot_drop in &loot_drops {
                for _ in 0..loot_drop.quantity {
                    let _ = gs.player.add_to_inv(loot_drop.item.clone());
                }
                gs.toasts.success(format!(
                    "Found: {} x{}",
                    loot_drop.item.name, loot_drop.quantity
                ));
            }
            if loot_drops.is_empty() {
                gs.toasts.info("The chest was empty.");
            }

            Some(DungeonState::Navigation)
        }
        _ => {
            // For other room types, just clear and move to navigation
            if let Some(dungeon) = gs.dungeon_mut() {
                if let Some(room) = dungeon.current_room_mut() {
                    room.clear();
                }
            }
            Some(DungeonState::Navigation)
        }
    }
}
