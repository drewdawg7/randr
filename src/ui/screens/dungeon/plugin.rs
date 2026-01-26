use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::chest::Chest;
use crate::crafting_station::{AnvilCraftingState, CraftingStationType, ForgeCraftingState};
use crate::rock::{Rock, RockType};
use crate::dungeon::{
    DungeonCommands, DungeonEntity, DungeonLayout, DungeonRegistry, DungeonState,
    EntityRenderData, GridOccupancy, GridPosition, GridSize, TileRenderer, TileType,
};
use crate::ui::{AnimationConfig, PlayerSpriteSheet, PlayerWalkTimer, SpriteAnimation};
use crate::inventory::{Inventory, ManagesItems};
use crate::location::LocationId;
use crate::input::{GameAction, NavigationDirection};
use crate::loot::{collect_loot_drops, HasLoot};
use crate::states::AppState;
use crate::stats::{StatSheet, StatType};
use crate::mob::MobId;
use crate::ui::screens::fight_modal::state::{FightModalMob, SpawnFightModal};
use crate::ui::screens::anvil_modal::ActiveAnvilEntity;
use crate::ui::screens::forge_modal::ActiveForgeEntity;
use crate::ui::screens::merchant_modal::MerchantStock;
use crate::ui::screens::modal::{ActiveModal, ModalType, OpenModal};
use crate::ui::screens::results_modal::{ResultsModalData, SpawnResultsModal};
use crate::ui::widgets::PlayerStats;
use crate::ui::{DungeonMobSprite, DungeonPlayerSprite, MobSpriteSheets};

/// Scale factor for dungeon tiles (1.0 = original, 1.5 = 1.5x bigger, 2.0 = 2x bigger).
/// Changing this adjusts both tile size and layout dimensions.
pub const DUNGEON_SCALE: f32 = 1.5;

/// Base tile size before dungeon scaling (original sprite size / 2).
const BASE_TILE_UNSCALED: f32 = 8.0;

/// Actual tile size used for rendering (BASE_TILE_UNSCALED * DUNGEON_SCALE).
pub const BASE_TILE: f32 = BASE_TILE_UNSCALED * DUNGEON_SCALE;

/// Movement speed in tiles per second.
const MOVE_SPEED: f32 = 6.0;

/// Visual scale for player/mob sprites relative to tile size.
const ENTITY_VISUAL_SCALE: f32 = 2.0;

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


/// Tracks smooth pixel-based visual position for dungeon entities.
#[derive(Component)]
pub struct SmoothPosition {
    /// Current pixel position (interpolated each frame).
    pub current: Vec2,
    /// Target pixel position (set on movement).
    pub target: Vec2,
    /// Whether currently animating toward target.
    pub moving: bool,
}

/// Marker for the absolute-positioned entity/player overlay layer.
#[derive(Component)]
struct EntityLayer;

/// Marker component for grid cells.
#[derive(Component)]
pub struct DungeonCell;

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

/// Marker for entities placed on the dungeon grid.
#[derive(Component)]
pub struct DungeonEntityMarker {
    pub pos: GridPosition,
    pub entity_type: DungeonEntity,
}

/// Timer component for forge active animation.
/// When present, forge plays active animation. Removed when timer expires.
#[derive(Component)]
pub struct ForgeActiveTimer(pub Timer);

/// Timer component for anvil active animation.
/// When present, anvil plays active animation. Removed when timer expires.
#[derive(Component)]
pub struct AnvilActiveTimer(pub Timer);

/// Declarative dungeon floor component.
/// Spawning this triggers an observer that builds the full dungeon UI hierarchy,
/// calculates UiScale, and populates GridOccupancy. The trigger entity is despawned
/// after rendering (consumed by observer).
#[derive(Component)]
pub struct DungeonFloor {
    pub layout: DungeonLayout,
    pub player_pos: GridPosition,
    pub player_size: GridSize,
}

pub struct DungeonScreenPlugin;

