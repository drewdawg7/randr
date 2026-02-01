use bevy::prelude::*;
use tracing::instrument;

use crate::dungeon::{DungeonEntityMarker, Occupancy};

#[instrument(level = "debug", skip_all, fields(entity = ?trigger.entity))]
pub fn track_entity_occupancy(
    trigger: On<Add, DungeonEntityMarker>,
    query: Query<&DungeonEntityMarker>,
    occupancy: Option<ResMut<Occupancy>>,
) {
    let Some(mut occupancy) = occupancy else {
        return;
    };
    let entity = trigger.entity;
    let Ok(marker) = query.get(entity) else {
        return;
    };

    let size = marker.entity_type.size();
    occupancy.occupy(marker.pos, size, entity);
}
