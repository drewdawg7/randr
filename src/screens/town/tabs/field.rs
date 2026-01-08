use bevy::prelude::*;

use crate::entities::Progression;
use crate::game::PlayerResource;
use crate::input::{GameAction, NavigationDirection};
use crate::states::AppState;
use crate::stats::HasStats;

use super::super::shared::{spawn_menu, MenuOption};
use super::super::{ContentArea, CurrentTab, TabContent, TownTab};

/// Plugin for the Field tab.
pub struct FieldTabPlugin;

impl Plugin for FieldTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FieldTabState>().add_systems(
            Update,
            (
                handle_field_input,
                render_field_content.run_if(resource_changed::<FieldTabState>),
                render_field_on_tab_change.run_if(resource_changed::<CurrentTab>),
            )
                .run_if(in_state(AppState::Town))
                .run_if(|tab: Res<CurrentTab>| tab.tab == TownTab::Field),
        );
    }
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
    mut next_state: ResMut<NextState<AppState>>,
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
                0 => next_state.set(AppState::Fight),
                1 => next_state.set(AppState::Mine),
                _ => {}
            },
            _ => {}
        }
    }
}

/// Render field content when tab is changed to Field.
fn render_field_on_tab_change(
    mut commands: Commands,
    current_tab: Res<CurrentTab>,
    field_state: Res<FieldTabState>,
    player: Res<PlayerResource>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
) {
    if current_tab.tab != TownTab::Field {
        return;
    }

    // Despawn existing tab content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Get content area
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    spawn_field_ui(&mut commands, content_entity, &field_state, &player);
}

/// Render field content when state changes.
fn render_field_content(
    mut commands: Commands,
    field_state: Res<FieldTabState>,
    player: Res<PlayerResource>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
) {
    // Despawn existing tab content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Get content area
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    spawn_field_ui(&mut commands, content_entity, &field_state, &player);
}

/// Spawn the field UI.
fn spawn_field_ui(
    commands: &mut Commands,
    content_entity: Entity,
    field_state: &FieldTabState,
    player: &PlayerResource,
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
                // Player stats summary
                spawn_player_stats(content, player);

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

/// Spawn player stats display.
fn spawn_player_stats(parent: &mut ChildBuilder, player: &PlayerResource) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(5.0),
            ..default()
        },))
        .with_children(|stats| {
            // HP
            stats.spawn((
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

            // Level & XP
            stats.spawn((
                Text::new(format!(
                    "Level: {}  XP: {}/{}",
                    player.prog.level,
                    player.prog.xp,
                    Progression::xp_to_next_level(player.prog.level)
                )),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
            ));

            // Gold
            stats.spawn((
                Text::new(format!("Gold: {}", player.gold)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.3)),
            ));
        });
}