impl Plugin for DungeonScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_dungeon_floor)
            .add_systems(OnEnter(AppState::Dungeon), spawn_dungeon_screen)
            .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
            .add_systems(
                Update,
                (
                    handle_dungeon_movement,
                    interpolate_positions,
                    handle_mine_interaction,
                    handle_back_action,
                    handle_window_resize,
                    advance_floor_system.run_if(resource_exists::<AdvanceFloor>),
                    revert_forge_idle,
                    revert_anvil_idle,
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

/// Observer that renders a complete dungeon floor when a `DungeonFloor` component is spawned.
/// Handles UiScale calculation, UI hierarchy creation, tile/entity/player spawning,
/// and GridOccupancy population. The trigger entity is despawned after rendering.
fn on_add_dungeon_floor(
    trigger: Trigger<OnAdd, DungeonFloor>,
    mut commands: Commands,
    query: Query<&DungeonFloor>,
    game_sprites: Res<GameSprites>,
    mob_sheets: Res<MobSpriteSheets>,
    window: Single<&Window>,
) {
    let entity = trigger.entity();
    let Ok(floor) = query.get(entity) else {
        return;
    };

    let layout = floor.layout.clone();
    let player_pos = floor.player_pos;
    let player_size = floor.player_size;

    // Despawn the trigger entity (consumed)
    commands.entity(entity).despawn();

    // Calculate scale from window dimensions
    let scale = UiScale::calculate(window.height());
    let tile_size = BASE_TILE * scale as f32;
    commands.insert_resource(UiScale(scale));

    // Build occupancy
    let mut occupancy = GridOccupancy::new(layout.width(), layout.height());
    let grid_width = tile_size * layout.width() as f32;
    let grid_height = tile_size * layout.height() as f32;

    let tile_sheet = game_sprites.get(SpriteSheetKey::DungeonTileset);

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
                    // Tile layer: CSS Grid for static tile backgrounds
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
                                        DungeonCell,
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
                                                        looping: true,
                                                        synchronized: true,
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
                                                if let Some(tile_sheet) = tile_sheet.as_ref() {
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
                                        }
                                    });
                                }
                            }
                        });

                    // Entity layer: absolute-positioned overlay for entities + player
                    let entity_sprite_size = ENTITY_VISUAL_SCALE * tile_size;
                    let entity_offset = -(entity_sprite_size - tile_size) / 2.0;

                    container
                        .spawn((
                            EntityLayer,
                            Node {
                                position_type: PositionType::Absolute,
                                width: Val::Px(grid_width),
                                height: Val::Px(grid_height),
                                ..default()
                            },
                        ))
                        .with_children(|layer| {
                            // Spawn entities with absolute positioning
                            for (pos, entity) in layout.entities() {
                                let size = entity.size();
                                let (visual_width, visual_height) = match entity.render_data() {
                                    EntityRenderData::AnimatedMob { mob_id } => {
                                        let frame_size = mob_sheets
                                            .get(mob_id)
                                            .map(|s| s.frame_size)
                                            .unwrap_or(UVec2::splat(32));
                                        let aspect = frame_size.x as f32 / frame_size.y as f32;
                                        (entity_sprite_size * aspect, entity_sprite_size)
                                    }
                                    EntityRenderData::SpriteSheet { sheet_key: SpriteSheetKey::CraftingStations, sprite_name } => {
                                        // Size based on actual sprite dimensions
                                        if sprite_name.starts_with("anvil") {
                                            // Anvil is 32x16 (2:1 aspect) - render at 2x tile width, 1x tile height
                                            (entity_sprite_size, tile_size)
                                        } else {
                                            // Forge is 32x49 - render at 2x tile size
                                            (entity_sprite_size, entity_sprite_size)
                                        }
                                    }
                                    _ => (tile_size, tile_size),
                                };
                                let offset_x = -(visual_width - tile_size) / 2.0;
                                let offset_y = -(visual_height - tile_size) / 2.0;
                                let left = pos.x as f32 * tile_size + offset_x;
                                let top = pos.y as f32 * tile_size + offset_y;

                                let entity_node = Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(left),
                                    top: Val::Px(top),
                                    width: Val::Px(visual_width),
                                    height: Val::Px(visual_height),
                                    ..default()
                                };

                                let marker = DungeonEntityMarker {
                                    pos: *pos,
                                    entity_type: *entity,
                                };

                                let bevy_entity = match entity.render_data() {
                                    EntityRenderData::SpriteSheet { sheet_key, sprite_name } => {
                                        game_sprites.get(sheet_key)
                                            .and_then(|sheet| sheet.image_node(sprite_name))
                                            .map(|img| {
                                                let mut entity_cmd = layer.spawn((
                                                    marker,
                                                    z_for_entity(pos.y),
                                                    img,
                                                    entity_node,
                                                ));
                                                // Add crafting state to crafting station entities
                                                match entity {
                                                    DungeonEntity::CraftingStation { station_type: CraftingStationType::Forge, .. } => {
                                                        entity_cmd.insert(ForgeCraftingState::default());
                                                    }
                                                    DungeonEntity::CraftingStation { station_type: CraftingStationType::Anvil, .. } => {
                                                        entity_cmd.insert(AnvilCraftingState::default());
                                                    }
                                                    _ => {}
                                                }
                                                entity_cmd.id()
                                            })
                                    }
                                    EntityRenderData::AnimatedMob { mob_id } => {
                                        Some(layer.spawn((
                                            marker,
                                            DungeonMobSprite { mob_id },
                                            z_for_entity(pos.y),
                                            entity_node,
                                        )).id())
                                    }
                                };

                                if let Some(bevy_entity) = bevy_entity {
                                    occupancy.occupy(*pos, size, bevy_entity);
                                }
                            }

                            // Spawn player with absolute positioning + SmoothPosition
                            let player_px = Vec2::new(
                                player_pos.x as f32 * tile_size + entity_offset,
                                player_pos.y as f32 * tile_size + entity_offset,
                            );

                            let player_entity = layer.spawn((
                                DungeonPlayer,
                                DungeonPlayerSprite,
                                PlayerWalkTimer(Timer::from_seconds(0.3, TimerMode::Once)),
                                SmoothPosition {
                                    current: player_px,
                                    target: player_px,
                                    moving: false,
                                },
                                ZIndex(player_pos.y as i32 + 100),
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(player_px.x),
                                    top: Val::Px(player_px.y),
                                    width: Val::Px(entity_sprite_size),
                                    height: Val::Px(entity_sprite_size),
                                    ..default()
                                },
                            )).id();

                            occupancy.occupy(player_pos, player_size, player_entity);
                        });
                });
        });

    commands.insert_resource(occupancy);
}

