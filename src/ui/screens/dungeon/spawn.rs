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
use crate::ui::{MobSpriteSheets, PlayerSpriteSheet};

use super::components::{DungeonPlayer, DungeonRoot, FloorRoot};

#[derive(Component)]
pub struct DungeonCamera;

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

    let size = marker.entity_type.size();
    let collider = match &marker.entity_type {
        DungeonEntity::CraftingStation { station_type: CraftingStationType::Forge, .. } => {
            Collider::compound(vec![(
                Vec2::new(0.0, -8.0),
                0.0,
                Collider::rectangle(size.width * 0.75, size.height),
            )])
        }
        DungeonEntity::Mob { .. } => {
            Collider::compound(vec![(
                Vec2::new(0.0, -8.0),
                0.0,
                Collider::rectangle(16.0, 16.0),
            )])
        }
        DungeonEntity::Stairs { .. } => {
            Collider::rectangle(size.width * 0.6, size.height * 0.6)
        }
        _ => Collider::rectangle(size.width * 0.9, size.height * 0.9),
    };
    let (physics_body, layers) = physics_components_for_entity(&marker.entity_type);

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
                        Transform::from_translation(world_pos),
                        collider,
                        layers,
                    ));
                    match physics_body {
                        PhysicsBody::Rigid(rigid_body) => {
                            entity_cmd.insert(rigid_body);
                        }
                        PhysicsBody::Sensor => {
                            entity_cmd.insert(Sensor);
                        }
                    }
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
                let mut entity_cmd = commands.entity(entity);
                entity_cmd.insert((
                    MobCombatBundle::from_mob_id(mob_id),
                    Sprite::from_atlas_image(
                        sheet.texture.clone(),
                        TextureAtlas {
                            layout: sheet.layout.clone(),
                            index: sheet.animation.first_frame,
                        },
                    ),
                    Transform::from_translation(world_pos),
                    SpriteAnimation::new(&sheet.animation),
                    collider,
                    layers,
                ));
                match physics_body {
                    PhysicsBody::Rigid(rigid_body) => {
                        entity_cmd.insert(rigid_body);
                    }
                    PhysicsBody::Sensor => {
                        entity_cmd.insert(Sensor);
                    }
                }
            }
        }
        EntityRenderData::Invisible => {
            let mut entity_cmd = commands.entity(entity);
            entity_cmd.insert((
                Transform::from_translation(world_pos),
                collider,
                layers,
                Sensor,
            ));
        }
    }
}

enum PhysicsBody {
    Rigid(RigidBody),
    Sensor,
}

fn physics_components_for_entity(entity_type: &DungeonEntity) -> (PhysicsBody, CollisionLayers) {
    match entity_type {
        DungeonEntity::Mob { .. } => (
            PhysicsBody::Rigid(RigidBody::Kinematic),
            CollisionLayers::new(GameLayer::Mob, [GameLayer::Player]),
        ),
        DungeonEntity::Stairs { .. } => (
            PhysicsBody::Rigid(RigidBody::Static),
            CollisionLayers::new(GameLayer::Trigger, [GameLayer::Player]),
        ),
        DungeonEntity::Door { .. } => (
            PhysicsBody::Sensor,
            CollisionLayers::new(GameLayer::Trigger, [GameLayer::Player]),
        ),
        _ => (
            PhysicsBody::Rigid(RigidBody::Static),
            CollisionLayers::new(GameLayer::StaticEntity, [GameLayer::Player]),
        ),
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

    commands.spawn((TiledMap(map_handle), ChildOf(floor_root)));

    let center_x = (map_width as f32 * tile_size) / 2.0;
    let center_y = (map_height as f32 * tile_size) / 2.0;
    commands.entity(camera_entity).insert((
        DungeonCamera,
        Transform::from_xyz(center_x, center_y, 999.0),
    ));

    commands
        .spawn((
            DungeonRoot,
            ChildOf(floor_root),
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

#[instrument(level = "debug", skip_all, fields(?player_pos, collider_w = 16.0, collider_h = 20.0))]
pub fn spawn_player(
    commands: &mut Commands,
    player_pos: Vec2,
    player_sheet: &PlayerSpriteSheet,
) {
    let Some(texture) = player_sheet.texture.clone() else {
        return;
    };
    let Some(layout) = player_sheet.layout.clone() else {
        return;
    };

    let z = player_pos.y * 0.0001;

    commands.spawn((
        DungeonPlayer,
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: player_sheet.animation.first_frame,
            },
        ),
        Transform::from_translation(Vec3::new(player_pos.x, player_pos.y, z)),
        SpriteAnimation::new(&player_sheet.animation),
        RigidBody::Dynamic,
        LinearVelocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Collider::compound(vec![(
            Vec2::new(0.0, -(32.0 / 2.0) + (16.0 / 2.0)),
            0.0,
            Collider::rectangle(16.0, 16.0),
        )]),
        CollisionLayers::new(
            GameLayer::Player,
            [
                GameLayer::Default,
                GameLayer::Mob,
                GameLayer::StaticEntity,
                GameLayer::Trigger,
            ],
        ),
        CollisionEventsEnabled,
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

