use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::assets::GameSprites;
use crate::crafting_station::{AnvilCraftingState, CraftingStationType, ForgeCraftingState};
use crate::dungeon::systems::on_map_created;
use crate::dungeon::{map_path, DungeonEntity, DungeonEntityMarker, EntityRenderData, FloorType, GameLayer};
use crate::mob::MobCombatBundle;
use crate::ui::animation::SpriteAnimation;
use crate::ui::{MobSpriteSheets, PlayerSpriteSheet};

use super::components::{DungeonPlayer, DungeonRoot, FloorRoot};

#[derive(Component)]
pub struct DungeonCamera;

#[derive(Bundle)]
struct StaticEntityBundle {
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
    collision_layers: CollisionLayers,
    rigid_body: RigidBody,
}

#[derive(Bundle)]
struct AnimatedMobBundle {
    combat: MobCombatBundle,
    sprite: Sprite,
    transform: Transform,
    animation: SpriteAnimation,
    collider: Collider,
    collision_layers: CollisionLayers,
    rigid_body: RigidBody,
}

#[derive(Bundle)]
struct SensorEntityBundle {
    transform: Transform,
    collider: Collider,
    collision_layers: CollisionLayers,
    sensor: Sensor,
}

#[derive(Bundle)]
struct PlayerBundle {
    marker: DungeonPlayer,
    sprite: Sprite,
    transform: Transform,
    animation: SpriteAnimation,
    rigid_body: RigidBody,
    velocity: LinearVelocity,
    locked_axes: LockedAxes,
    collider: Collider,
    collision_layers: CollisionLayers,
    collision_events: CollisionEventsEnabled,
}

pub fn add_entity_visuals(
    trigger: On<Add, DungeonEntityMarker>,
    mut commands: Commands,
    marker_query: Query<&DungeonEntityMarker>,
    game_sprites: Res<GameSprites>,
    mob_sheets: Res<MobSpriteSheets>,
) {
    let entity = trigger.entity;
    let Ok(marker) = marker_query.get(entity) else {
        return;
    };

    let z = marker.pos.y * 0.0001;
    let world_pos = Vec3::new(marker.pos.x, marker.pos.y, z);

    let collider = collider_for_entity(&marker.entity_type);
    let collision_layers = collision_layers_for_entity(&marker.entity_type);

    match marker.entity_type.render_data() {
        EntityRenderData::SpriteSheet { sheet_key, sprite_name } => {
            let Some(sheet) = game_sprites.get(sheet_key) else {
                return;
            };
            let Some(sprite) = sheet.sprite(sprite_name) else {
                return;
            };

            commands.entity(entity).insert(StaticEntityBundle {
                sprite,
                transform: Transform::from_translation(world_pos),
                collider,
                collision_layers,
                rigid_body: RigidBody::Static,
            });

            if let DungeonEntity::CraftingStation { station_type, .. } = marker.entity_type {
                match station_type {
                    CraftingStationType::Forge => {
                        commands.entity(entity).insert(ForgeCraftingState::default());
                    }
                    CraftingStationType::Anvil => {
                        commands.entity(entity).insert(AnvilCraftingState::default());
                    }
                }
            }
        }
        EntityRenderData::AnimatedMob { mob_id } => {
            let Some(sheet) = mob_sheets.get(mob_id) else {
                return;
            };

            commands.entity(entity).insert(AnimatedMobBundle {
                combat: MobCombatBundle::from_mob_id(mob_id),
                sprite: Sprite::from_atlas_image(
                    sheet.texture.clone(),
                    TextureAtlas {
                        layout: sheet.layout.clone(),
                        index: sheet.animation.first_frame,
                    },
                ),
                transform: Transform::from_translation(world_pos),
                animation: SpriteAnimation::new(&sheet.animation),
                collider,
                collision_layers,
                rigid_body: RigidBody::Kinematic,
            });
        }
        EntityRenderData::Invisible => {
            commands.entity(entity).insert(SensorEntityBundle {
                transform: Transform::from_translation(world_pos),
                collider,
                collision_layers,
                sensor: Sensor,
            });
        }
    }
}

