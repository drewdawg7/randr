use bevy::prelude::*;

use crate::dungeon::{Direction, Dungeon, RoomType};
use crate::ui::nav_selection_text;
use crate::input::{GameAction, NavigationDirection};
use crate::screens::dungeon::state::{DungeonScreenState, DungeonViewMode};

/// Component marker for dungeon navigation UI root
#[derive(Component)]
pub struct NavigationRoot;

/// Component marker for compass direction buttons
#[derive(Component)]
pub struct CompassDirection {
    pub direction: Direction,
    pub index: usize,
}

/// Component marker for the "Enter Room" action
#[derive(Component)]
pub struct EnterRoomAction {
    pub index: usize,
}

/// System to spawn the navigation UI (compass + minimap)
pub fn spawn_navigation_ui(
    mut commands: Commands,
    dungeon: Res<crate::game::DungeonResource>,
) {
    let available_dirs = dungeon.available_directions();
    let action_count = available_dirs.len() + 1; // directions + "Enter Room"

    // Root container
    commands
        .spawn((
            NavigationRoot,
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
            // Title
            parent.spawn((
                Text::new("Dungeon Navigation"),
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

            // Current room info
            if let Some(room) = dungeon.current_room() {
                let room_desc = match room.room_type {
                    RoomType::Monster => "Monster Room",
                    RoomType::Boss => "Boss Room",
                    RoomType::Rest => "Rest Area",
                    RoomType::Trap => "Trap Room",
                    RoomType::Chest => "Treasure Room",
                    RoomType::Treasure => "Loot Room",
                };
                parent.spawn((
                    Text::new(format!("Current: {}", room_desc)),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    Node {
                        margin: UiRect::bottom(Val::Px(30.0)),
                        ..default()
                    },
                ));
            }

            // Minimap
            spawn_minimap(parent, &dungeon);

            // Compass
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Movement:"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));

                    // Spawn compass directions
                    for (i, dir) in available_dirs.iter().enumerate() {
                        parent.spawn((
                            CompassDirection {
                                direction: *dir,
                                index: i,
                            },
                            Text::new(dir.name()),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                        ));
                    }

                    // Enter Room action
                    parent.spawn((
                        EnterRoomAction {
                            index: available_dirs.len(),
                        },
                        Text::new("Enter Room"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });
        });

    // Store action count in state
    commands.insert_resource(DungeonScreenState {
        mode: DungeonViewMode::Navigation,
        selected_action: 0,
        action_count,
    });
}

/// Spawn the minimap UI showing room states
fn spawn_minimap(parent: &mut ChildBuilder, dungeon: &Dungeon) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(2.0),
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Minimap:"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Render the dungeon grid
            for (y, row) in dungeon.rooms.iter().enumerate() {
                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(2.0),
                        ..default()
                    })
                    .with_children(|parent| {
                        for (x, room_opt) in row.iter().enumerate() {
                            let (symbol, color) = if let Some(room) = room_opt {
                                let is_current = dungeon.player_position == (x as i32, y as i32);
                                if is_current {
                                    ("@", Color::srgb(0.0, 1.0, 0.0)) // Player = green
                                } else if !room.is_revealed {
                                    ("?", Color::srgb(0.3, 0.3, 0.3)) // Unrevealed = dark gray
                                } else if room.room_type == RoomType::Boss {
                                    ("B", Color::srgb(1.0, 0.0, 0.0)) // Boss = red
                                } else if room.is_cleared {
                                    (".", Color::srgb(0.5, 0.5, 0.5)) // Cleared = gray
                                } else {
                                    ("#", Color::srgb(0.8, 0.8, 0.8)) // Uncleared = white
                                }
                            } else {
                                (" ", Color::srgb(0.1, 0.1, 0.1)) // No room = background
                            };

                            parent.spawn((
                                Text::new(symbol),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(color),
                            ));
                        }
                    });
            }
        });
}

/// Handle navigation input
pub fn handle_navigation_input(
    mut action_reader: EventReader<GameAction>,
    mut state: ResMut<DungeonScreenState>,
    mut dungeon: ResMut<crate::game::DungeonResource>,
    compass_dirs: Query<&CompassDirection, Without<EnterRoomAction>>,
    enter_action: Query<&EnterRoomAction, Without<CompassDirection>>,
    mut all_items: Query<
        (&mut TextColor, Option<&CompassDirection>, Option<&EnterRoomAction>),
        Or<(With<CompassDirection>, With<EnterRoomAction>)>,
    >,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                state.move_up();
                update_navigation_visuals(&state, &mut all_items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                state.move_down();
                update_navigation_visuals(&state, &mut all_items);
            }
            GameAction::Select => {
                // Check if selecting a direction
                let mut moved = false;
                for compass in compass_dirs.iter() {
                    if compass.index == state.selected_action {
                        // Move player
                        if dungeon.move_player(compass.direction).is_ok() {
                            moved = true;
                        }
                        break;
                    }
                }

                // Check if selecting "Enter Room"
                if !moved {
                    for enter in enter_action.iter() {
                        if enter.index == state.selected_action {
                            // Switch to RoomEntry mode
                            state.set_mode(DungeonViewMode::RoomEntry, 0);
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Update visual highlighting for navigation options
fn update_navigation_visuals(
    state: &DungeonScreenState,
    items: &mut Query<
        (&mut TextColor, Option<&CompassDirection>, Option<&EnterRoomAction>),
        Or<(With<CompassDirection>, With<EnterRoomAction>)>,
    >,
) {
    for (mut color, compass_opt, enter_opt) in items.iter_mut() {
        let is_selected = if let Some(compass) = compass_opt {
            compass.index == state.selected_action
        } else if let Some(enter) = enter_opt {
            enter.index == state.selected_action
        } else {
            false
        };

        *color = TextColor(nav_selection_text(is_selected));
    }
}

/// Reset navigation state when entering
pub fn reset_navigation_state(
    mut state: ResMut<DungeonScreenState>,
    dungeon: Res<crate::game::DungeonResource>,
) {
    let available_dirs = dungeon.available_directions();
    state.set_mode(DungeonViewMode::Navigation, available_dirs.len() + 1);
}

/// Despawn navigation UI
pub fn despawn_navigation_ui(
    mut commands: Commands,
    navigation_root: Query<Entity, With<NavigationRoot>>,
) {
    if let Ok(entity) = navigation_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
