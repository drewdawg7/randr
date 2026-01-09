mod render;
mod shared;
mod tabs;

use bevy::prelude::*;

use crate::input::{GameAction, NavigationDirection};
use crate::states::AppState;

pub use tabs::TabsPlugin;
use render::{cleanup_tab_content, update_tab_header_visuals};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TabNavigationSet;

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<TownTab>()
            .add_plugins(TabsPlugin)
            .add_systems(OnEnter(AppState::Town), setup_town_ui)
            .add_systems(OnExit(AppState::Town), cleanup_town_ui)
            // Cleanup tab content when exiting any tab
            .add_systems(OnExit(TownTab::Store), cleanup_tab_content)
            .add_systems(OnExit(TownTab::Blacksmith), cleanup_tab_content)
            .add_systems(OnExit(TownTab::Alchemist), cleanup_tab_content)
            .add_systems(OnExit(TownTab::Field), cleanup_tab_content)
            .add_systems(OnExit(TownTab::Dungeon), cleanup_tab_content)
            .add_systems(
                Update,
                (handle_tab_navigation, handle_back_action, update_tab_header_visuals)
                    .in_set(TabNavigationSet)
                    .run_if(in_state(AppState::Town)),
            );
    }
}

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::Town)]
pub enum TownTab {
    #[default]
    Store,
    Blacksmith,
    Alchemist,
    Field,
    Dungeon,
}

impl TownTab {
    pub fn name(&self) -> &'static str {
        match self {
            TownTab::Store => "Store",
            TownTab::Blacksmith => "Blacksmith",
            TownTab::Alchemist => "Alchemist",
            TownTab::Field => "Field",
            TownTab::Dungeon => "Dungeon",
        }
    }

    pub fn all() -> [TownTab; 5] {
        [
            TownTab::Store,
            TownTab::Blacksmith,
            TownTab::Alchemist,
            TownTab::Field,
            TownTab::Dungeon,
        ]
    }

    pub fn next(&self) -> Self {
        match self {
            TownTab::Store => TownTab::Blacksmith,
            TownTab::Blacksmith => TownTab::Alchemist,
            TownTab::Alchemist => TownTab::Field,
            TownTab::Field => TownTab::Dungeon,
            TownTab::Dungeon => TownTab::Store,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            TownTab::Store => TownTab::Dungeon,
            TownTab::Blacksmith => TownTab::Store,
            TownTab::Alchemist => TownTab::Blacksmith,
            TownTab::Field => TownTab::Alchemist,
            TownTab::Dungeon => TownTab::Field,
        }
    }
}


#[derive(Component)]
pub struct TownUiRoot;

#[derive(Component)]
pub(super) struct TabHeaderItem {
    pub tab: TownTab,
}

#[derive(Component)]
pub struct ContentArea;

#[derive(Component)]
pub struct TabContent;

fn setup_town_ui(mut commands: Commands, current_tab: Res<State<TownTab>>) {
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
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
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

fn cleanup_town_ui(mut commands: Commands, query: Query<Entity, With<TownUiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_tab_navigation(
    current_tab: Res<State<TownTab>>,
    mut next_tab: ResMut<NextState<TownTab>>,
    mut action_events: EventReader<GameAction>,
) {
    for action in action_events.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Left) => {
                next_tab.set(current_tab.get().prev());
            }
            GameAction::Navigate(NavigationDirection::Right) => {
                next_tab.set(current_tab.get().next());
            }
            _ => {}
        }
    }
}

fn handle_back_action(
    mut action_events: EventReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            next_state.set(AppState::Menu);
        }
    }
}
