use bevy::prelude::*;
use tracing::instrument;

use crate::dungeon::{DungeonEntityMarker, GridOccupancy};

#[instrument(level = "debug", skip_all, fields(entity = ?trigger.entity()))]
pub fn track_entity_occupancy(
    trigger: On<OnAdd, DungeonEntityMarker>,
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
