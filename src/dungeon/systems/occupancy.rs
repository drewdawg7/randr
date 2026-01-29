use bevy::prelude::*;

use crate::dungeon::{DungeonEntityMarker, GridOccupancy};

pub fn track_entity_occupancy(
    trigger: Trigger<OnAdd, DungeonEntityMarker>,
    query: Query<&DungeonEntityMarker>,
    mut occupancy: ResMut<GridOccupancy>,
) {
    let entity = trigger.entity();
    let Ok(marker) = query.get(entity) else {
        return;
    };

    let size = marker.entity_type.size();
    occupancy.occupy(marker.pos, size, entity);
}
