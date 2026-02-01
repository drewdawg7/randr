use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::assets::GameSprites;
use crate::crafting_station::{AnvilCraftingState, CraftingStationType, ForgeCraftingState};
use crate::dungeon::{map_path, DungeonEntity, DungeonEntityMarker, EntityRenderData, FloorType, GameLayer};
use crate::mob::MobCombatBundle;
use crate::ui::animation::SpriteAnimation;
use crate::ui::widgets::PlayerStats;
use crate::ui::{MobSpriteSheets, PlayerSpriteSheet, PlayerWalkTimer};

use super::components::{DungeonPlayer, DungeonRoot, TargetPosition};

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

    let scale = tilemap_query
        .single()
        .map(|(_, _, _, _, _, gt)| gt.to_scale_rotation_translation().0.x)
        .unwrap_or(1.0);

    let z = marker.pos.y * 0.0001;
    let world_pos = Vec3::new(marker.pos.x, marker.pos.y, z);

    let size = marker.entity_type.size();
    let collider = Collider::rectangle(size.width * scale * 0.9, size.height * scale * 0.9);
    let (rigid_body, layers) = physics_components_for_entity(&marker.entity_type);

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
                        rigid_body,
                        collider,
                        layers,
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
                    rigid_body,
                    collider,
                    layers,
                ));
            }
        }
    }
}

fn physics_components_for_entity(entity_type: &DungeonEntity) -> (RigidBody, CollisionLayers) {
    match entity_type {
        DungeonEntity::Mob { .. } => (
            RigidBody::Kinematic,
            CollisionLayers::new(GameLayer::Mob, [GameLayer::Player]),
        ),
        DungeonEntity::Stairs { .. } | DungeonEntity::Door { .. } => (
            RigidBody::Static,
            CollisionLayers::new(GameLayer::Trigger, [GameLayer::Player]),
        ),
        _ => (
            RigidBody::Static,
            CollisionLayers::new(GameLayer::StaticEntity, [GameLayer::Player]),
        ),
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
    player_pos: Vec2,
    player_sheet: &PlayerSpriteSheet,
) {
    let scale = tilemap_query
        .single()
        .map(|(_, _, _, _, _, gt)| gt.to_scale_rotation_translation().0.x)
        .unwrap_or(1.0);

    let Some(texture) = player_sheet.texture.clone() else {
        return;
    };
    let Some(layout) = player_sheet.layout.clone() else {
        return;
    };

    let z = player_pos.y * 0.0001;
    let player_collider_size = 32.0 * scale * 0.9;

    commands.spawn((
        DungeonPlayer,
        TargetPosition(player_pos),
        PlayerWalkTimer(Timer::from_seconds(0.3, TimerMode::Once)),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: player_sheet.animation.first_frame,
            },
        ),
        Transform::from_translation(Vec3::new(player_pos.x, player_pos.y, z))
            .with_scale(Vec3::splat(scale * CHARACTER_SCALE)),
        SpriteAnimation::new(&player_sheet.animation),
        RigidBody::Kinematic,
        Collider::rectangle(player_collider_size, player_collider_size),
        CollisionLayers::new(
            GameLayer::Player,
            [
                GameLayer::Tile,
                GameLayer::Mob,
                GameLayer::StaticEntity,
                GameLayer::Trigger,
            ],
        ),
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

