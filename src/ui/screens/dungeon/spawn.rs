use bevy::prelude::*;
use tracing::instrument;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::crafting_station::{AnvilCraftingState, CraftingStationType, ForgeCraftingState};
use crate::dungeon::{
    resolve_tile, DungeonEntity, DungeonEntityMarker, DungeonLayout, EntityRenderData, FloorType,
    GridPosition, TilesetGrid,
};
use crate::mob::MobCombatBundle;
use crate::ui::{DungeonMobSprite, DungeonPlayerSprite, MobSpriteSheets, PlayerWalkTimer};

use super::components::{
    DungeonCell, DungeonContainer, DungeonGrid, DungeonPlayer, DungeonRoot, EntityLayer,
    TargetPosition, TileSizes,
};
use super::constants::{BASE_TILE, ENTITY_VISUAL_SCALE};

#[derive(Bundle)]
pub struct DungeonPlayerBundle {
    pub player: DungeonPlayer,
    pub sprite: DungeonPlayerSprite,
    pub target_pos: TargetPosition,
    pub walk_timer: PlayerWalkTimer,
    pub z_index: ZIndex,
    pub node: Node,
}

impl DungeonPlayerBundle {
    pub fn new(player_pos: GridPosition, tile_size: f32, entity_sprite_size: f32) -> Self {
        let entity_offset = -(entity_sprite_size - tile_size) / 2.0;
        let player_px = Vec2::new(
            player_pos.x as f32 * tile_size + entity_offset,
            player_pos.y as f32 * tile_size + entity_offset,
        );

        Self {
            player: DungeonPlayer,
            sprite: DungeonPlayerSprite,
            target_pos: TargetPosition(player_px),
            walk_timer: PlayerWalkTimer(Timer::from_seconds(0.3, TimerMode::Once)),
            z_index: ZIndex(player_pos.y as i32 + 100),
            node: Node {
                position_type: PositionType::Absolute,
                left: Val::Px(player_px.x),
                top: Val::Px(player_px.y),
                width: Val::Px(entity_sprite_size),
                height: Val::Px(entity_sprite_size),
                ..default()
            },
        }
    }
}

use crate::ui::widgets::PlayerStats;

pub fn spawn_floor_ui(
    commands: &mut Commands,
    layout: &DungeonLayout,
    player_pos: GridPosition,
    floor_type: FloorType,
    game_sprites: &GameSprites,
    mob_sheets: &MobSpriteSheets,
    tileset: &TilesetGrid,
    window: &Window,
) {
    let available_width = window.width() - 20.0;
    let available_height = window.height() - 50.0;

    let tile_scale = floor_type.tile_scale();
    let max_tile_from_width = available_width / (layout.width() as f32 * tile_scale);
    let max_tile_from_height = available_height / (layout.height() as f32 * tile_scale);

    let base_tile_size = max_tile_from_width.min(max_tile_from_height).max(BASE_TILE);
    let tile_size = base_tile_size * tile_scale;

    commands.insert_resource(TileSizes { tile_size, base_tile_size });

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
                            spawn_grid_tiles(grid, layout, floor_type, tileset, game_sprites);
                        });

                    let entity_sprite_size = ENTITY_VISUAL_SCALE * base_tile_size;

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
                            spawn_entities(
                                layer,
                                layout,
                                tile_size,
                                base_tile_size,
                                entity_sprite_size,
                                game_sprites,
                                mob_sheets,
                            );

                            spawn_player(
                                layer,
                                player_pos,
                                tile_size,
                                entity_sprite_size,
                            );
                        });
                });
        });
}

fn spawn_grid_tiles(
    grid: &mut ChildBuilder,
    layout: &DungeonLayout,
    floor_type: FloorType,
    tileset: &TilesetGrid,
    game_sprites: &GameSprites,
) {
    // Layout is at 2x resolution, so visual tiles are at every other position.
    // Each visual tile spans 2x2 grid cells.
    for y in (0..layout.height()).step_by(2) {
        for x in (0..layout.width()).step_by(2) {
            grid.spawn((
                DungeonCell,
                Node {
                    grid_column: GridPlacement::start(x as i16 + 1).set_span(2),
                    grid_row: GridPlacement::start(y as i16 + 1).set_span(2),
                    ..default()
                },
            ))
            .with_children(|cell| {
                if let Some(tile) = layout.tile_at(x, y) {
                    if let Some(tileset_id) = tile.tileset_id {
                        if let Some(img) = tileset.image_node_for_tile(tileset_id) {
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
                    } else if let Some(resolved) = resolve_tile(floor_type, layout, x, y) {
                        if let Some(sheet) = game_sprites.get(resolved.tileset_key) {
                            if let Some(mut img) = sheet.image_node(resolved.slice_name) {
                                if resolved.flip_x {
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
}

#[instrument(level = "debug", skip_all, fields(entity_count = layout.entities().len()))]
fn spawn_entities(
    layer: &mut ChildBuilder,
    layout: &DungeonLayout,
    tile_size: f32,
    base_tile_size: f32,
    entity_sprite_size: f32,
    game_sprites: &GameSprites,
    mob_sheets: &MobSpriteSheets,
) {
    for (pos, entity) in layout.entities() {
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
                if sprite_name.starts_with("anvil") {
                    (entity_sprite_size, base_tile_size)
                } else {
                    (entity_sprite_size, entity_sprite_size)
                }
            }
            _ => (base_tile_size, base_tile_size),
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

        match entity.render_data() {
            EntityRenderData::SpriteSheet { sheet_key, sprite_name } => {
                if let Some(sheet) = game_sprites.get(sheet_key) {
                    if let Some(img) = sheet.image_node(sprite_name) {
                        let mut entity_cmd = layer.spawn((
                            marker,
                            ZIndex(pos.y as i32),
                            img,
                            entity_node,
                        ));
                        match entity {
                            DungeonEntity::CraftingStation { station_type: CraftingStationType::Forge, .. } => {
                                entity_cmd.insert(ForgeCraftingState::default());
                            }
                            DungeonEntity::CraftingStation { station_type: CraftingStationType::Anvil, .. } => {
                                entity_cmd.insert(AnvilCraftingState::default());
                            }
                            _ => {}
                        }
                    }
                }
            }
            EntityRenderData::AnimatedMob { mob_id } => {
                layer.spawn((
                    marker,
                    DungeonMobSprite { mob_id },
                    MobCombatBundle::from_mob_id(mob_id),
                    ZIndex(pos.y as i32),
                    entity_node,
                ));
            }
        }
    }
}

fn spawn_player(
    layer: &mut ChildBuilder,
    player_pos: GridPosition,
    tile_size: f32,
    entity_sprite_size: f32,
) {
    layer.spawn(DungeonPlayerBundle::new(player_pos, tile_size, entity_sprite_size));
}
