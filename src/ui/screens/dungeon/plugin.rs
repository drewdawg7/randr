use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::dungeon::{
    DungeonEntity, DungeonLayout, GridOccupancy, GridPosition, GridSize, LayoutId, TileRenderer,
    TileType,
};
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

/// Marker for the container holding the dungeon grid.
#[derive(Component)]
struct DungeonContainer;

/// Marker for entities placed on the dungeon grid with grid span positioning.
#[derive(Component)]
pub struct DungeonEntityMarker {
    pub pos: GridPosition,
    pub size: GridSize,
    pub entity_type: DungeonEntity,
}

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

    // Initialize grid occupancy and populate with entities
    let mut occupancy = GridOccupancy::new(layout.width(), layout.height());

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

            // Container for the dungeon grid
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
                    // Dungeon grid (tiles + entities using grid spans)
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
                            // Spawn tile backgrounds first (z-index 0 by default)
                            for y in 0..layout.height() {
                                for x in 0..layout.width() {
                                    grid.spawn((
                                        DungeonCell { x, y },
                                        Node {
                                            grid_column: GridPlacement::start(x as i16 + 1),
                                            grid_row: GridPlacement::start(y as i16 + 1),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|cell| {
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

                            // Spawn entities using grid spans (z-indexed by Y position)
                            for (pos, entity) in layout.entities() {
                                let size = entity.size();
                                let width_px = size.width as f32 * tile_size;
                                let height_px = size.height as f32 * tile_size;

                                // GridPlacement uses 1-indexed columns/rows
                                let entity_node = Node {
                                    grid_column: GridPlacement::start_span(
                                        pos.x as i16 + 1,
                                        size.width as u16,
                                    ),
                                    grid_row: GridPlacement::start_span(
                                        pos.y as i16 + 1,
                                        size.height as u16,
                                    ),
                                    width: Val::Px(width_px),
                                    height: Val::Px(height_px),
                                    ..default()
                                };

                                let bevy_entity = match entity {
                                    DungeonEntity::Chest { .. } => {
                                        if let Some(entity_sheet) =
                                            game_sprites.get(entity.sprite_sheet_key())
                                        {
                                            if let Some(img) =
                                                entity_sheet.image_node(entity.sprite_name())
                                            {
                                                Some(grid.spawn((
                                                    DungeonEntityMarker {
                                                        pos: *pos,
                                                        size,
                                                        entity_type: entity.clone(),
                                                    },
                                                    z_for_entity(pos.y),
                                                    img,
                                                    entity_node,
                                                )).id())
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    DungeonEntity::Mob { mob_id, .. } => {
                                        Some(grid.spawn((
                                            DungeonEntityMarker {
                                                pos: *pos,
                                                size,
                                                entity_type: entity.clone(),
                                            },
                                            DungeonMobSprite { mob_id: *mob_id },
                                            z_for_entity(pos.y),
                                            entity_node,
                                        )).id())
                                    }
                                };

                                // Populate occupancy with spawned entity
                                if let Some(bevy_entity) = bevy_entity {
                                    occupancy.occupy(*pos, size, bevy_entity);
                                }
                            }

                            // Spawn player using grid span (high z-index to render on top)
                            let (px, py) = player_pos;
                            grid.spawn((
                                DungeonPlayer,
                                DungeonPlayerSprite,
                                // Player renders above all entities
                                ZIndex(py as i32 + 100),
                                Node {
                                    grid_column: GridPlacement::start_span(px as i16 + 1, 1),
                                    grid_row: GridPlacement::start_span(py as i16 + 1, 1),
                                    width: Val::Px(tile_size),
                                    height: Val::Px(tile_size),
                                    ..default()
                                },
                            ));
                        });
                });
        });

    // Insert grid occupancy resource
    commands.insert_resource(occupancy);
}

/// Handle arrow key movement in the dungeon.
fn handle_dungeon_movement(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut state: ResMut<DungeonState>,
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
                // Empty floor - move player by updating grid placement
                state.player_pos = (new_x, new_y);
                player_node.grid_column = GridPlacement::start_span(new_x as i16 + 1, 1);
                player_node.grid_row = GridPlacement::start_span(new_y as i16 + 1, 1);
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
    mut container_query: Query<&mut Node, (With<DungeonContainer>, Without<DungeonGrid>, Without<DungeonPlayer>)>,
    mut player_query: Query<&mut Node, (With<DungeonPlayer>, Without<DungeonGrid>, Without<DungeonContainer>)>,
    mut entity_query: Query<(&DungeonEntityMarker, &mut Node), (Without<DungeonPlayer>, Without<DungeonGrid>, Without<DungeonContainer>)>,
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

            // Update grid track sizes
            if let Ok(mut grid_node) = grid_query.get_single_mut() {
                grid_node.grid_template_columns =
                    vec![GridTrack::px(tile_size); state.layout.width()];
                grid_node.grid_template_rows =
                    vec![GridTrack::px(tile_size); state.layout.height()];
            }

            // Update container dimensions
            if let Ok(mut container_node) = container_query.get_single_mut() {
                container_node.width = Val::Px(grid_width);
                container_node.height = Val::Px(grid_height);
            }

            // Update player size (grid placement handles position automatically)
            if let Ok(mut player_node) = player_query.get_single_mut() {
                player_node.width = Val::Px(tile_size);
                player_node.height = Val::Px(tile_size);
            }

            // Update entity sizes based on their grid size
            for (marker, mut entity_node) in entity_query.iter_mut() {
                entity_node.width = Val::Px(marker.size.width as f32 * tile_size);
                entity_node.height = Val::Px(marker.size.height as f32 * tile_size);
            }
        }
    }
}

/// Calculate z-index for entities based on Y position (higher Y = rendered on top).
fn z_for_entity(y: usize) -> ZIndex {
    ZIndex(y as i32)
}

fn cleanup_dungeon(mut commands: Commands, query: Query<Entity, With<DungeonRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<DungeonState>();
    commands.remove_resource::<UiScale>();
    commands.remove_resource::<GridOccupancy>();
}
