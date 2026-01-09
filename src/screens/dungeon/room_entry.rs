use bevy::prelude::*;

use crate::dungeon::RoomType;
use crate::input::{GameAction, NavigationDirection};
use crate::screens::dungeon::state::{DungeonMode, DungeonSelectionState};
use crate::states::AppState;

/// Component marker for room entry UI root
#[derive(Component)]
pub struct RoomEntryRoot;

/// Component marker for room action items
#[derive(Component)]
pub struct RoomAction {
    pub action: RoomActionType,
    pub index: usize,
}

/// Types of actions available in a room
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomActionType {
    Fight,
    Loot,
    Rest,
    ChallengeBoss,
    Leave,
}

/// Spawn the room entry UI
pub fn spawn_room_entry_ui(
    mut commands: Commands,
    dungeon: Res<crate::game::DungeonResource>,
    mut selection_state: ResMut<DungeonSelectionState>,
    mut next_mode: ResMut<NextState<DungeonMode>>,
) {
    let current_room = dungeon.current_room();
    if current_room.is_none() {
        // No room to enter, return to navigation
        next_mode.set(DungeonMode::Navigation);
        return;
    }

    let room = current_room.unwrap();
    let (room_name, room_desc, actions) = match room.room_type {
        RoomType::Monster => (
            "Monster Room",
            "A dark chamber filled with hostile creatures.",
            if room.is_cleared {
                vec![RoomActionType::Leave]
            } else {
                vec![RoomActionType::Fight, RoomActionType::Leave]
            },
        ),
        RoomType::Boss => (
            "Boss Chamber",
            "An ominous chamber where a powerful boss awaits.",
            if room.is_cleared {
                vec![RoomActionType::Leave]
            } else {
                vec![RoomActionType::ChallengeBoss, RoomActionType::Leave]
            },
        ),
        RoomType::Rest => (
            "Rest Area",
            "A peaceful sanctuary where you can recover your strength.",
            vec![RoomActionType::Rest, RoomActionType::Leave],
        ),
        RoomType::Trap => (
            "Trap Room",
            "A dangerous room filled with deadly traps.",
            vec![RoomActionType::Fight, RoomActionType::Leave], // Treat like monster room
        ),
        RoomType::Chest | RoomType::Treasure => (
            "Treasure Room",
            "A room containing valuable loot.",
            if room.chest.is_some() {
                vec![RoomActionType::Loot, RoomActionType::Leave]
            } else {
                vec![RoomActionType::Leave] // Already looted
            },
        ),
    };

    let action_count = actions.len();

    // Root container
    commands
        .spawn((
            RoomEntryRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Room title
            parent.spawn((
                Text::new(room_name),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Room description
            parent.spawn((
                Text::new(room_desc),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Room status
            if room.is_cleared {
                parent.spawn((
                    Text::new("[CLEARED]"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 1.0, 0.0)),
                    Node {
                        margin: UiRect::bottom(Val::Px(30.0)),
                        ..default()
                    },
                ));
            }

            // Actions
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    ..default()
                })
                .with_children(|parent| {
                    for (i, action_type) in actions.iter().enumerate() {
                        let label = match action_type {
                            RoomActionType::Fight => "Fight",
                            RoomActionType::Loot => "Loot Chest",
                            RoomActionType::Rest => "Rest",
                            RoomActionType::ChallengeBoss => "Challenge Boss",
                            RoomActionType::Leave => "Leave Room",
                        };

                        parent.spawn((
                            RoomAction {
                                action: *action_type,
                                index: i,
                            },
                            Text::new(label),
                            TextFont {
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        ));
                    }
                });
        });

    // Update selection state
    selection_state.reset(action_count);
}

/// Handle room entry input
pub fn handle_room_entry_input(
    mut action_reader: EventReader<GameAction>,
    mut selection_state: ResMut<DungeonSelectionState>,
    mut next_mode: ResMut<NextState<DungeonMode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut combat_source: ResMut<crate::game::CombatSourceResource>,
    mut dungeon: ResMut<crate::game::DungeonResource>,
    room_actions: Query<&RoomAction>,
    mut items: Query<(&RoomAction, &mut TextColor)>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                selection_state.move_up();
                update_room_entry_visuals(&selection_state, &mut items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                selection_state.move_down();
                update_room_entry_visuals(&selection_state, &mut items);
            }
            GameAction::Select => {
                // Find selected action
                for room_action in room_actions.iter() {
                    if room_action.index == selection_state.selected_action {
                        match room_action.action {
                            RoomActionType::Fight => {
                                // Set combat source and transition to Fight
                                combat_source.set_dungeon();
                                next_app_state.set(AppState::Fight);
                            }
                            RoomActionType::Loot => {
                                // TODO: Open loot chest (requires inventory system)
                                // For now, mark room as cleared and return to navigation
                                if let Some(room) = dungeon.current_room_mut() {
                                    room.is_cleared = true;
                                }
                                next_mode.set(DungeonMode::Navigation);
                            }
                            RoomActionType::Rest => {
                                // Switch to Rest mode
                                next_mode.set(DungeonMode::Rest);
                            }
                            RoomActionType::ChallengeBoss => {
                                // Switch to Boss mode
                                next_mode.set(DungeonMode::Boss);
                            }
                            RoomActionType::Leave => {
                                // Return to Navigation mode
                                next_mode.set(DungeonMode::Navigation);
                            }
                        }
                        break;
                    }
                }
            }
            GameAction::Back => {
                // Return to navigation
                next_mode.set(DungeonMode::Navigation);
            }
            _ => {}
        }
    }
}

/// Update visual highlighting for room actions
fn update_room_entry_visuals(
    selection_state: &DungeonSelectionState,
    items: &mut Query<(&RoomAction, &mut TextColor)>,
) {
    for (action, mut color) in items.iter_mut() {
        if action.index == selection_state.selected_action {
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0)); // White
        } else {
            *color = TextColor(Color::srgb(0.7, 0.7, 0.7)); // Gray
        }
    }
}

/// Despawn room entry UI
pub fn despawn_room_entry_ui(
    mut commands: Commands,
    room_entry_root: Query<Entity, With<RoomEntryRoot>>,
) {
    if let Ok(entity) = room_entry_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
