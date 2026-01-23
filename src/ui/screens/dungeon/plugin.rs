use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::assets::{DungeonTileSlice, GameSprites, SpriteSheetKey};
use crate::chest::Chest;
use crate::rock::{Rock, RockType};
use crate::dungeon::{
    DungeonEntity, DungeonLayout, DungeonRegistry, DungeonState, GridOccupancy, GridPosition,
    GridSize, TileRenderer, TileType,
};
use crate::ui::AnimationConfig;
use crate::inventory::Inventory;
use crate::location::LocationId;
use crate::input::{GameAction, NavigationDirection};
use crate::loot::{collect_loot_drops, HasLoot};
use crate::states::AppState;
use crate::stats::{StatSheet, StatType};
use crate::ui::screens::fight_modal::state::{FightModalMob, SpawnFightModal};
use crate::ui::screens::modal::ActiveModal;
use crate::ui::screens::results_modal::{ResultsModalData, SpawnResultsModal};
use crate::ui::widgets::PlayerStats;
use crate::ui::{DungeonMobSprite, DungeonPlayerSprite};

/// Scale factor for dungeon tiles (1.0 = original, 1.5 = 1.5x bigger, 2.0 = 2x bigger).
/// Changing this adjusts both tile size and layout dimensions.
pub const DUNGEON_SCALE: f32 = 1.5;

/// Grid size for entities (player and mobs). Must be a positive integer.
/// Use 2 for normal scale, 1 for 2x scale.
pub const ENTITY_GRID_SIZE: u8 = 2;

/// Base tile size before dungeon scaling (original sprite size / 2).
const BASE_TILE_UNSCALED: f32 = 8.0;

/// Actual tile size used for rendering (BASE_TILE_UNSCALED * DUNGEON_SCALE).
pub const BASE_TILE: f32 = BASE_TILE_UNSCALED * DUNGEON_SCALE;

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

pub struct DungeonScreenPlugin;

impl Plugin for DungeonScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Dungeon), spawn_dungeon_screen)
            .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
            .add_systems(
                Update,
                (
                    handle_dungeon_movement,
                    handle_mine_interaction,
                    handle_back_action,
                    handle_window_resize,
                    advance_floor_system.run_if(resource_exists::<AdvanceFloor>),
                )
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

fn spawn_dungeon_screen(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    window: Single<&Window>,
    registry: Res<DungeonRegistry>,
    mut state: ResMut<DungeonState>,
) {
    let Some(tile_sheet) = game_sprites.get(SpriteSheetKey::DungeonTileset) else {
        return;
    };

    // If not already in a dungeon, enter the first available dungeon
    // (This is temporary until there's a dungeon selection system)
    if !state.is_in_dungeon() {
        state.enter_dungeon(LocationId::GoblinCave, &registry);
    }

    // Load the layout for the current floor
    state.load_floor_layout();

    let Some(layout) = state.layout.as_ref() else {
        return;
    };

    // Calculate scale based on window size
    let scale = UiScale::calculate(window.height());
    let tile_size = BASE_TILE * scale as f32;
    commands.insert_resource(UiScale(scale));

    // Get player position and size from state (set by load_floor_layout)
    let player_pos = state.player_pos;
    let player_size = state.player_size;

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
                                        let tile_type = layout
                                            .tile_at(x, y)
                                            .map(|t| t.tile_type);

                                        match tile_type {
                                            Some(TileType::TorchWall) => {
                                                if let Some(torch_sheet) =
                                                    game_sprites.get(SpriteSheetKey::TorchWall)
                                                {
                                                    let config = AnimationConfig {
                                                        first_frame: 0,
                                                        last_frame: 2,
                                                        frame_duration: 0.4,
                                                    };
                                                    if let Some(bundle) =
                                                        torch_sheet.image_bundle_animated(
                                                            "torch_1",
                                                            tile_size,
                                                            tile_size,
                                                            config,
                                                        )
                                                    {
                                                        cell.spawn(bundle);
                                                    }
                                                }
                                            }
                                            _ => {
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
                                                                position_type:
                                                                    PositionType::Absolute,
                                                                width: Val::Percent(100.0),
                                                                height: Val::Percent(100.0),
                                                                ..default()
                                                            },
                                                        ));
                                                    }
                                                }
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
                                    DungeonEntity::Stairs { .. } => {
                                        if let Some(img) =
                                            tile_sheet.image_node(DungeonTileSlice::Stairs.as_str())
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
                                    }
                                    DungeonEntity::Rock { .. } => {
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
                                };

                                // Populate occupancy with spawned entity (center cell only for collision)
                                if let Some(bevy_entity) = bevy_entity {
                                    let center_pos = GridPosition::new(
                                        pos.x + size.width as usize / 2,
                                        pos.y + size.height as usize / 2,
                                    );
                                    occupancy.occupy(center_pos, GridSize::single(), bevy_entity);
                                }
                            }

                            // Spawn player using grid span (high z-index to render on top)
                            let player_entity = grid.spawn((
                                DungeonPlayer,
                                DungeonPlayerSprite,
                                // Player renders above all entities
                                ZIndex(player_pos.y as i32 + 100),
                                Node {
                                    grid_column: GridPlacement::start_span(player_pos.x as i16 + 1, player_size.width as u16),
                                    grid_row: GridPlacement::start_span(player_pos.y as i16 + 1, player_size.height as u16),
                                    width: Val::Px(tile_size * player_size.width as f32),
                                    height: Val::Px(tile_size * player_size.height as f32),
                                    ..default()
                                },
                            )).id();

                            // Add player to occupancy grid
                            occupancy.occupy(player_pos, player_size, player_entity);
                        });
                });
        });

    // Insert grid occupancy resource
    commands.insert_resource(occupancy);
}