fn collider_for_entity(entity_type: &DungeonEntity) -> Collider {
    let size = entity_type.size();
    match entity_type {
        DungeonEntity::CraftingStation { station_type: CraftingStationType::Forge, .. } => {
            Collider::compound(vec![(
                Vec2::new(0.0, -8.0),
                0.0,
                Collider::rectangle(size.width * 0.75, size.height),
            )])
        }
        DungeonEntity::Mob { .. } | DungeonEntity::Npc { .. } => {
            Collider::compound(vec![(
                Vec2::new(0.0, -8.0),
                0.0,
                Collider::rectangle(16.0, 16.0),
            )])
        }
        DungeonEntity::Stairs { .. } => Collider::rectangle(size.width * 0.6, size.height * 0.6),
        _ => Collider::rectangle(size.width * 0.9, size.height * 0.9),
    }
}

fn collision_layers_for_entity(entity_type: &DungeonEntity) -> CollisionLayers {
    match entity_type {
        DungeonEntity::Mob { .. } | DungeonEntity::Npc { .. } => {
            CollisionLayers::new(GameLayer::Mob, [GameLayer::Player])
        }
        DungeonEntity::Stairs { .. } | DungeonEntity::Door { .. } => {
            CollisionLayers::new(GameLayer::Trigger, [GameLayer::Player])
        }
        _ => CollisionLayers::new(GameLayer::StaticEntity, [GameLayer::Player]),
    }
}

pub fn spawn_floor_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    floor_type: FloorType,
    camera_entity: Entity,
    map_width: usize,
    map_height: usize,
) {
    let tile_size = 32.0;

    let layout_id = floor_type.layout_id(false);
    let path = map_path(layout_id);
    let map_handle: Handle<TiledMapAsset> = asset_server.load(path);

    let floor_root = commands.spawn((FloorRoot, Transform::default(), Visibility::default())).id();

    commands
        .spawn((TiledMap(map_handle), ChildOf(floor_root)))
        .observe(on_map_created);

    let center_x = (map_width as f32 * tile_size) / 2.0;
    let center_y = (map_height as f32 * tile_size) / 2.0;
    commands.entity(camera_entity).insert((
        DungeonCamera,
        Transform::from_xyz(center_x, center_y, 999.0),
    ));

    commands.spawn((
        DungeonRoot,
        ChildOf(floor_root),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
    ));
}

#[instrument(level = "debug", skip_all, fields(?player_pos, collider_w = 16.0, collider_h = 20.0))]
pub fn spawn_player(commands: &mut Commands, player_pos: Vec2, player_sheet: &PlayerSpriteSheet) {
    let Some(texture) = player_sheet.texture.clone() else {
        return;
    };
    let Some(layout) = player_sheet.layout.clone() else {
        return;
    };

    let z = player_pos.y * 0.0001;

    commands.spawn(PlayerBundle {
        marker: DungeonPlayer,
        sprite: Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: player_sheet.animation.first_frame,
            },
        ),
        transform: Transform::from_translation(Vec3::new(player_pos.x, player_pos.y, z)),
        animation: SpriteAnimation::new(&player_sheet.animation),
        rigid_body: RigidBody::Dynamic,
        velocity: LinearVelocity::default(),
        locked_axes: LockedAxes::ROTATION_LOCKED,
        collider: Collider::compound(vec![(
            Vec2::new(0.0, -(32.0 / 2.0) + (16.0 / 2.0)),
            0.0,
            Collider::rectangle(16.0, 16.0),
        )]),
        collision_layers: CollisionLayers::new(
            GameLayer::Player,
            [
                GameLayer::Default,
                GameLayer::Mob,
                GameLayer::StaticEntity,
                GameLayer::Trigger,
            ],
        ),
        collision_events: CollisionEventsEnabled,
    });
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

