use bevy::prelude::*;

use crate::input::GameAction;
use crate::states::AppState;
use crate::ui::widgets::PlayerStats;

use super::components::{ContentArea, TabContent, TabHeaderItem, TownUiRoot};
use super::state::TownTab;

pub fn setup_town_ui(mut commands: Commands, current_tab: Res<State<TownTab>>) {
    commands
        .spawn((
            TownUiRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Player stats banner at top
            parent.spawn(PlayerStats);

            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
                ))
                .with_children(|header| {
                    for tab in TownTab::all() {
                        spawn_tab_header_item(header, tab, tab == *current_tab.get());
                    }
                });

            parent.spawn((
                ContentArea,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.12, 0.12)),
            ));
        });
}

fn spawn_tab_header_item(parent: &mut ChildBuilder, tab: TownTab, is_active: bool) {
    let bg_color = if is_active {
        Color::srgb(0.4, 0.4, 0.8)
    } else {
        Color::srgb(0.2, 0.2, 0.2)
    };

    parent
        .spawn((
            TabHeaderItem { tab },
            Node {
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(bg_color),
        ))
        .with_children(|item| {
            item.spawn((
                Text::new(tab.name()),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn cleanup_town_ui(mut commands: Commands, query: Query<Entity, With<TownUiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn handle_tab_navigation(
    current_tab: Res<State<TownTab>>,
    mut next_tab: ResMut<NextState<TownTab>>,
    mut action_events: EventReader<GameAction>,
) {
    for action in action_events.read() {
        match action {
            GameAction::NextTab => {
                next_tab.set(current_tab.get().next());
            }
            GameAction::PrevTab => {
                next_tab.set(current_tab.get().prev());
            }
            _ => {}
        }
    }
}

pub fn handle_back_action(
    mut action_events: EventReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            next_state.set(AppState::Menu);
        }
    }
}

/// Updates the visual appearance of tab headers based on the current tab state.
pub fn update_tab_header_visuals(
    current_tab: Res<State<TownTab>>,
    mut tab_query: Query<(&TabHeaderItem, &mut BackgroundColor)>,
) {
    for (tab_item, mut bg_color) in tab_query.iter_mut() {
        if tab_item.tab == *current_tab.get() {
            *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.8));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
        }
    }
}

/// Cleans up tab content when exiting a tab. Used as OnExit system for each TownTab.
pub fn cleanup_tab_content(mut commands: Commands, tab_content_query: Query<Entity, With<TabContent>>) {
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }
}
