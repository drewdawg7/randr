use avian2d::prelude::{CollisionLayers, RigidBody, Sensor};
use bevy::prelude::*;

use crate::crafting_station::CraftingStationType;
use crate::mob::MobId;
use crate::rock::RockType;

use super::grid::EntitySize;
use super::physics::{mob_layers, static_entity_layers, trigger_layers};

#[derive(Component)]
pub struct DungeonEntityMarker {
    pub pos: Vec2,
    pub size: EntitySize,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
#[require(RigidBody::Static, CollisionLayers = static_entity_layers())]
pub struct ChestEntity {
    pub variant: u8,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
#[require(RigidBody::Static, CollisionLayers = static_entity_layers())]
pub struct RockEntity {
    pub rock_type: RockType,
    pub sprite_variant: u8,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
#[require(RigidBody::Static, Sensor, CollisionLayers = trigger_layers())]
pub struct StairsEntity;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
#[require(RigidBody::Kinematic, CollisionLayers = mob_layers())]
pub struct MobEntity {
    pub mob_id: MobId,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
#[require(RigidBody::Kinematic, CollisionLayers = mob_layers())]
pub struct NpcEntity {
    pub mob_id: MobId,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
#[require(RigidBody::Static, CollisionLayers = static_entity_layers())]
pub struct CraftingStationEntity {
    pub station_type: CraftingStationType,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
#[require(RigidBody::Static, Sensor, CollisionLayers = trigger_layers())]
pub struct DoorEntity;