fn spawn_dungeon_screen(
    mut commands: Commands,
    registry: Res<DungeonRegistry>,
    mut state: ResMut<DungeonState>,
) {
    if !state.is_in_dungeon() {
        state.enter_dungeon(LocationId::GoblinCave, &registry);
    }

    state.load_floor_layout();

    let Some(layout) = state.layout.clone() else {
        return;
    };

    commands.spawn(DungeonFloor {
        layout,
        player_pos: state.player_pos,
        player_size: state.player_size,
    });
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
    scale: Res<UiScale>,
    keyboard: Res<ButtonInput<KeyCode>>,
    active_modal: Res<ActiveModal>,
    sheet: Res<PlayerSpriteSheet>,
    mut player_query: Query<(Entity, &mut SmoothPosition, &mut ImageNode, &mut SpriteAnimation, &mut PlayerWalkTimer), With<DungeonPlayer>>,
    entity_query: Query<&DungeonEntityMarker>,
) {
    // Block movement if any modal is open
    if active_modal.modal.is_some() {
        return;
    }

    let Ok((player_entity, mut smooth_pos, mut player_image, mut anim, mut walk_timer)) = player_query.get_single_mut() else {
        return;
    };

    // Block new movement while animating
    if smooth_pos.moving {
        return;
    }

    // Determine direction: prefer events (for initial press), fall back to held keys
    let direction = action_reader
        .read()
        .find_map(|a| match a {
            GameAction::Navigate(dir) => Some(*dir),
            _ => None,
        })
        .or_else(|| held_direction(&keyboard));

    let Some(direction) = direction else {
        return;
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
        return;
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
                commands.insert_resource(FightModalMob {
                    mob_id,
                    mob: mob_id.spawn(),
                    pos: entity_pos,
                    entity: entity_id,
                });
                commands.insert_resource(SpawnFightModal);
            }
            DungeonEntity::Chest { .. }
            | DungeonEntity::Rock { .. }
            | DungeonEntity::Npc { .. }
            | DungeonEntity::CraftingStation { .. } => {}
            DungeonEntity::Stairs { .. } => {
                commands.insert_resource(AdvanceFloor);
            }
        }
        return;
    }

    // Update logical state
    occupancy.vacate(state.player_pos, state.player_size);
    occupancy.occupy(new_pos, state.player_size, player_entity);
    state.player_pos = new_pos;

    // Set interpolation target
    let tile_size = BASE_TILE * scale.0 as f32;
    let entity_sprite_size = ENTITY_VISUAL_SCALE * tile_size;
    let entity_offset = -(entity_sprite_size - tile_size) / 2.0;
    smooth_pos.target = Vec2::new(
        new_pos.x as f32 * tile_size + entity_offset,
        new_pos.y as f32 * tile_size + entity_offset,
    );
    smooth_pos.moving = true;

    // Flip sprite based on horizontal direction
    match direction {
        NavigationDirection::Left => player_image.flip_x = true,
        NavigationDirection::Right => player_image.flip_x = false,
        _ => {}
    }

    // Switch to walk animation (don't reset frame if already walking)
    let already_walking = anim.first_frame == sheet.walk_animation.first_frame;
    if !already_walking {
        anim.first_frame = sheet.walk_animation.first_frame;
        anim.last_frame = sheet.walk_animation.last_frame;
        anim.current_frame = sheet.walk_animation.first_frame;
        anim.frame_duration = sheet.walk_animation.frame_duration;
        anim.synchronized = false;
        anim.timer = Timer::from_seconds(sheet.walk_animation.frame_duration, TimerMode::Repeating);
    }
    walk_timer.0.reset();
}

