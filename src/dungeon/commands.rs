use bevy::ecs::world::Command;
use bevy::prelude::*;

use crate::dungeon::{GridOccupancy, GridPosition, GridSize};

/// Extension trait for dungeon entity operations on `Commands`.
pub trait DungeonCommands {
    /// Despawn a dungeon entity and vacate its position in the occupancy grid.
    fn despawn_dungeon_entity(&mut self, entity: Entity, pos: GridPosition, size: GridSize);
}

impl DungeonCommands for Commands<'_, '_> {
    fn despawn_dungeon_entity(&mut self, entity: Entity, pos: GridPosition, size: GridSize) {
        self.queue(DespawnDungeonEntity { entity, pos, size });
    }
}

struct DespawnDungeonEntity {
    entity: Entity,
    pos: GridPosition,
    size: GridSize,
}

impl Command for DespawnDungeonEntity {
    fn apply(self, world: &mut World) {
        if let Some(mut occupancy) = world.get_resource_mut::<GridOccupancy>() {
            occupancy.vacate(self.pos, self.size);
        }
        if let Ok(entity) = world.get_entity_mut(self.entity) {
            entity.despawn();
        }
    }
}