/// Check if all destination cells are walkable floor tiles.
fn all_cells_walkable(layout: Option<&DungeonLayout>, pos: GridPosition, size: GridSize) -> bool {
    let Some(layout) = layout else {
        return false;
    };
    pos.occupied_cells(size)
        .all(|(x, y)| layout.is_walkable(x, y))
}

/// Check for entity collision and return the collided entity, its bevy entity, and position.
fn check_entity_collision(
    occupancy: &GridOccupancy,
    entity_query: &Query<&DungeonEntityMarker>,
    pos: GridPosition,
    size: GridSize,
) -> Option<(DungeonEntity, Entity, GridPosition)> {
    for (x, y) in pos.occupied_cells(size) {
        if let Some(entity) = occupancy.entity_at(x, y) {
            if let Ok(marker) = entity_query.get(entity) {
                return Some((marker.entity_type.clone(), entity, marker.pos));
            }
        }
    }
    None
}

/// Handle arrow key movement in the dungeon.
fn handle_dungeon_movement(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut state: ResMut<DungeonState>,
    mut occupancy: ResMut<GridOccupancy>,
    active_modal: Res<ActiveModal>,
    player_query: Single<(Entity, &mut Node), With<DungeonPlayer>>,
    entity_query: Query<&DungeonEntityMarker>,
) {
    // Block movement if any modal is open
    if active_modal.modal.is_some() {
        return;
    }

    let (player_entity, mut player_node) = player_query.into_inner();

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

        let new_pos = GridPosition::new(
            (state.player_pos.x as i32 + dx).max(0) as usize,
            (state.player_pos.y as i32 + dy).max(0) as usize,
        );

        // Check if all destination cells are walkable
        if !all_cells_walkable(state.layout.as_ref(), new_pos, state.player_size) {
            continue;
        }

        // Check for entity collision (any cell player would occupy)
        if let Some((entity_type, entity_id, entity_pos)) = check_entity_collision(
            &occupancy,
            &entity_query,
            new_pos,
            state.player_size,
        ) {
            match entity_type {
                DungeonEntity::Mob { mob_id, .. } => {
                    // Trigger fight modal with full mob data
                    commands.insert_resource(FightModalMob {
                        mob_id,
                        mob: mob_id.spawn(),
                        pos: entity_pos,
                        entity: entity_id,
                    });
                    commands.insert_resource(SpawnFightModal);
                }
                DungeonEntity::Chest { .. } => {
                    // Block movement (chests are obstacles)
                }
                DungeonEntity::Rock { .. } => {
                    // Block movement (rocks are obstacles)
                }
                DungeonEntity::Stairs { .. } => {
                    commands.insert_resource(AdvanceFloor);
                }
            }
            continue;
        }

        // Update occupancy: vacate old position, occupy new position
        occupancy.vacate(state.player_pos, state.player_size);
        occupancy.occupy(new_pos, state.player_size, player_entity);

        // Update state and visual position
        state.player_pos = new_pos;
        player_node.grid_column = GridPlacement::start_span(new_pos.x as i16 + 1, state.player_size.width as u16);
        player_node.grid_row = GridPlacement::start_span(new_pos.y as i16 + 1, state.player_size.height as u16);
    }
}

