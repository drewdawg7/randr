use bevy::prelude::*;

use crate::input::{clear_game_action_events, GameAction, NavigationDirection};
use crate::states::{AppState, RequestFightEvent, RequestMineEvent};

use super::super::shared::{spawn_menu, MenuOption};
use super::super::{ContentArea, TabContent, TownTab};

/// Plugin for the Field tab.
pub struct FieldTabPlugin;

impl Plugin for FieldTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FieldTabState>()
            .add_systems(OnExit(AppState::Town), clear_game_action_events)
            .add_systems(OnEnter(TownTab::Field), spawn_field_content)
            .add_systems(
                Update,
                (handle_field_input, refresh_field_on_state_change)
                    .run_if(in_state(TownTab::Field)),
            );
    }
}

/// Spawns field UI content when entering the Field tab.
fn spawn_field_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    field_state: Res<FieldTabState>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_field_ui(&mut commands, content_entity, &field_state);
}

/// Refreshes field UI when state changes.
fn refresh_field_on_state_change(
    mut commands: Commands,
    field_state: Res<FieldTabState>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
) {
    if !field_state.is_changed() {
        return;
    }

    // Despawn existing content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Respawn with new state
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_field_ui(&mut commands, content_entity, &field_state);
}

/// Field tab state - just tracks menu selection.
#[derive(Resource, Default)]
pub struct FieldTabState {
    pub selected_index: usize,
}

const FIELD_OPTIONS: &[MenuOption] = &[
    MenuOption {
        label: "Fight",
        description: Some("Battle a random enemy"),
    },
    MenuOption {
        label: "Mine",
        description: Some("Enter the mines"),
    },
];

/// Handle input for the Field tab.
fn handle_field_input(
    mut field_state: ResMut<FieldTabState>,
    mut action_events: EventReader<GameAction>,
    mut fight_events: EventWriter<RequestFightEvent>,
    mut mine_events: EventWriter<RequestMineEvent>,
) {
    for action in action_events.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                if field_state.selected_index > 0 {
                    field_state.selected_index -= 1;
                } else {
                    field_state.selected_index = FIELD_OPTIONS.len() - 1;
                }
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                field_state.selected_index = (field_state.selected_index + 1) % FIELD_OPTIONS.len();
            }
            GameAction::Select => match field_state.selected_index {
                0 => {
                    fight_events.send(RequestFightEvent);
                }
                1 => {
                    mine_events.send(RequestMineEvent);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

/// Spawn the field UI.
pub fn spawn_field_ui(
    commands: &mut Commands,
    content_entity: Entity,
    field_state: &FieldTabState,
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
                // Menu options
                spawn_menu(
                    content,
                    FIELD_OPTIONS,
                    field_state.selected_index,
                    Some("Field"),
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
