use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::screens::dungeon::state::{DungeonMode, DungeonSelectionState};
use crate::states::AppState;
use crate::stats::HasStats;
use crate::ui::widgets::PlayerStats;
use crate::ui::{nav_selection_text, update_menu_colors, MenuIndex};

/// Component marker for boss UI root
#[derive(Component)]
pub struct BossRoot;

/// Component marker for boss actions.
/// Use with `MenuIndex` for selection tracking.
#[derive(Component)]
pub struct BossAction {
    pub action: BossActionType,
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
                        let selected = i == 0;

                        parent.spawn((
                            BossAction {
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

/// Handle boss input
pub fn handle_boss_input(
    mut action_reader: EventReader<GameAction>,
    mut selection_state: ResMut<DungeonSelectionState>,
    mut next_mode: ResMut<NextState<DungeonMode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut combat_source: ResMut<crate::game::CombatSourceResource>,
    boss_actions: Query<(&MenuIndex, &BossAction)>,
    mut items: Query<(&MenuIndex, &mut TextColor), With<BossAction>>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                selection_state.move_up();
                update_menu_colors::<BossAction>(selection_state.selected_action, &mut items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                selection_state.move_down();
                update_menu_colors::<BossAction>(selection_state.selected_action, &mut items);
            }
            GameAction::Select => {
                // Find selected action
                for (menu_index, boss_action) in boss_actions.iter() {
                    if menu_index.0 == selection_state.selected_action {
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

/// Despawn boss UI
pub fn despawn_boss_ui(mut commands: Commands, boss_root: Query<Entity, With<BossRoot>>) {
    if let Ok(entity) = boss_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
