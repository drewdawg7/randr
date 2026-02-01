use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TilePos;
use tracing::instrument;

use crate::dungeon::{FloorMonsterCount, FloorReady, FloorType, GridOccupancy, GridSize};

#[derive(Message)]
pub struct SpawnFloor {
    pub player_pos: TilePos,
    pub player_size: GridSize,
    pub floor_type: FloorType,
    pub map_width: usize,
    pub map_height: usize,
}

#[instrument(level = "debug", skip_all)]
pub fn prepare_floor(
    mut commands: Commands,
    mut events: MessageReader<SpawnFloor>,
    mut floor_ready: MessageWriter<FloorReady>,
) {
    for event in events.read() {
        let mut occupancy = GridOccupancy::new(event.map_width as u32, event.map_height as u32);
        occupancy.mark_blocked(event.player_pos, event.player_size);
        commands.insert_resource(occupancy);
        commands.insert_resource(FloorMonsterCount(0));

        floor_ready.write(FloorReady {
            player_pos: event.player_pos,
            player_size: event.player_size,
            floor_type: event.floor_type,
            map_width: event.map_width,
            map_height: event.map_height,
        });
    }
}
