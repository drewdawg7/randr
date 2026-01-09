use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::screens::dungeon::state::{DungeonMode, DungeonSelectionState};
use crate::states::AppState;
use crate::stats::HasStats;
use crate::ui::widgets::PlayerStats;

/// Component marker for boss UI root
#[derive(Component)]
pub struct BossRoot;

/// Component marker for boss actions
#[derive(Component)]
pub struct BossAction {
    pub action: BossActionType,
    pub index: usize,
}

/// Types of actions in the boss screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BossActionType {
    ChallengeBoss,
    Retreat,
}

/// Spawn the boss UI
pub fn spawn_boss_ui(
    mut commands: Commands,
    dungeon: Res<crate::game::DungeonResource>,
    mut selection_state: ResMut<DungeonSelectionState>,
) {
    let boss_info = if let Some(boss) = &dungeon.boss {
        format!(
            "{}\nHP: {} | Attack: {} | Defense: {}",
            boss.name, boss.hp(), boss.attack(), boss.defense()
        )
    } else {
        "Unknown Boss".to_string()
    };

    let actions = vec![BossActionType::ChallengeBoss, BossActionType::Retreat];
    let action_count = actions.len();

    // Root container
    commands
        .spawn((
            BossRoot,
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
                Text::new("Boss Chamber"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)), // Red for boss
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Warning
            parent.spawn((
                Text::new("A powerful boss awaits!"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Boss info
            parent.spawn((
                Text::new(boss_info),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Player stats
            parent.spawn(PlayerStats);

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
                            BossActionType::ChallengeBoss => "Challenge Boss",
                            BossActionType::Retreat => "Retreat",
                        };

                        parent.spawn((
                            BossAction {
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

/// Handle boss input
pub fn handle_boss_input(
    mut action_reader: EventReader<GameAction>,
    mut selection_state: ResMut<DungeonSelectionState>,
    mut next_mode: ResMut<NextState<DungeonMode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut combat_source: ResMut<crate::game::CombatSourceResource>,
    boss_actions: Query<&BossAction>,
    mut items: Query<(&BossAction, &mut TextColor)>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                selection_state.move_up();
                update_boss_visuals(&selection_state, &mut items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                selection_state.move_down();
                update_boss_visuals(&selection_state, &mut items);
            }
            GameAction::Select => {
                // Find selected action
                for boss_action in boss_actions.iter() {
                    if boss_action.index == selection_state.selected_action {
                        match boss_action.action {
                            BossActionType::ChallengeBoss => {
                                // Set combat source to DungeonBoss and transition to Fight
                                combat_source.set_dungeon_boss();
                                next_app_state.set(AppState::Fight);
                            }
                            BossActionType::Retreat => {
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

/// Update visual highlighting for boss actions
fn update_boss_visuals(
    selection_state: &DungeonSelectionState,
    items: &mut Query<(&BossAction, &mut TextColor)>,
) {
    for (action, mut color) in items.iter_mut() {
        if action.index == selection_state.selected_action {
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0)); // White
        } else {
            *color = TextColor(Color::srgb(0.7, 0.7, 0.7)); // Gray
        }
    }
}

/// Despawn boss UI
pub fn despawn_boss_ui(mut commands: Commands, boss_root: Query<Entity, With<BossRoot>>) {
    if let Ok(entity) = boss_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