/// Check which arrow key is currently held (if any).
fn held_direction(keyboard: &ButtonInput<KeyCode>) -> Option<NavigationDirection> {
    if keyboard.pressed(KeyCode::ArrowLeft) {
        Some(NavigationDirection::Left)
    } else if keyboard.pressed(KeyCode::ArrowRight) {
        Some(NavigationDirection::Right)
    } else if keyboard.pressed(KeyCode::ArrowUp) {
        Some(NavigationDirection::Up)
    } else if keyboard.pressed(KeyCode::ArrowDown) {
        Some(NavigationDirection::Down)
    } else {
        None
    }
}

/// Interpolates entity positions smoothly toward their targets each frame.
fn interpolate_positions(
    time: Res<Time>,
    scale: Res<UiScale>,
    mut query: Query<(&mut SmoothPosition, &mut Node)>,
) {
    let tile_size = BASE_TILE * scale.0 as f32;
    let speed = MOVE_SPEED * tile_size;

    for (mut pos, mut node) in &mut query {
        if !pos.moving {
            continue;
        }

        let delta = pos.target - pos.current;
        let distance = delta.length();

        if distance < 0.5 {
            pos.current = pos.target;
            pos.moving = false;
        } else {
            let step = speed * time.delta_secs();
            pos.current += delta.normalize() * step.min(distance);
        }

        node.left = Val::Px(pos.current.x);
        node.top = Val::Px(pos.current.y);
    }
}

