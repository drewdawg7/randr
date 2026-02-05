use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::dungeon::{
    DepthSorting, DungeonRegistry, DungeonState, FloorReady, SpawnFloor, TilemapInfo,
};
use crate::location::LocationId;
use crate::ui::PlayerSpriteSheet;

use super::components::{DungeonPlayer, FloorRoot, PendingPlayerSpawn};
use super::spawn::{position_camera, spawn_floor_ui, spawn_player, DungeonCamera};

pub fn enter_dungeon(
    mut commands: Commands,
    registry: Res<DungeonRegistry>,
    mut state: ResMut<DungeonState>,
    mut spawn_floor: MessageWriter<SpawnFloor>,
) {
    if !state.is_in_dungeon() {
        state.enter_dungeon(LocationId::Home, &registry);
    }

    let Some(spawn_config) = state.get_spawn_config() else {
        return;
    };
    commands.insert_resource(spawn_config);

    let floor_id = state
        .current_floor()
        .unwrap_or(crate::dungeon::FloorId::HomeFloor);

    spawn_floor.write(SpawnFloor { floor_id });
}

#[instrument(level = "debug", skip_all)]
pub fn handle_floor_ready(
    mut commands: Commands,
    mut events: MessageReader<FloorReady>,
    asset_server: Res<AssetServer>,
    camera_query: Single<Entity, With<Camera2d>>,
    floor_root_query: Query<Entity, With<FloorRoot>>,
) {
    for event in events.read() {
        if let Ok(floor_root) = floor_root_query.single() {
            commands.entity(floor_root).despawn();
        }

        spawn_floor_ui(
            &mut commands,
            &asset_server,
            event.floor_id,
            *camera_query,
        );
    }
}

pub fn on_map_created_queue_player_spawn(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    existing_player: Query<Entity, With<DungeonPlayer>>,
) {
    for entity in &existing_player {
        commands.entity(entity).despawn();
    }

    commands.insert_resource(PendingPlayerSpawn);
}

#[instrument(level = "debug", skip_all)]
pub fn spawn_player_when_ready(
    mut commands: Commands,
    tilemap_info: Option<Res<TilemapInfo>>,
    depth_sorting: Option<Res<DepthSorting>>,
    player_sheet: Res<PlayerSpriteSheet>,
    camera_query: Query<Entity, With<DungeonCamera>>,
) {
    let Some(info) = tilemap_info else {
        return;
    };

    let depth = depth_sorting.map(|d| *d).unwrap_or_default();

    spawn_player(&mut commands, info.center, &player_sheet, &depth);

    if let Ok(camera_entity) = camera_query.single() {
        position_camera(&mut commands, camera_entity, info.center, &depth);
    }

    commands.remove_resource::<PendingPlayerSpawn>();
}
