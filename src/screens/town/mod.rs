mod shared;
mod tabs;

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::game::{Player, Storage};
use crate::input::{GameAction, NavigationDirection};
use crate::states::AppState;

pub use tabs::TabsPlugin;
use tabs::{
    spawn_alchemist_ui, spawn_blacksmith_ui, spawn_dungeon_ui, spawn_field_ui, spawn_store_ui,
    AlchemistTabState, BlacksmithTabState, DungeonTabState, FieldTabState, StoreTabState,
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TabNavigationSet;

#[derive(Resource, Default)]
struct ForceTabRefresh(bool);

pub struct TownPlugin;

impl Plugin for TownPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTab>()
            .init_resource::<ForceTabRefresh>()
            .add_plugins(TabsPlugin)
            .add_systems(OnEnter(AppState::Town), (setup_town_ui, trigger_tab_refresh).chain())
            .add_systems(OnExit(AppState::Town), cleanup_town_ui)
            .add_systems(
                Update,
                (handle_tab_navigation, handle_back_action, update_tab_header_visuals)
                    .in_set(TabNavigationSet)
                    .run_if(in_state(AppState::Town)),
            )
            .add_systems(
                Update,
                render_tab_content
                    .after(TabNavigationSet)
                    .run_if(in_state(AppState::Town)),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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

#[derive(Resource, Default)]
pub struct CurrentTab {
    pub tab: TownTab,
}

#[derive(Component)]
pub struct TownUiRoot;

#[derive(Component)]
struct TabHeaderItem {
    tab: TownTab,
}

#[derive(Component)]
pub struct ContentArea;

#[derive(Component)]
pub struct TabContent;

#[derive(SystemParam)]
struct TabStates<'w> {
    field: Res<'w, FieldTabState>,
    dungeon: Res<'w, DungeonTabState>,
    store: Res<'w, StoreTabState>,
    blacksmith: Res<'w, BlacksmithTabState>,
    alchemist: Res<'w, AlchemistTabState>,
}

fn setup_town_ui(mut commands: Commands, current_tab: Res<CurrentTab>) {
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
                        spawn_tab_header_item(header, tab, tab == current_tab.tab);
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
    mut current_tab: ResMut<CurrentTab>,
    mut action_events: EventReader<GameAction>,
) {
    for action in action_events.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Left) => {
                current_tab.tab = current_tab.tab.prev();
            }
            GameAction::Navigate(NavigationDirection::Right) => {
                current_tab.tab = current_tab.tab.next();
            }
            _ => {}
        }
    }
}

fn update_tab_header_visuals(
    current_tab: Res<CurrentTab>,
    mut tab_query: Query<(&TabHeaderItem, &mut BackgroundColor)>,
) {
    if !current_tab.is_changed() {
        return;
    }

    for (tab_item, mut bg_color) in tab_query.iter_mut() {
        if tab_item.tab == current_tab.tab {
            *bg_color = BackgroundColor(Color::srgb(0.4, 0.4, 0.8));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
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

fn trigger_tab_refresh(mut refresh: ResMut<ForceTabRefresh>) {
    refresh.0 = true;
}

fn render_tab_content(
    mut commands: Commands,
    current_tab: Res<CurrentTab>,
    mut force_refresh: ResMut<ForceTabRefresh>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    tab_states: TabStates,
    player: Res<Player>,
    storage: Res<Storage>,
) {
    let should_render = force_refresh.0
        || current_tab.is_changed()
        || (current_tab.tab == TownTab::Store && tab_states.store.is_changed())
        || (current_tab.tab == TownTab::Blacksmith && tab_states.blacksmith.is_changed())
        || (current_tab.tab == TownTab::Alchemist && tab_states.alchemist.is_changed())
        || (current_tab.tab == TownTab::Field && tab_states.field.is_changed())
        || (current_tab.tab == TownTab::Dungeon && tab_states.dungeon.is_changed());

    if !should_render {
        return;
    }

    force_refresh.0 = false;

    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    match current_tab.tab {
        TownTab::Store => {
            spawn_store_ui(&mut commands, content_entity, &tab_states.store, &player, &storage);
        }
        TownTab::Blacksmith => {
            spawn_blacksmith_ui(&mut commands, content_entity, &tab_states.blacksmith, &player);
        }
        TownTab::Alchemist => {
            spawn_alchemist_ui(&mut commands, content_entity, &tab_states.alchemist, &player);
        }
        TownTab::Field => {
            spawn_field_ui(&mut commands, content_entity, &tab_states.field, &player);
        }
        TownTab::Dungeon => {
            spawn_dungeon_ui(&mut commands, content_entity, &tab_states.dungeon, &player);
        }
    }
}