/// Handle space key to interact with adjacent entities (NPCs, chests, rocks, forges).
fn handle_mine_interaction(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    state: Res<DungeonState>,
    occupancy: Res<GridOccupancy>,
    active_modal: Res<ActiveModal>,
    stats: Res<StatSheet>,
    mut inventory: ResMut<Inventory>,
    entity_query: Query<&DungeonEntityMarker>,
    forge_query: Query<&ForgeActiveTimer>,
    anvil_query: Query<&AnvilActiveTimer>,
) {
    // Block interaction if any modal is open
    if active_modal.modal.is_some() {
        return;
    }

    for action in action_reader.read() {
        if *action != GameAction::Mine {
            continue;
        }

        // First check for adjacent NPC
        if let Some((_, _, mob_id)) = find_adjacent_npc(&state, &occupancy, &entity_query) {
            // Handle NPC interaction based on type
            match mob_id {
                MobId::Merchant => {
                    commands.insert_resource(MerchantStock::generate());
                    commands.trigger(OpenModal(ModalType::MerchantModal));
                }
                _ => {
                    // Other NPCs - could add dialogue system later
                }
            }
            break;
        }

        // Check for adjacent crafting station (forge or anvil)
        if let Some((entity_id, _, entity_type)) =
            find_adjacent_crafting_station(&state, &occupancy, &entity_query)
        {
            if let DungeonEntity::CraftingStation { station_type, .. } = entity_type {
                match station_type {
                    CraftingStationType::Forge => {
                        // Only open modal if forge is not already crafting
                        if forge_query.get(entity_id).is_err() {
                            commands.insert_resource(ActiveForgeEntity(entity_id));
                            commands.trigger(OpenModal(ModalType::ForgeModal));
                        }
                    }
                    CraftingStationType::Anvil => {
                        // Only open modal if anvil is not already crafting
                        if anvil_query.get(entity_id).is_err() {
                            commands.insert_resource(ActiveAnvilEntity(entity_id));
                            commands.trigger(OpenModal(ModalType::AnvilModal));
                        }
                    }
                }
            }
            break;
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
        commands.despawn_dungeon_entity(entity_id, entity_pos, GridSize::single());

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
/// Checks the 4 cardinal directions from the player's 1x1 cell.
fn find_adjacent_minable(
    state: &DungeonState,
    occupancy: &GridOccupancy,
    entity_query: &Query<&DungeonEntityMarker>,
) -> Option<(Entity, GridPosition, DungeonEntity)> {
    let px = state.player_pos.x;
    let py = state.player_pos.y;

    let adjacent_cells: [(i32, i32); 4] = [
        (px as i32, py as i32 - 1), // up
        (px as i32, py as i32 + 1), // down
        (px as i32 - 1, py as i32), // left
        (px as i32 + 1, py as i32), // right
    ];

    for (x, y) in adjacent_cells {
        if x < 0 || y < 0 {
            continue;
        }
        if let Some(entity) = occupancy.entity_at(x as usize, y as usize) {
            if let Ok(marker) = entity_query.get(entity) {
                if matches!(marker.entity_type, DungeonEntity::Chest { .. } | DungeonEntity::Rock { .. }) {
                    return Some((entity, marker.pos, marker.entity_type));
                }
            }
        }
    }

    None
}

/// Find an adjacent NPC to the player's current position.
/// Checks the 4 cardinal directions from the player's 1x1 cell.
fn find_adjacent_npc(
    state: &DungeonState,
    occupancy: &GridOccupancy,
    entity_query: &Query<&DungeonEntityMarker>,
) -> Option<(Entity, GridPosition, MobId)> {
    let px = state.player_pos.x;
    let py = state.player_pos.y;

    let adjacent_cells: [(i32, i32); 4] = [
        (px as i32, py as i32 - 1), // up
        (px as i32, py as i32 + 1), // down
        (px as i32 - 1, py as i32), // left
        (px as i32 + 1, py as i32), // right
    ];

    for (x, y) in adjacent_cells {
        if x < 0 || y < 0 {
            continue;
        }
        if let Some(entity) = occupancy.entity_at(x as usize, y as usize) {
            if let Ok(marker) = entity_query.get(entity) {
                if let DungeonEntity::Npc { mob_id, .. } = marker.entity_type {
                    return Some((entity, marker.pos, mob_id));
                }
            }
        }
    }

    None
}

/// Find an adjacent crafting station to the player's current position.
/// Checks the 4 cardinal directions from the player's 1x1 cell.
fn find_adjacent_crafting_station(
    state: &DungeonState,
    occupancy: &GridOccupancy,
    entity_query: &Query<&DungeonEntityMarker>,
) -> Option<(Entity, GridPosition, DungeonEntity)> {
    let px = state.player_pos.x;
    let py = state.player_pos.y;

    let adjacent_cells: [(i32, i32); 4] = [
        (px as i32, py as i32 - 1), // up
        (px as i32, py as i32 + 1), // down
        (px as i32 - 1, py as i32), // left
        (px as i32 + 1, py as i32), // right
    ];

    for (x, y) in adjacent_cells {
        if x < 0 || y < 0 {
            continue;
        }
        if let Some(entity) = occupancy.entity_at(x as usize, y as usize) {
            if let Ok(marker) = entity_query.get(entity) {
                if matches!(marker.entity_type, DungeonEntity::CraftingStation { .. }) {
                    return Some((entity, marker.pos, marker.entity_type));
                }
            }
        }
    }

    None
}

/// System to revert forge to idle animation after the active timer expires.
/// Also completes the crafting process, converting ore+coal to ingots.
fn revert_forge_idle(
    mut commands: Commands,
    time: Res<Time>,
    game_sprites: Res<GameSprites>,
    mut query: Query<(Entity, &mut ForgeActiveTimer, &mut ImageNode, Option<&mut ForgeCraftingState>)>,
) {
    for (entity, mut timer, mut image, forge_state) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // Complete crafting if forge has crafting state
            if let Some(mut state) = forge_state {
                state.complete_crafting();
            }

            // Revert to idle sprite
            if let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) {
                if let Some(idle_idx) = sheet.get("forge_1_idle") {
                    if let Some(ref mut atlas) = image.texture_atlas {
                        atlas.index = idle_idx;
                    }
                }
            }
            // Remove timer and animation components
            commands.entity(entity).remove::<ForgeActiveTimer>();
            commands.entity(entity).remove::<SpriteAnimation>();
        }
    }
}

