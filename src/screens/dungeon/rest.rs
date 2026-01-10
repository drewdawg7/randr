use bevy::prelude::*;

use crate::game::Player;
use crate::input::{GameAction, NavigationDirection};
use crate::screens::dungeon::state::{DungeonMode, DungeonSelectionState};
use crate::stats::{HasStats, Healable};
use crate::ui::widgets::PlayerStats;
use crate::ui::{nav_selection_text, update_menu_colors, MenuIndex};

/// Component marker for rest UI root
#[derive(Component)]
pub struct RestRoot;

/// Component marker for rest actions.
/// Use with `MenuIndex` for selection tracking.
#[derive(Component)]
pub struct RestAction {
    pub action: RestActionType,
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
    dungeon: Res<crate::game::DungeonResource>,
    mut selection_state: ResMut<DungeonSelectionState>,
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

            // Player stats
            parent.spawn(PlayerStats);

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
                        let selected = i == 0;

                        parent.spawn((
                            RestAction {
                                action: *action_type,
                            },
                            MenuIndex(i),
                            Text::new(label),
                            TextFont {
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(nav_selection_text(selected)),
                        ));
                    }
                });
        });

    // Update selection state
    selection_state.reset(action_count);
}

/// Handle rest input
pub fn handle_rest_input(
    mut action_reader: EventReader<GameAction>,
    mut selection_state: ResMut<DungeonSelectionState>,
    mut next_mode: ResMut<NextState<DungeonMode>>,
    mut player: ResMut<Player>,
    mut dungeon: ResMut<crate::game::DungeonResource>,
    rest_actions: Query<(&MenuIndex, &RestAction)>,
    mut items: Query<(&MenuIndex, &mut TextColor), With<RestAction>>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                selection_state.move_up();
                update_menu_colors::<RestAction>(selection_state.selected_action, &mut items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                selection_state.move_down();
                update_menu_colors::<RestAction>(selection_state.selected_action, &mut items);
            }
            GameAction::Select => {
                // Find selected action
                for (menu_index, rest_action) in rest_actions.iter() {
                    if menu_index.0 == selection_state.selected_action {
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
                                next_mode.set(DungeonMode::Navigation);
                            }
                            RestActionType::Leave => {
                                // Return to navigation
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

/// Despawn rest UI
pub fn despawn_rest_ui(mut commands: Commands, rest_root: Query<Entity, With<RestRoot>>) {
    if let Ok(entity) = rest_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
