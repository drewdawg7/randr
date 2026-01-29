use bevy::prelude::*;

use crate::dungeon::{
    DungeonEntity, DungeonLayout, FloorMonsterCount, FloorReady, FloorType, GridOccupancy,
    GridPosition, GridSize,
};

#[derive(Event)]
pub struct SpawnFloor {
    pub layout: DungeonLayout,
    pub player_pos: GridPosition,
    pub player_size: GridSize,
    pub floor_type: FloorType,
}

pub fn prepare_floor(
    mut commands: Commands,
    mut events: EventReader<SpawnFloor>,
    mut floor_ready: EventWriter<FloorReady>,
) {
    for event in events.read() {
        let layout = &event.layout;

        let mut occupancy = GridOccupancy::new(layout.width(), layout.height());
        occupancy.mark_blocked(event.player_pos, event.player_size);
        commands.insert_resource(occupancy);

        let mob_count = layout
            .entities()
            .iter()
            .filter(|(_, e)| matches!(e, DungeonEntity::Mob { .. }))
            .count();
        commands.insert_resource(FloorMonsterCount(mob_count));

        floor_ready.send(FloorReady {
            layout: event.layout.clone(),
            player_pos: event.player_pos,
            player_size: event.player_size,
            floor_type: event.floor_type,
        });
    }
}
