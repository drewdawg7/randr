use bevy::prelude::*;
use tracing::instrument;

use crate::dungeon::{EntitySize, FloorMonsterCount, FloorReady, FloorType, Occupancy};

#[derive(Message)]
pub struct SpawnFloor {
    pub player_pos: Vec2,
    pub player_size: EntitySize,
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
        let mut occupancy = Occupancy::new();
        occupancy.set_player(event.player_pos, event.player_size);
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