/// Tick anvil animation timer. On expiry: complete crafting, add item to inventory, revert sprite.
fn revert_anvil_idle(
    mut commands: Commands,
    time: Res<Time>,
    game_sprites: Res<GameSprites>,
    mut inventory: ResMut<Inventory>,
    mut query: Query<(Entity, &mut AnvilActiveTimer, &mut ImageNode, Option<&mut AnvilCraftingState>)>,
) {
    for (entity, mut timer, mut image, anvil_state) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // Complete crafting if anvil has crafting state
            if let Some(mut state) = anvil_state {
                if let Some(recipe_id) = state.complete_crafting() {
                    // Add crafted item to inventory
                    let spec = recipe_id.spec();
                    let item = spec.output.spawn();
                    let _ = inventory.add_to_inv(item);
                }
            }

            // Revert to idle sprite
            if let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) {
                if let Some(idle_idx) = sheet.get("anvil_idle") {
                    if let Some(ref mut atlas) = image.texture_atlas {
                        atlas.index = idle_idx;
                    }
                }
            }
            // Remove timer and animation components
            commands.entity(entity).remove::<AnvilActiveTimer>();
            commands.entity(entity).remove::<SpriteAnimation>();
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
    mob_sheets: Res<MobSpriteSheets>,
    mut scale: ResMut<UiScale>,
    mut grid_query: Query<&mut Node, With<DungeonGrid>>,
    mut container_query: Query<&mut Node, (With<DungeonContainer>, Without<DungeonGrid>, Without<DungeonPlayer>, Without<EntityLayer>)>,
    mut layer_query: Query<&mut Node, (With<EntityLayer>, Without<DungeonGrid>, Without<DungeonContainer>, Without<DungeonPlayer>)>,
    mut player_query: Query<(&mut Node, &mut SmoothPosition), (With<DungeonPlayer>, Without<DungeonGrid>, Without<DungeonContainer>, Without<EntityLayer>)>,
    mut entity_query: Query<(&DungeonEntityMarker, &mut Node), (Without<DungeonPlayer>, Without<DungeonGrid>, Without<DungeonContainer>, Without<EntityLayer>)>,
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

            // Update entity layer dimensions
            if let Ok(mut layer_node) = layer_query.get_single_mut() {
                layer_node.width = Val::Px(grid_width);
                layer_node.height = Val::Px(grid_height);
            }

            // Update player position and size
            if let Ok((mut player_node, mut smooth_pos)) = player_query.get_single_mut() {
                let entity_sprite_size = ENTITY_VISUAL_SCALE * tile_size;
                let entity_offset = -(entity_sprite_size - tile_size) / 2.0;
                let new_px = Vec2::new(
                    state.player_pos.x as f32 * tile_size + entity_offset,
                    state.player_pos.y as f32 * tile_size + entity_offset,
                );
                smooth_pos.current = new_px;
                smooth_pos.target = new_px;
                smooth_pos.moving = false;
                player_node.left = Val::Px(new_px.x);
                player_node.top = Val::Px(new_px.y);
                player_node.width = Val::Px(entity_sprite_size);
                player_node.height = Val::Px(entity_sprite_size);
            }

            // Update entity positions and sizes
            let entity_sprite_size = ENTITY_VISUAL_SCALE * tile_size;
            for (marker, mut entity_node) in entity_query.iter_mut() {
                let (visual_width, visual_height) = match marker.entity_type {
                    DungeonEntity::Mob { mob_id, .. } => {
                        let frame_size = mob_sheets
                            .get(mob_id)
                            .map(|s| s.frame_size)
                            .unwrap_or(UVec2::splat(32));
                        let aspect = frame_size.x as f32 / frame_size.y as f32;
                        (entity_sprite_size * aspect, entity_sprite_size)
                    }
                    _ => (tile_size, tile_size),
                };
                let offset_x = -(visual_width - tile_size) / 2.0;
                let offset_y = -(visual_height - tile_size) / 2.0;
                entity_node.left = Val::Px(marker.pos.x as f32 * tile_size + offset_x);
                entity_node.top = Val::Px(marker.pos.y as f32 * tile_size + offset_y);
                entity_node.width = Val::Px(visual_width);
                entity_node.height = Val::Px(visual_height);
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
    mut state: ResMut<DungeonState>,
    root_query: Query<Entity, With<DungeonRoot>>,
) {
    commands.remove_resource::<AdvanceFloor>();

    for entity in &root_query {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<UiScale>();
    commands.remove_resource::<GridOccupancy>();

    state.floor_index += 1;
    state.load_floor_layout();

    let Some(layout) = state.layout.clone() else {
        return;
    };

    commands.spawn(DungeonFloor {
        layout,
        player_pos: state.player_pos,
        player_size: state.player_size,
    });
}
