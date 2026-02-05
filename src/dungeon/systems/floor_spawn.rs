use bevy::prelude::*;
use tracing::instrument;

use crate::dungeon::{FloorId, FloorMonsterCount, FloorReady};

use super::TransitionInProgress;

#[derive(Message)]
pub struct SpawnFloor {
    pub floor_id: FloorId,
}

#[instrument(level = "debug", skip_all)]
pub fn prepare_floor(
    mut commands: Commands,
    mut events: MessageReader<SpawnFloor>,
    mut floor_ready: MessageWriter<FloorReady>,
) {
    for event in events.read() {
        commands.insert_resource(FloorMonsterCount(0));
        commands.remove_resource::<TransitionInProgress>();

        floor_ready.write(FloorReady {
            floor_id: event.floor_id,
        });
    }
}
