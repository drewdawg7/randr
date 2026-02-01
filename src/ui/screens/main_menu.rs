use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites, SpriteSheetKey, UiAllSlice};
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
                (
                    handle_menu_navigation,
                    handle_menu_selection,
                    update_sprite_menu_items,
                    populate_randr_title,
                    populate_menu_background,
                )
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

/// Marker component for the randr title banner.
#[derive(Component)]
struct RandrTitle;

/// Marker component indicating the background needs to be populated.
#[derive(Component)]
struct NeedsBackground;

/// System to spawn the main menu UI.
fn spawn_main_menu(mut commands: Commands) {
    // Root container with placeholder background (populated by system when asset loads)
    commands
        .spawn((
            MainMenuRoot,
            NeedsBackground,
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
            // Title banner with "randr" text (populated by system)
            parent.spawn((
                RandrTitle,
                Node {
                    width: Val::Px(276.0),  // 92 * 3
                    height: Val::Px(78.0),  // 26 * 3
                    margin: UiRect::bottom(Val::Px(40.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
                            unselected_slice: UiAllSlice::ButtonTown.as_str(),
                            selected_slice: UiAllSlice::ButtonTownSelected.as_str(),
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
                            unselected_slice: UiAllSlice::ButtonProfile.as_str(),
                            selected_slice: UiAllSlice::ButtonProfileSelected.as_str(),
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
                            unselected_slice: UiAllSlice::ButtonQuit.as_str(),
                            selected_slice: UiAllSlice::ButtonQuitSelected.as_str(),
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
    mut action_reader: MessageReader<GameAction>,
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
    mut action_reader: MessageReader<GameAction>,
    menu_selection: Res<MenuSelection>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for action in action_reader.read() {
        if *action == GameAction::Select {
            match menu_selection.index {
                0 => {
                    // Dungeon
                    next_state.set(AppState::Dungeon);
                }
                1 => {
                    // Profile
                    next_state.set(AppState::Profile);
                }
                2 => {
                    // Quit
                    app_exit.write(AppExit::Success);
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
    let Some(ui_all) = game_sprites.get(SpriteSheetKey::UiAll) else {
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
                if let Some(img) = ui_all.image_node(slice_name) {
                    commands.entity(entity).insert(img);
                }
            }
        }
    }
}

/// System to populate the randr title banner with sprite and text.
fn populate_randr_title(
    mut commands: Commands,
    query: Query<Entity, With<RandrTitle>>,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
) {
    let Some(ui_all) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };

    for entity in &query {
        let Some(img) = ui_all.image_node(UiAllSlice::TitleBanner.as_str()) else {
            continue;
        };

        commands
            .entity(entity)
            .remove::<RandrTitle>()
            .insert(img)
            .with_children(|parent| {
                parent.spawn((
                    Text::new("RANDR"),
                    game_fonts.pixel_font(30.0),
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                ));
            });
    }
}

/// System to populate the menu background when the asset loads.
fn populate_menu_background(
    mut commands: Commands,
    query: Query<Entity, With<NeedsBackground>>,
    game_sprites: Res<GameSprites>,
) {
    let Some(sheet) = game_sprites.get(SpriteSheetKey::MenuBackground) else {
        return;
    };
    let Some(bg) = sheet.image_node("Background") else {
        return;
    };

    for entity in &query {
        commands
            .entity(entity)
            .remove::<NeedsBackground>()
            .remove::<BackgroundColor>()
            .insert(bg.clone());
    }
}

/// System to despawn the main menu UI.
fn despawn_main_menu(mut commands: Commands, menu_root: Query<Entity, With<MainMenuRoot>>) {
    if let Ok(entity) = menu_root.get_single() {
        commands.entity(entity).despawn();
    }
}
