use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::crafting_station::{AnvilCraftingState, CraftingStationType, ForgeCraftingState};
use crate::dungeon::systems::on_map_created;
use crate::assets::DungeonTileSlice;
use crate::dungeon::constants::{
    CHEST_SPRITE_NAME, FORGE_COLLIDER, MOB_COLLIDER, PLAYER_COLLIDER, STAIRS_COLLIDER,
    STATIC_COLLIDER,
};
use crate::dungeon::{
    ChestEntity, CraftingStationEntity, DepthSorting, DoorEntity, DungeonEntityMarker,
    FloorId, GameLayer, MobEntity, NpcEntity, RockEntity, StairsEntity,
};
use crate::mob::MobCombatBundle;
use crate::ui::animation::SpriteAnimation;
use crate::ui::{MobSpriteSheets, PlayerSpriteSheet, PlayerWalkTimer};

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
    walk_timer: PlayerWalkTimer,
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
    depth_sorting: Option<Res<DepthSorting>>,
) {
    let entity = trigger.entity;
    let Ok(marker) = marker_query.get(entity) else {
        return;
    };

    let depth = depth_sorting.map(|d| *d).unwrap_or_default();
    let z = depth.entity_z(marker.pos.y);
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
        let sprite_size = Vec2::new(marker.size.width, marker.size.height);
        let collider = STAIRS_COLLIDER.create_collider(sprite_size);
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
        let sprite_size = Vec2::new(marker.size.width, marker.size.height);
        let collider = match crafting.station_type {
            CraftingStationType::Forge => FORGE_COLLIDER.create_collider(sprite_size),
            CraftingStationType::Anvil => STATIC_COLLIDER.create_collider(sprite_size),
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
        let sprite_size = Vec2::new(marker.size.width, marker.size.height);
        let collider = STATIC_COLLIDER.create_collider(sprite_size);
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
    let sprite_size = Vec2::new(size.width, size.height);
    let collider = STATIC_COLLIDER.create_collider(sprite_size);
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
    let sprite_size = sheet.frame_size.as_vec2();
    let collider = MOB_COLLIDER.create_collider(sprite_size);

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
    floor_id: FloorId,
    camera_entity: Entity,
) {
    let path = floor_id.spec().path;
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

pub fn position_camera(commands: &mut Commands, camera_entity: Entity, center: Vec2, depth: &DepthSorting) {
    commands.entity(camera_entity).insert(Transform::from_xyz(center.x, center.y, depth.camera_z));
}

#[instrument(level = "debug", skip_all, fields(?player_pos))]
pub fn spawn_player(commands: &mut Commands, player_pos: Vec2, player_sheet: &PlayerSpriteSheet, depth: &DepthSorting) {
    let Some(texture) = player_sheet.texture.clone() else {
        return;
    };
    let Some(layout) = player_sheet.layout.clone() else {
        return;
    };

    let z = depth.entity_z(player_pos.y);
    let sprite_size = player_sheet.frame_size.as_vec2();
    let collider = PLAYER_COLLIDER.create_collider(sprite_size);

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
        walk_timer: PlayerWalkTimer(Timer::from_seconds(0.1, TimerMode::Once)),
        rigid_body: RigidBody::Dynamic,
        velocity: LinearVelocity::default(),
        locked_axes: LockedAxes::ROTATION_LOCKED,
        collider,
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


