use bevy::ecs::system::Command;
use bevy::prelude::*;

use crate::dungeon::Occupancy;

pub trait DungeonCommands {
    fn despawn_dungeon_entity(&mut self, entity: Entity);
}

impl DungeonCommands for Commands<'_, '_> {
    fn despawn_dungeon_entity(&mut self, entity: Entity) {
        self.queue(DespawnDungeonEntity { entity });
    }
}

struct DespawnDungeonEntity {
    entity: Entity,
}

impl Command for DespawnDungeonEntity {
    fn apply(self, world: &mut World) {
        if let Some(mut occupancy) = world.get_resource_mut::<Occupancy>() {
            occupancy.vacate(self.entity);
        }
        if let Ok(entity) = world.get_entity_mut(self.entity) {
            entity.despawn();
        }
    }
}
