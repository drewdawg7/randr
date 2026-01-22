use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::dungeon::{DungeonEntity, DungeonLayout, LayoutId, TileRenderer, TileType};
use crate::input::{GameAction, NavigationDirection};
use crate::states::AppState;
use crate::ui::screens::fight_modal::state::{FightModalMob, SpawnFightModal};
use crate::ui::screens::modal::ActiveModal;
use crate::ui::widgets::PlayerStats;
use crate::ui::{DungeonMobSprite, DungeonPlayerSprite};

/// Base tile size in pixels (before scaling).
pub const BASE_TILE: f32 = 8.0;

/// Resource tracking current UI scale factor (power of 2: 2, 4, 8, 16).
#[derive(Resource)]
pub struct UiScale(pub u32);

impl UiScale {
    /// Returns the scaled tile size in pixels.
    pub fn tile_size(&self) -> f32 {
        BASE_TILE * self.0 as f32
    }

    /// Calculate power-of-2 scale based on window size.
    pub fn calculate(window_height: f32) -> u32 {
        // Scale based on window height to keep tiles a reasonable size
        match window_height as u32 {
            0..=400 => 2,
            401..=800 => 4,
            801..=1600 => 8,
            _ => 16,
        }
    }
}

/// Resource tracking dungeon state for player movement.
#[derive(Resource)]
pub struct DungeonState {
    pub layout: DungeonLayout,
    pub player_pos: (usize, usize),
}

/// Marker component for grid cells with their coordinates.
#[derive(Component)]
pub struct DungeonCell {
    pub x: usize,
    pub y: usize,
}

/// Marker component for the player entity in the dungeon.
#[derive(Component)]
pub struct DungeonPlayer;

/// Marker for dungeon screen root
#[derive(Component)]
struct DungeonRoot;

/// Marker for the dungeon grid container.
#[derive(Component)]
struct DungeonGrid;

/// Marker for the entity overlay layer (renders on top of tiles).
#[derive(Component)]
struct EntityOverlay;

/// Marker for the container holding both grid and overlay.
#[derive(Component)]
struct DungeonContainer;

pub struct DungeonPlugin;

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Dungeon), spawn_dungeon_screen)
            .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
            .add_systems(
                Update,
                (handle_dungeon_movement, handle_back_action, handle_window_resize)
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

fn spawn_dungeon_screen(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    window: Single<&Window>,
) {
    let Some(tile_sheet) = game_sprites.get(SpriteSheetKey::DungeonTileset) else {
        return;
    };

    let layout = LayoutId::StartingRoom.layout();

    // Calculate scale based on window size
    let scale = UiScale::calculate(window.height());
    let tile_size = BASE_TILE * scale as f32;
    commands.insert_resource(UiScale(scale));

    // Find player spawn position
    let player_pos = layout
        .iter()
        .find(|(_, _, tile)| tile.tile_type == TileType::PlayerSpawn)
        .map(|(x, y, _)| (x, y))
        .unwrap_or((0, 0));

    // Insert dungeon state resource
    commands.insert_resource(DungeonState {
        layout: layout.clone(),
        player_pos,
    });

    // Calculate grid dimensions in pixels
    let grid_width = tile_size * layout.width() as f32;
    let grid_height = tile_size * layout.height() as f32;

    // Spawn root UI
    commands
        .spawn((
            DungeonRoot,
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

            // Container for grid + overlay (relative positioning context)
            parent
                .spawn((
                    DungeonContainer,
                    Node {
                        width: Val::Px(grid_width),
                        height: Val::Px(grid_height),
                        ..default()
                    },
                ))
                .with_children(|container| {
                    // Dungeon grid (tiles only)
                    container
                        .spawn((
                            DungeonGrid,
                            Node {
                                display: Display::Grid,
                                grid_template_columns: vec![GridTrack::px(tile_size); layout.width()],
                                grid_template_rows: vec![GridTrack::px(tile_size); layout.height()],
                                ..default()
                            },
                        ))
                        .with_children(|grid| {
                            for y in 0..layout.height() {
                                for x in 0..layout.width() {
                                    grid.spawn((
                                        DungeonCell { x, y },
                                        Node::default(),
                                    ))
                                    .with_children(|cell| {
                                        // Spawn tile background only
                                        if let Some((slice, flip_x)) =
                                            TileRenderer::resolve(&layout, x, y)
                                        {
                                            if let Some(mut img) =
                                                tile_sheet.image_node(slice.as_str())
                                            {
                                                if flip_x {
                                                    img.flip_x = true;
                                                }
                                                cell.spawn((
                                                    img,
                                                    Node {
                                                        position_type: PositionType::Absolute,
                                                        width: Val::Percent(100.0),
                                                        height: Val::Percent(100.0),
                                                        ..default()
                                                    },
                                                ));
                                            }
                                        }
                                    });
                                }
                            }
                        });

                    // Entity overlay (renders on top of all tiles)
                    container
                        .spawn((
                            EntityOverlay,
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(grid_width),
                                height: Val::Px(grid_height),
                                ..default()
                            },
                        ))
                        .with_children(|overlay| {
                            // Spawn player at its position
                            let (px, py) = player_pos;
                            overlay.spawn((
                                DungeonPlayer,
                                DungeonPlayerSprite,
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(px as f32 * tile_size),
                                    top: Val::Px(py as f32 * tile_size),
                                    width: Val::Px(tile_size),
                                    height: Val::Px(tile_size),
                                    ..default()
                                },
                            ));

                            // Spawn entities at their anchor positions
                            for (pos, entity) in layout.entities() {
                                let size = entity.size();
                                let x_px = pos.x as f32 * tile_size;
                                let y_px = pos.y as f32 * tile_size;
                                let w_px = tile_size * size.width as f32;
                                let h_px = tile_size * size.height as f32;

                                let entity_node = Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(x_px),
                                    top: Val::Px(y_px),
                                    width: Val::Px(w_px),
                                    height: Val::Px(h_px),
                                    ..default()
                                };

                                match entity {
                                    DungeonEntity::Chest { .. } => {
                                        if let Some(entity_sheet) =
                                            game_sprites.get(entity.sprite_sheet_key())
                                        {
                                            if let Some(img) =
                                                entity_sheet.image_node(entity.sprite_name())
                                            {
                                                overlay.spawn((img, entity_node));
                                            }
                                        }
                                    }
                                    DungeonEntity::Mob { mob_id, .. } => {
                                        overlay.spawn((
                                            DungeonMobSprite { mob_id: *mob_id },
                                            entity_node,
                                        ));
                                    }
                                }
                            }
                        });
                });
        });
}

