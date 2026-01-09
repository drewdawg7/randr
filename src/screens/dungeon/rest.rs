use bevy::prelude::*;

use crate::game::Player;
use crate::input::{GameAction, NavigationDirection};
use crate::screens::dungeon::state::{DungeonScreenState, DungeonViewMode};
use crate::stats::{HasStats, Healable};

/// Component marker for rest UI root
#[derive(Component)]
pub struct RestRoot;

/// Component marker for rest actions
#[derive(Component)]
pub struct RestAction {
    pub action: RestActionType,
    pub index: usize,
}

/// Types of actions in the rest screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestActionType {
    Rest,
    Leave,
}

/// Spawn the rest UI
pub fn spawn_rest_ui(
    mut commands: Commands,
    player: &Player,
    dungeon: &crate::game::DungeonResource,
    state: &mut DungeonScreenState,
) {
    let current_room = dungeon.current_room();
    let has_healed = current_room.map(|r| r.has_healed).unwrap_or(false);

    let actions = if has_healed {
        vec![RestActionType::Leave]
    } else {
        vec![RestActionType::Rest, RestActionType::Leave]
    };

    let action_count = actions.len();

    // Root container
    commands
        .spawn((
            RestRoot,
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
                Text::new("Rest Area"),
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

            // Description
            parent.spawn((
                Text::new("A peaceful sanctuary where you can recover."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // HP display
            parent.spawn((
                Text::new(format!(
                    "Health: {} / {}",
                    player.hp(),
                    player.max_hp()
                )),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Status message
            if has_healed {
                parent.spawn((
                    Text::new("You have already rested in this room."),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.0)),
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
                            RestActionType::Rest => "Rest (Heal to Full HP)",
                            RestActionType::Leave => "Leave",
                        };

                        parent.spawn((
                            RestAction {
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

    // Update state
    state.set_mode(DungeonViewMode::Rest, action_count);
}

/// Handle rest input
pub fn handle_rest_input(
    mut action_reader: EventReader<GameAction>,
    mut state: ResMut<DungeonScreenState>,
    mut player: ResMut<Player>,
    mut dungeon: ResMut<crate::game::DungeonResource>,
    rest_actions: Query<&RestAction>,
    mut items: Query<(&RestAction, &mut TextColor)>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                state.move_up();
                update_rest_visuals(&state, &mut items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                state.move_down();
                update_rest_visuals(&state, &mut items);
            }
            GameAction::Select => {
                // Find selected action
                for rest_action in rest_actions.iter() {
                    if rest_action.index == state.selected_action {
                        match rest_action.action {
                            RestActionType::Rest => {
                                // Heal player to full
                                let heal_amount = player.max_hp();
                                player.heal(heal_amount);

                                // Mark room as healed
                                if let Some(room) = dungeon.current_room_mut() {
                                    room.has_healed = true;
                                }

                                // Return to navigation
                                state.set_mode(DungeonViewMode::Navigation, 0);
                            }
                            RestActionType::Leave => {
                                // Return to navigation
                                state.set_mode(DungeonViewMode::Navigation, 0);
                            }
                        }
                        break;
                    }
                }
            }
            GameAction::Back => {
                // Return to navigation
                state.set_mode(DungeonViewMode::Navigation, 0);
            }
            _ => {}
        }
    }
}

/// Update visual highlighting for rest actions
fn update_rest_visuals(
    state: &DungeonScreenState,
    items: &mut Query<(&RestAction, &mut TextColor)>,
) {
    for (action, mut color) in items.iter_mut() {
        if action.index == state.selected_action {
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0)); // White
        } else {
            *color = TextColor(Color::srgb(0.7, 0.7, 0.7)); // Gray
        }
    }
}

/// Reset rest state when entering
pub fn reset_rest_state(mut items: Query<(&RestAction, &mut TextColor)>) {
    for (action, mut color) in items.iter_mut() {
        if action.index == 0 {
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0)); // First item selected
        } else {
            *color = TextColor(Color::srgb(0.7, 0.7, 0.7));
        }
    }
}

/// Despawn rest UI
pub fn despawn_rest_ui(mut commands: Commands, rest_root: Query<Entity, With<RestRoot>>) {
    if let Ok(entity) = rest_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
