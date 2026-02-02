use bevy::prelude::*;

use crate::crafting_station::CraftingStationType;
use crate::mob::MobId;
use crate::rock::RockType;

use super::grid::EntitySize;

#[derive(Component)]
pub struct DungeonEntityMarker {
    pub pos: Vec2,
    pub size: EntitySize,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct ChestEntity {
    pub variant: u8,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct RockEntity {
    pub rock_type: RockType,
    pub sprite_variant: u8,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct StairsEntity;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct MobEntity {
    pub mob_id: MobId,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct NpcEntity {
    pub mob_id: MobId,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct CraftingStationEntity {
    pub station_type: CraftingStationType,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct DoorEntity;