/// Handle space key to mine adjacent rocks or open adjacent chests.
fn handle_mine_interaction(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    state: Res<DungeonState>,
    mut occupancy: ResMut<GridOccupancy>,
    active_modal: Res<ActiveModal>,
    stats: Res<StatSheet>,
    mut inventory: ResMut<Inventory>,
    entity_query: Query<&DungeonEntityMarker>,
) {
    // Block interaction if any modal is open
    if active_modal.modal.is_some() {
        return;
    }

    for action in action_reader.read() {
        if *action != GameAction::Mine {
            continue;
        }

        // Find an adjacent minable entity (chest or rock)
        let Some((entity_id, entity_pos, entity_type)) =
            find_adjacent_minable(&state, &occupancy, &entity_query)
        else {
            continue;
        };

        let magic_find = stats.value(StatType::MagicFind);

        let (title, loot_drops) = match entity_type {
            DungeonEntity::Chest { .. } => {
                let chest = Chest::default();
                let drops = chest.roll_drops(magic_find);
                ("Chest Opened!".to_string(), drops)
            }
            DungeonEntity::Rock { rock_type, .. } => {
                let rock = Rock::new(rock_type);
                let drops = rock.roll_drops(magic_find);
                let rock_name = match rock_type {
                    RockType::Copper => "Copper Rock",
                    RockType::Coal => "Coal Rock",
                    RockType::Tin => "Tin Rock",
                };
                (format!("{} Mined!", rock_name), drops)
            }
            _ => continue,
        };

        // Collect loot into inventory
        collect_loot_drops(&mut *inventory, &loot_drops);

        // Despawn entity from dungeon
        occupancy.vacate(entity_pos, GridSize::single());
        commands.entity(entity_id).despawn_recursive();

        // Show results modal
        commands.insert_resource(ResultsModalData {
            title,
            subtitle: None,
            sprite: None,
            gold_gained: None,
            xp_gained: None,
            loot_drops,
        });
        commands.insert_resource(SpawnResultsModal);
        break;
    }
}

