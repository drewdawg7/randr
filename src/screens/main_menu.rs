use bevy::prelude::*;

use crate::assets::GameSprites;
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
                (handle_menu_navigation, handle_menu_selection, update_sprite_menu_items)
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

/// Component for menu items that use sprites instead of text.
#[derive(Component)]
struct SpriteMenuItem {
    index: usize,
    unselected_slice: &'static str,
    selected_slice: &'static str,
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
                    // Town - uses sprite (placeholder, populated by system)
                    // Original sprite is 47x14, scale 3x to match 32px text
                    parent.spawn((
                        SpriteMenuItem {
                            index: 0,
                            unselected_slice: "Slice_295",
                            selected_slice: "Slice_329",
                        },
                        Node {
                            width: Val::Px(141.0),
                            height: Val::Px(42.0),
                            ..default()
                        },
                    ));
                    // Profile - uses sprite
                    // Original sprite is 47x14, scale 3x to match 32px text
                    parent.spawn((
                        SpriteMenuItem {
                            index: 1,
                            unselected_slice: "Slice_193",
                            selected_slice: "Slice_227",
                        },
                        Node {
                            width: Val::Px(141.0),
                            height: Val::Px(42.0),
                            ..default()
                        },
                    ));
                    // Quit - uses sprite
                    // Original sprite is 47x14, scale 3x to match 32px text
                    parent.spawn((
                        SpriteMenuItem {
                            index: 2,
                            unselected_slice: "Slice_397",
                            selected_slice: "Slice_431",
                        },
                        Node {
                            width: Val::Px(141.0),
                            height: Val::Px(42.0),
                            ..default()
                        },
                    ));
                });
        });
}

/// System to handle menu navigation using GameAction events.
fn handle_menu_navigation(
    mut action_reader: EventReader<GameAction>,
    mut menu_selection: ResMut<MenuSelection>,
) {
    for action in action_reader.read() {
        match action {
            GameAction::Navigate(NavigationDirection::Up) => {
                menu_selection.up();
            }
            GameAction::Navigate(NavigationDirection::Down) => {
                menu_selection.down();
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

/// System to reset the menu selection to the first item.
fn reset_menu_selection(mut menu_selection: ResMut<MenuSelection>) {
    menu_selection.index = 0;
}

/// System to populate and update sprite menu items based on selection.
fn update_sprite_menu_items(
    mut commands: Commands,
    menu_selection: Res<MenuSelection>,
    game_sprites: Res<GameSprites>,
    mut query: Query<(Entity, &SpriteMenuItem, Option<&mut ImageNode>)>,
) {
    let Some(ui_all) = &game_sprites.ui_all else {
        return;
    };

    for (entity, sprite_item, image_node) in &mut query {
        let slice_name = if sprite_item.index == menu_selection.index {
            sprite_item.selected_slice
        } else {
            sprite_item.unselected_slice
        };

        let Some(index) = ui_all.get(slice_name) else {
            continue;
        };

        match image_node {
            Some(mut node) => {
                // Update existing sprite's atlas index
                if let Some(atlas) = &mut node.texture_atlas {
                    atlas.index = index;
                }
            }
            None => {
                // First time - insert the ImageNode
                commands.entity(entity).insert(ImageNode::from_atlas_image(
                    ui_all.texture.clone(),
                    TextureAtlas {
                        layout: ui_all.layout.clone(),
                        index,
                    },
                ));
            }
        }
    }
}

/// System to despawn the main menu UI.
fn despawn_main_menu(mut commands: Commands, menu_root: Query<Entity, With<MainMenuRoot>>) {
    if let Ok(entity) = menu_root.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
