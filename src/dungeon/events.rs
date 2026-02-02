use bevy::prelude::*;

use crate::crafting_station::CraftingStationType;
use crate::dungeon::{EntitySize, FloorType};
use crate::input::NavigationDirection;
use crate::loot::LootDrop;
use crate::mob::MobId;
use crate::rock::RockType;

#[derive(Message, Debug, Clone)]
pub struct PlayerMoveIntent {
    pub direction: NavigationDirection,
}

#[derive(Message, Debug, Clone)]
pub enum MoveResult {
    Moved {
        new_pos: Vec2,
    },
    Blocked,
    TriggeredCombat {
        mob_id: MobId,
        entity: Entity,
        pos: Vec2,
    },
    TriggeredStairs,
    TriggeredDoor,
}

#[derive(Message, Debug, Clone)]
pub enum FloorTransition {
    AdvanceFloor,
    EnterDoor,
    ReturnToHome,
}

#[derive(Message, Debug, Clone)]
pub struct FloorReady {
    pub player_pos: Vec2,
    pub player_size: EntitySize,
    pub floor_type: FloorType,
    pub map_width: usize,
    pub map_height: usize,
}

#[derive(Message, Debug, Clone)]
pub struct NpcInteraction {
    pub mob_id: MobId,
}

#[derive(Message, Debug, Clone)]
pub struct CraftingStationInteraction {
    pub entity: Entity,
    pub station_type: CraftingStationType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MineableEntityType {
    Chest,
    Rock { rock_type: RockType },
}

#[derive(Message, Debug, Clone)]
pub struct MineEntity {
    pub entity: Entity,
    pub pos: Vec2,
    pub mineable_type: MineableEntityType,
}

#[derive(Message, Debug, Clone)]
pub struct MiningResult {
    pub mineable_type: MineableEntityType,
    pub loot_drops: Vec<LootDrop>,
}