/// Find an adjacent minable entity (chest or rock) to the player's current position.
/// Returns the bevy Entity, GridPosition, and DungeonEntity type if found.
fn find_adjacent_minable(
    state: &DungeonState,
    occupancy: &GridOccupancy,
    entity_query: &Query<&DungeonEntityMarker>,
) -> Option<(Entity, GridPosition, DungeonEntity)> {
    let px = state.player_pos.x;
    let py = state.player_pos.y;
    let w = state.player_size.width as usize;
    let h = state.player_size.height as usize;

    // Collect adjacent cells (border around player's occupied area)
    let mut adjacent_cells = Vec::new();

    // Top row (y = py - 1)
    if py > 0 {
        for x in px..px + w {
            adjacent_cells.push((x, py - 1));
        }
    }
    // Bottom row (y = py + h)
    for x in px..px + w {
        adjacent_cells.push((x, py + h));
    }
    // Left column (x = px - 1)
    if px > 0 {
        for y in py..py + h {
            adjacent_cells.push((px - 1, y));
        }
    }
    // Right column (x = px + w)
    for y in py..py + h {
        adjacent_cells.push((px + w, y));
    }

    // Check each adjacent cell for a minable entity (chest or rock)
    for (x, y) in adjacent_cells {
        if let Some(entity) = occupancy.entity_at(x, y) {
            if let Ok(marker) = entity_query.get(entity) {
                if matches!(marker.entity_type, DungeonEntity::Chest { .. } | DungeonEntity::Rock { .. }) {
                    return Some((entity, marker.pos, marker.entity_type));
                }
            }
        }
    }

    None
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
    let Some(layout) = state.layout.as_ref() else {
        return;
    };

    for event in resize_events.read() {
        let Ok(window) = windows.get(event.window) else {
            continue;
        };

        let new_scale = UiScale::calculate(window.height());

        if new_scale != scale.0 {
            scale.0 = new_scale;
            let tile_size = BASE_TILE * new_scale as f32;
            let grid_width = tile_size * layout.width() as f32;
            let grid_height = tile_size * layout.height() as f32;

            // Update grid track sizes
            if let Ok(mut grid_node) = grid_query.get_single_mut() {
                grid_node.grid_template_columns = vec![GridTrack::px(tile_size); layout.width()];
                grid_node.grid_template_rows = vec![GridTrack::px(tile_size); layout.height()];
            }

            // Update container dimensions
            if let Ok(mut container_node) = container_query.get_single_mut() {
                container_node.width = Val::Px(grid_width);
                container_node.height = Val::Px(grid_height);
            }

            // Update player size (grid placement handles position automatically)
            if let Ok(mut player_node) = player_query.get_single_mut() {
                player_node.width = Val::Px(tile_size * state.player_size.width as f32);
                player_node.height = Val::Px(tile_size * state.player_size.height as f32);
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

fn cleanup_dungeon(
    mut commands: Commands,
    query: Query<Entity, With<DungeonRoot>>,
    mut state: ResMut<DungeonState>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    // Exit dungeon (clears runtime state but preserves cleared_floors)
    state.exit_dungeon();
    commands.remove_resource::<UiScale>();
    commands.remove_resource::<GridOccupancy>();
}

/// Resource that triggers advancing to the next dungeon floor.
#[derive(Resource)]
struct AdvanceFloor;

fn advance_floor_system(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    window: Single<&Window>,
    mut state: ResMut<DungeonState>,
    root_query: Query<Entity, With<DungeonRoot>>,
) {
    commands.remove_resource::<AdvanceFloor>();

    // Despawn current dungeon UI
    for entity in &root_query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<UiScale>();
    commands.remove_resource::<GridOccupancy>();

    // Advance to a new floor (uses same layout template for now)
    state.floor_index += 1;
    state.load_floor_layout();

    let Some(tile_sheet) = game_sprites.get(SpriteSheetKey::DungeonTileset) else {
        return;
    };
    let Some(layout) = state.layout.as_ref() else {
        return;
    };

    let scale = UiScale::calculate(window.height());
    let tile_size = BASE_TILE * scale as f32;
    commands.insert_resource(UiScale(scale));

    let player_pos = state.player_pos;
    let player_size = state.player_size;
    let mut occupancy = GridOccupancy::new(layout.width(), layout.height());
    let grid_width = tile_size * layout.width() as f32;
    let grid_height = tile_size * layout.height() as f32;

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
            parent.spawn(PlayerStats);

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
                                        Node {
                                            grid_column: GridPlacement::start(x as i16 + 1),
                                            grid_row: GridPlacement::start(y as i16 + 1),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|cell| {
                                        let tile_type = layout
                                            .tile_at(x, y)
                                            .map(|t| t.tile_type);

                                        match tile_type {
                                            Some(TileType::TorchWall) => {
                                                if let Some(torch_sheet) =
                                                    game_sprites.get(SpriteSheetKey::TorchWall)
                                                {
                                                    let config = AnimationConfig {
                                                        first_frame: 0,
                                                        last_frame: 2,
                                                        frame_duration: 0.4,
                                                    };
                                                    if let Some(bundle) =
                                                        torch_sheet.image_bundle_animated(
                                                            "torch_1",
                                                            tile_size,
                                                            tile_size,
                                                            config,
                                                        )
                                                    {
                                                        cell.spawn(bundle);
                                                    }
                                                }
                                            }
                                            _ => {
                                                if let Some((slice, flip_x)) =
                                                    TileRenderer::resolve(layout, x, y)
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
                                            }
                                        }
                                    });
                                }
                            }

                            for (pos, entity) in layout.entities() {
                                let size = entity.size();
                                let width_px = size.width as f32 * tile_size;
                                let height_px = size.height as f32 * tile_size;

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
                                                        entity_type: *entity,
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
                                                entity_type: *entity,
                                            },
                                            DungeonMobSprite { mob_id: *mob_id },
                                            z_for_entity(pos.y),
                                            entity_node,
                                        )).id())
                                    }
                                    DungeonEntity::Stairs { .. } => {
                                        if let Some(img) =
                                            tile_sheet.image_node(DungeonTileSlice::Stairs.as_str())
                                        {
                                            Some(grid.spawn((
                                                DungeonEntityMarker {
                                                    pos: *pos,
                                                    size,
                                                    entity_type: *entity,
                                                },
                                                z_for_entity(pos.y),
                                                img,
                                                entity_node,
                                            )).id())
                                        } else {
                                            None
                                        }
                                    }
                                    DungeonEntity::Rock { .. } => {
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
                                                        entity_type: *entity,
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
                                };

                                if let Some(bevy_entity) = bevy_entity {
                                    let center_pos = GridPosition::new(
                                        pos.x + size.width as usize / 2,
                                        pos.y + size.height as usize / 2,
                                    );
                                    occupancy.occupy(center_pos, GridSize::single(), bevy_entity);
                                }
                            }

                            let player_entity = grid.spawn((
                                DungeonPlayer,
                                DungeonPlayerSprite,
                                ZIndex(player_pos.y as i32 + 100),
                                Node {
                                    grid_column: GridPlacement::start_span(player_pos.x as i16 + 1, player_size.width as u16),
                                    grid_row: GridPlacement::start_span(player_pos.y as i16 + 1, player_size.height as u16),
                                    width: Val::Px(tile_size * player_size.width as f32),
                                    height: Val::Px(tile_size * player_size.height as f32),
                                    ..default()
                                },
                            )).id();

                            occupancy.occupy(player_pos, player_size, player_entity);
                        });
                });
        });

    commands.insert_resource(occupancy);
}
