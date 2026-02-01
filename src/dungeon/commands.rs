use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TilePos;

use crate::dungeon::{GridOccupancy, GridSize};

pub trait DungeonCommands {
    fn despawn_dungeon_entity(&mut self, entity: Entity, pos: TilePos, size: GridSize);
}

impl DungeonCommands for Commands<'_, '_> {
    fn despawn_dungeon_entity(&mut self, entity: Entity, pos: TilePos, size: GridSize) {
        self.queue(DespawnDungeonEntity { entity, pos, size });
    }
}

struct DespawnDungeonEntity {
    entity: Entity,
    pos: TilePos,
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
