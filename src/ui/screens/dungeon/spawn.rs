use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::assets::GameSprites;
use crate::crafting_station::{AnvilCraftingState, CraftingStationType, ForgeCraftingState};
use crate::dungeon::{map_path, DungeonEntity, DungeonEntityMarker, EntityRenderData, FloorType, TilePos};
use crate::mob::MobCombatBundle;
use crate::ui::animation::SpriteAnimation;
use crate::ui::widgets::PlayerStats;
use crate::ui::{MobSpriteSheets, PlayerSpriteSheet, PlayerWalkTimer};

use super::components::{DungeonPlayer, DungeonRoot, Interpolating, TargetPosition};

const CHARACTER_SCALE: f32 = 2.0;

#[derive(Component)]
pub struct DungeonCamera;

pub fn add_entity_visuals(
    trigger: On<Add, DungeonEntityMarker>,
    mut commands: Commands,
    marker_query: Query<&DungeonEntityMarker>,
    tilemap_query: TilemapConfigQuery,
    game_sprites: Res<GameSprites>,
    mob_sheets: Res<MobSpriteSheets>,
) {
    let entity = trigger.entity;
    let Ok(marker) = marker_query.get(entity) else {
        return;
    };

    let Some((world_pos, scale)) = tile_to_world(&tilemap_query, marker.pos) else {
        return;
    };

    let z = marker.pos.y as f32 * 0.01;
    let world_pos = Vec3::new(world_pos.x, world_pos.y, z);

    match marker.entity_type.render_data() {
        EntityRenderData::SpriteSheet {
            sheet_key,
            sprite_name,
        } => {
            if let Some(sheet) = game_sprites.get(sheet_key) {
                if let Some(sprite) = sheet.sprite(sprite_name) {
                    let mut entity_cmd = commands.entity(entity);
                    entity_cmd.insert((
                        sprite,
                        Transform::from_translation(world_pos).with_scale(Vec3::splat(scale)),
                    ));
                    match marker.entity_type {
                        DungeonEntity::CraftingStation {
                            station_type: CraftingStationType::Forge,
                            ..
                        } => {
                            entity_cmd.insert(ForgeCraftingState::default());
                        }
                        DungeonEntity::CraftingStation {
                            station_type: CraftingStationType::Anvil,
                            ..
                        } => {
                            entity_cmd.insert(AnvilCraftingState::default());
                        }
                        _ => {}
                    }
                }
            }
        }
        EntityRenderData::AnimatedMob { mob_id } => {
            if let Some(sheet) = mob_sheets.get(mob_id) {
                commands.entity(entity).insert((
                    MobCombatBundle::from_mob_id(mob_id),
                    Sprite::from_atlas_image(
                        sheet.texture.clone(),
                        TextureAtlas {
                            layout: sheet.layout.clone(),
                            index: sheet.animation.first_frame,
                        },
                    ),
                    Transform::from_translation(world_pos)
                        .with_scale(Vec3::splat(scale * CHARACTER_SCALE)),
                    SpriteAnimation::new(&sheet.animation),
                ));
            }
        }
    }
}

pub fn spawn_floor_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    floor_type: FloorType,
    window: &Window,
    camera_entity: Entity,
    map_width: usize,
    map_height: usize,
) {
    let available_width = window.width() - 20.0;
    let available_height = window.height() - 50.0;

    let tile_visual_scale = floor_type.tile_scale();
    let max_tile_from_width = available_width / (map_width as f32 * tile_visual_scale);
    let max_tile_from_height = available_height / (map_height as f32 * tile_visual_scale);

    let base_tile_size = max_tile_from_width.min(max_tile_from_height).max(16.0);
    let tile_size = base_tile_size * tile_visual_scale;
    let tile_scale = tile_size / 32.0;

    let layout_id = floor_type.layout_id(false);
    let path = map_path(layout_id);
    let map_handle: Handle<TiledMapAsset> = asset_server.load(path);

    commands.spawn((TiledMap(map_handle), Transform::from_scale(Vec3::splat(tile_scale))));

    let center_x = (map_width as f32 * tile_size) / 2.0;
    let center_y = (map_height as f32 * tile_size) / 2.0;
    commands.entity(camera_entity).insert((
        DungeonCamera,
        Transform::from_xyz(center_x, center_y, 999.0),
    ));

    commands
        .spawn((
            DungeonRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(PlayerStats);
        });
}

#[instrument(level = "debug", skip_all, fields(?player_pos))]
pub fn spawn_player(
    commands: &mut Commands,
    tilemap_query: &TilemapConfigQuery,
    player_pos: TilePos,
    player_sheet: &PlayerSpriteSheet,
) {
    let Some((world_pos, scale)) = tile_to_world(tilemap_query, player_pos) else {
        return;
    };

    let Some(texture) = player_sheet.texture.clone() else {
        return;
    };
    let Some(layout) = player_sheet.layout.clone() else {
        return;
    };

    let z = player_pos.y as f32 * 0.01;

    commands.spawn((
        DungeonPlayer,
        TargetPosition(world_pos),
        Interpolating,
        PlayerWalkTimer(Timer::from_seconds(0.3, TimerMode::Once)),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: player_sheet.animation.first_frame,
            },
        ),
        Transform::from_translation(Vec3::new(world_pos.x, world_pos.y, z))
            .with_scale(Vec3::splat(scale * CHARACTER_SCALE)),
        SpriteAnimation::new(&player_sheet.animation),
    ));
}

pub type TilemapConfigQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static TilemapSize,
        &'static TilemapGridSize,
        &'static TilemapTileSize,
        &'static TilemapType,
        &'static TilemapAnchor,
        &'static GlobalTransform,
    ),
    With<TiledTilemap>,
>;

pub fn tile_to_world(tilemap_query: &TilemapConfigQuery, pos: TilePos) -> Option<(Vec2, f32)> {
    let Ok((map_size, grid_size, tile_size, map_type, anchor, gt)) = tilemap_query.single() else {
        return None;
    };

    let local_pos = pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);
    let world_pos = gt.transform_point(local_pos.extend(0.0)).truncate();
    let scale = gt.to_scale_rotation_translation().0.x;

    Some((world_pos, scale))
}