/// Handle arrow key movement in the dungeon.
fn handle_dungeon_movement(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut state: ResMut<DungeonState>,
    scale: Res<UiScale>,
    active_modal: Res<ActiveModal>,
    mut player_query: Query<&mut Node, With<DungeonPlayer>>,
) {
    // Block movement if any modal is open
    if active_modal.modal.is_some() {
        return;
    }

    let Ok(mut player_node) = player_query.get_single_mut() else {
        return;
    };

    let tile_size = scale.tile_size();

    for action in action_reader.read() {
        let GameAction::Navigate(direction) = action else {
            continue;
        };

        let (dx, dy): (i32, i32) = match direction {
            NavigationDirection::Up => (0, -1),
            NavigationDirection::Down => (0, 1),
            NavigationDirection::Left => (-1, 0),
            NavigationDirection::Right => (1, 0),
        };

        let (cur_x, cur_y) = state.player_pos;
        let new_x = (cur_x as i32 + dx).max(0) as usize;
        let new_y = (cur_y as i32 + dy).max(0) as usize;

        // Check if target tile is a Floor
        let is_floor = state
            .layout
            .tile_at(new_x, new_y)
            .map(|t| t.tile_type == TileType::Floor)
            .unwrap_or(false);

        if !is_floor {
            continue;
        }

        // Check what entity is at the target tile
        match state.layout.entity_at(new_x, new_y) {
            None => {
                // Empty floor - move player by updating position
                state.player_pos = (new_x, new_y);
                player_node.left = Val::Px(new_x as f32 * tile_size);
                player_node.top = Val::Px(new_y as f32 * tile_size);
            }
            Some(DungeonEntity::Mob { mob_id, .. }) => {
                // Trigger fight modal
                commands.insert_resource(FightModalMob { mob_id: *mob_id });
                commands.insert_resource(SpawnFightModal);
            }
            Some(DungeonEntity::Chest { .. }) => {
                // Block movement (chests are obstacles for now)
            }
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

fn handle_window_resize(
    mut resize_events: EventReader<WindowResized>,
    windows: Query<&Window>,
    state: Res<DungeonState>,
    mut scale: ResMut<UiScale>,
    mut grid_query: Query<&mut Node, With<DungeonGrid>>,
    mut container_query: Query<&mut Node, (With<DungeonContainer>, Without<DungeonGrid>, Without<EntityOverlay>, Without<DungeonPlayer>)>,
    mut overlay_query: Query<&mut Node, (With<EntityOverlay>, Without<DungeonGrid>, Without<DungeonContainer>, Without<DungeonPlayer>)>,
    mut player_query: Query<&mut Node, (With<DungeonPlayer>, Without<DungeonGrid>, Without<DungeonContainer>, Without<EntityOverlay>)>,
) {
    for event in resize_events.read() {
        let Ok(window) = windows.get(event.window) else {
            continue;
        };

        let new_scale = UiScale::calculate(window.height());

        if new_scale != scale.0 {
            scale.0 = new_scale;
            let tile_size = BASE_TILE * new_scale as f32;
            let grid_width = tile_size * state.layout.width() as f32;
            let grid_height = tile_size * state.layout.height() as f32;

            // Update grid
            if let Ok(mut grid_node) = grid_query.get_single_mut() {
                grid_node.grid_template_columns =
                    vec![GridTrack::px(tile_size); state.layout.width()];
                grid_node.grid_template_rows =
                    vec![GridTrack::px(tile_size); state.layout.height()];
            }

            // Update container
            if let Ok(mut container_node) = container_query.get_single_mut() {
                container_node.width = Val::Px(grid_width);
                container_node.height = Val::Px(grid_height);
            }

            // Update overlay
            if let Ok(mut overlay_node) = overlay_query.get_single_mut() {
                overlay_node.width = Val::Px(grid_width);
                overlay_node.height = Val::Px(grid_height);
            }

            // Update player position
            if let Ok(mut player_node) = player_query.get_single_mut() {
                let (px, py) = state.player_pos;
                player_node.left = Val::Px(px as f32 * tile_size);
                player_node.top = Val::Px(py as f32 * tile_size);
                player_node.width = Val::Px(tile_size);
                player_node.height = Val::Px(tile_size);
            }

            // Note: Entity positions would also need updating here for full resize support
            // For now, entities use fixed positions from spawn time
        }
    }
}

fn cleanup_dungeon(mut commands: Commands, query: Query<Entity, With<DungeonRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<DungeonState>();
    commands.remove_resource::<UiScale>();
}
