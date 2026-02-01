use bevy::ecs::system::Command;
use bevy::prelude::*;

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
        if let Ok(entity) = world.get_entity_mut(self.entity) {
            entity.despawn();
        }
    }
}
