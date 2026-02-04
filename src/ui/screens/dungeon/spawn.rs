use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::crafting_station::{AnvilCraftingState, CraftingStationType, ForgeCraftingState};
use crate::dungeon::systems::on_map_created;
use crate::assets::DungeonTileSlice;
use crate::dungeon::constants::{
    CAMERA_Z, CHEST_SPRITE_NAME, COLLIDER_SCALE, DEFAULT_TILE_SIZE, FORGE_COLLIDER_OFFSET_Y,
    FORGE_COLLIDER_SCALE, MOB_COLLIDER_OFFSET_Y, MOB_COLLIDER_SIZE, PLAYER_COLLIDER_SIZE,
    STAIRS_COLLIDER_SCALE, Z_ORDER_FACTOR,
};
use crate::dungeon::{
    map_path, ChestEntity, CraftingStationEntity, DoorEntity, DungeonEntityMarker, GameLayer,
    LayoutId, MobEntity, NpcEntity, RockEntity, StairsEntity,
};
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
}

#[derive(Bundle)]
struct AnimatedMobBundle {
    combat: MobCombatBundle,
    sprite: Sprite,
    transform: Transform,
    animation: SpriteAnimation,
    collider: Collider,
}

#[derive(Bundle)]
struct SensorEntityBundle {
    transform: Transform,
    collider: Collider,
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
    chest_query: Query<&ChestEntity>,
    rock_query: Query<&RockEntity>,
    stairs_query: Query<(), With<StairsEntity>>,
    crafting_query: Query<&CraftingStationEntity>,
    door_query: Query<(), With<DoorEntity>>,
    mob_query: Query<&MobEntity>,
    npc_query: Query<&NpcEntity>,
    game_sprites: Res<GameSprites>,
    mob_sheets: Res<MobSpriteSheets>,
) {
    let entity = trigger.entity;
    let Ok(marker) = marker_query.get(entity) else {
        return;
    };

    let z = marker.pos.y * Z_ORDER_FACTOR;
    let world_pos = Vec3::new(marker.pos.x, marker.pos.y, z);

    if let Ok(_chest) = chest_query.get(entity) {
        add_static_sprite(
            &mut commands,
            entity,
            world_pos,
            marker.size,
            SpriteSheetKey::Chests,
            CHEST_SPRITE_NAME,
            &game_sprites,
        );
        return;
    }

    if let Ok(rock) = rock_query.get(entity) {
        let (sheet_key, sprite_name) = rock.rock_type.sprite_data(rock.sprite_variant);
        add_static_sprite(
            &mut commands,
            entity,
            world_pos,
            marker.size,
            sheet_key,
            sprite_name,
            &game_sprites,
        );
        return;
    }

    if stairs_query.get(entity).is_ok() {
        let collider = Collider::rectangle(
            marker.size.width * STAIRS_COLLIDER_SCALE,
            marker.size.height * STAIRS_COLLIDER_SCALE,
        );
        let Some(sheet) = game_sprites.get(SpriteSheetKey::DungeonTileset) else {
            return;
        };
        let Some(sprite) = sheet.sprite(DungeonTileSlice::Stairs.as_str()) else {
            return;
        };
        commands.entity(entity).insert(SensorEntityBundle {
            transform: Transform::from_translation(world_pos),
            collider,
        });
        commands.entity(entity).insert(sprite);
        return;
    }

    if let Ok(crafting) = crafting_query.get(entity) {
        let collider = match crafting.station_type {
            CraftingStationType::Forge => Collider::compound(vec![(
                Vec2::new(0.0, FORGE_COLLIDER_OFFSET_Y),
                0.0,
                Collider::rectangle(marker.size.width * FORGE_COLLIDER_SCALE, marker.size.height),
            )]),
            CraftingStationType::Anvil => Collider::rectangle(
                marker.size.width * COLLIDER_SCALE,
                marker.size.height * COLLIDER_SCALE,
            ),
        };

        let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) else {
            return;
        };
        let Some(sprite) = sheet.sprite(crafting.station_type.sprite_name()) else {
            return;
        };

        commands.entity(entity).insert(StaticEntityBundle {
            sprite,
            transform: Transform::from_translation(world_pos),
            collider,
        });

        match crafting.station_type {
            CraftingStationType::Forge => {
                commands.entity(entity).insert(ForgeCraftingState::default());
            }
            CraftingStationType::Anvil => {
                commands.entity(entity).insert(AnvilCraftingState::default());
            }
        }
        return;
    }

    if door_query.get(entity).is_ok() {
        let collider = Collider::rectangle(
            marker.size.width * COLLIDER_SCALE,
            marker.size.height * COLLIDER_SCALE,
        );
        commands.entity(entity).insert(SensorEntityBundle {
            transform: Transform::from_translation(world_pos),
            collider,
        });
        return;
    }

    if let Ok(mob) = mob_query.get(entity) {
        add_animated_mob(
            &mut commands,
            entity,
            world_pos,
            mob.mob_id,
            &mob_sheets,
        );
        return;
    }

    if let Ok(npc) = npc_query.get(entity) {
        add_animated_mob(
            &mut commands,
            entity,
            world_pos,
            npc.mob_id,
            &mob_sheets,
        );
    }
}

