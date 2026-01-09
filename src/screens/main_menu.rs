use bevy::prelude::*;

use crate::game::Player;
use crate::input::{GameAction, NavigationDirection};
use crate::states::AppState;

/// Plugin that manages the main menu screen.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuSelection>()
            .add_systems(OnEnter(AppState::Menu), (spawn_main_menu, reset_menu_selection).chain())
            .add_systems(OnExit(AppState::Menu), despawn_main_menu)
            .add_systems(
                Update,
                (handle_menu_navigation, handle_menu_selection)
                    .run_if(in_state(AppState::Menu)),
            );
    }
}

/// Resource tracking the currently selected menu option.
#[derive(Resource, Default)]
pub struct MenuSelection {
    pub index: usize,
}

impl MenuSelection {
    const MENU_ITEMS: usize = 3; // Town, Profile, Quit

    pub fn up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.index + 1 < Self::MENU_ITEMS {
            self.index += 1;
        }
    }
}

/// Component marker for the main menu UI root.
#[derive(Component)]
struct MainMenuRoot;

/// Component marker for menu items.
#[derive(Component)]
struct MenuItem {
    index: usize,
}

/// System to spawn the main menu UI.
fn spawn_main_menu(mut commands: Commands, player: Res<Player>) {
    // Root container
    commands
        .spawn((
            MainMenuRoot,
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
            // Player greeting
            parent.spawn((
                Text::new(format!("Welcome, {}!", player.name)),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Menu options container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_menu_item(parent, 0, "Town");
                    spawn_menu_item(parent, 1, "Profile");
                    spawn_menu_item(parent, 2, "Quit");
                });
        });
}

/// Helper to spawn a menu item.
fn spawn_menu_item(parent: &mut ChildBuilder, index: usize, label: &str) {
    parent.spawn((
        MenuItem { index },
        Text::new(label),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
    ));
}

/// System to handle menu navigation using GameAction events.
fn handle_menu_navigation(
    mut action_reader: EventReader<GameAction>,
    mut menu_selection: ResMut<MenuSelection>,
    mut menu_items: Query<(&MenuItem, &mut TextColor)>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                menu_selection.up();
                update_menu_visuals(&menu_selection, &mut menu_items);
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                menu_selection.down();
                update_menu_visuals(&menu_selection, &mut menu_items);
            }
            _ => {}
        }
    }
}

/// System to handle menu selection and state transitions.
fn handle_menu_selection(
    mut action_reader: EventReader<GameAction>,
    menu_selection: Res<MenuSelection>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: EventWriter<AppExit>,
) {
    for action in action_reader.read() {
        if *action == GameAction::Select {
            match menu_selection.index {
                0 => {
                    // Town
                    next_state.set(AppState::Town);
                }
                1 => {
                    // Profile
                    next_state.set(AppState::Profile);
                }
                2 => {
                    // Quit
                    app_exit.send(AppExit::Success);
                }
                _ => {}
            }
        }
    }
}

/// Update menu item visuals based on current selection.
fn update_menu_visuals(selection: &MenuSelection, menu_items: &mut Query<(&MenuItem, &mut TextColor)>) {
    for (item, mut color) in menu_items.iter_mut() {
        if item.index == selection.index {
            // Selected item is white
            *color = TextColor(Color::srgb(1.0, 1.0, 1.0));
        } else {
            // Unselected items are gray
            *color = TextColor(Color::srgb(0.7, 0.7, 0.7));
        }
    }
}

/// System to reset the menu selection to the first item.
fn reset_menu_selection(
    mut menu_selection: ResMut<MenuSelection>,
    mut menu_items: Query<(&MenuItem, &mut TextColor)>,
) {
    menu_selection.index = 0;
    update_menu_visuals(&menu_selection, &mut menu_items);
}

/// System to despawn the main menu UI.
fn despawn_main_menu(mut commands: Commands, menu_root: Query<Entity, With<MainMenuRoot>>) {
    if let Ok(entity) = menu_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
