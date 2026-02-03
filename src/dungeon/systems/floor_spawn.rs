use bevy::prelude::*;
use tracing::instrument;

use crate::dungeon::{FloorMonsterCount, FloorReady, LayoutId};

#[derive(Message)]
pub struct SpawnFloor {
    pub layout_id: LayoutId,
}

#[instrument(level = "debug", skip_all)]
pub fn prepare_floor(
    mut commands: Commands,
    mut events: MessageReader<SpawnFloor>,
    mut floor_ready: MessageWriter<FloorReady>,
) {
    for event in events.read() {
        commands.insert_resource(FloorMonsterCount(0));

        floor_ready.write(FloorReady {
            layout_id: event.layout_id,
        });
    }
}