fn add_static_sprite(
    commands: &mut Commands,
    entity: Entity,
    world_pos: Vec3,
    size: crate::dungeon::EntitySize,
    sheet_key: SpriteSheetKey,
    sprite_name: &str,
    game_sprites: &GameSprites,
) {
    let Some(sheet) = game_sprites.get(sheet_key) else {
        return;
    };
    let Some(sprite) = sheet.sprite(sprite_name) else {
        return;
    };
    let collider = Collider::rectangle(size.width * COLLIDER_SCALE, size.height * COLLIDER_SCALE);
    commands.entity(entity).insert(StaticEntityBundle {
        sprite,
        transform: Transform::from_translation(world_pos),
        collider,
    });
}

fn add_animated_mob(
    commands: &mut Commands,
    entity: Entity,
    world_pos: Vec3,
    mob_id: crate::mob::MobId,
    mob_sheets: &MobSpriteSheets,
) {
    let Some(sheet) = mob_sheets.get(mob_id) else {
        return;
    };
    let collider = Collider::compound(vec![(
        Vec2::new(0.0, MOB_COLLIDER_OFFSET_Y),
        0.0,
        Collider::rectangle(MOB_COLLIDER_SIZE, MOB_COLLIDER_SIZE),
    )]);

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
    });
}

pub fn spawn_floor_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layout_id: LayoutId,
    camera_entity: Entity,
) {
    let path = map_path(layout_id);
    let map_handle: Handle<TiledMapAsset> = asset_server.load(path);

    let floor_root = commands.spawn((FloorRoot, Transform::default(), Visibility::default())).id();

    commands
        .spawn((TiledMap(map_handle), ChildOf(floor_root)))
        .observe(on_map_created);

    commands.entity(camera_entity).insert(DungeonCamera);

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

pub fn position_camera(commands: &mut Commands, camera_entity: Entity, center: Vec2) {
    commands.entity(camera_entity).insert(Transform::from_xyz(center.x, center.y, CAMERA_Z));
}

#[instrument(level = "debug", skip_all, fields(?player_pos, collider_w = 16.0, collider_h = 20.0))]
pub fn spawn_player(commands: &mut Commands, player_pos: Vec2, player_sheet: &PlayerSpriteSheet) {
    let Some(texture) = player_sheet.texture.clone() else {
        return;
    };
    let Some(layout) = player_sheet.layout.clone() else {
        return;
    };

    let z = player_pos.y * Z_ORDER_FACTOR;
    let collider_offset_y = -(DEFAULT_TILE_SIZE / 2.0) + (PLAYER_COLLIDER_SIZE / 2.0);

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
            Vec2::new(0.0, collider_offset_y),
            0.0,
            Collider::rectangle(PLAYER_COLLIDER_SIZE, PLAYER_COLLIDER_SIZE),
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


