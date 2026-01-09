use bevy::prelude::*;

use crate::game::Player;
use crate::input::{GameAction, NavigationDirection};
use crate::states::AppState;
use crate::stats::HasStats;

use super::super::shared::{spawn_menu, MenuOption};
use super::super::{CurrentTab, TabContent, TownTab};

/// Plugin for the Dungeon tab.
pub struct DungeonTabPlugin;

impl Plugin for DungeonTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DungeonTabState>().add_systems(
            Update,
            handle_dungeon_input
                .run_if(in_state(AppState::Town))
                .run_if(|tab: Res<CurrentTab>| tab.tab == TownTab::Dungeon),
        );
    }
}

/// Dungeon tab state - tracks menu selection.
#[derive(Resource, Default)]
pub struct DungeonTabState {
    pub selected_index: usize,
}

const DUNGEON_OPTIONS: &[MenuOption] = &[MenuOption {
    label: "Enter Dungeon",
    description: Some("Descend into the depths"),
}];

/// Handle input for the Dungeon tab.
fn handle_dungeon_input(
    mut dungeon_state: ResMut<DungeonTabState>,
    mut action_events: EventReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_events.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                if dungeon_state.selected_index > 0 {
                    dungeon_state.selected_index -= 1;
                } else {
                    dungeon_state.selected_index = DUNGEON_OPTIONS.len() - 1;
                }
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                dungeon_state.selected_index =
                    (dungeon_state.selected_index + 1) % DUNGEON_OPTIONS.len();
            }
            GameAction::Select => match dungeon_state.selected_index {
                0 => next_state.set(AppState::Dungeon),
                _ => {}
            },
            _ => {}
        }
    }
}

/// Spawn the dungeon UI.
pub fn spawn_dungeon_ui(
    commands: &mut Commands,
    content_entity: Entity,
    dungeon_state: &DungeonTabState,
    player: &Player,
) {
    commands.entity(content_entity).with_children(|parent| {
        parent
            .spawn((
                TabContent,
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(20.0),
                    ..default()
                },
            ))
            .with_children(|content| {
                // Dungeon status
                spawn_dungeon_status(content, player);

                // Menu options
                spawn_menu(
                    content,
                    DUNGEON_OPTIONS,
                    dungeon_state.selected_index,
                    Some("Dungeon"),
                );

                // Navigation hint
                content.spawn((
                    Text::new("[↑↓] Navigate  [Enter] Select  [←→] Switch Tab"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    Node {
                        margin: UiRect::top(Val::Auto),
                        ..default()
                    },
                ));
            });
    });
}

/// Spawn dungeon status display.
fn spawn_dungeon_status(parent: &mut ChildBuilder, player: &Player) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(5.0),
            ..default()
        },))
        .with_children(|status| {
            // Player readiness
            status.spawn((
                Text::new(format!(
                    "HP: {}/{}",
                    player.hp(), player.max_hp()
                )),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.3, 0.3)),
            ));

            // Level
            status.spawn((
                Text::new(format!("Level: {}", player.prog.level)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
            ));

            // Dungeon info
            status.spawn((
                Text::new("The dungeon awaits..."),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}
