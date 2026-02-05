use bevy::prelude::*;

use crate::crafting_station::CraftingStationType;
use crate::dungeon::FloorId;
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
    pub floor_id: FloorId,
}

#[derive(Message, Debug, Clone)]
pub struct CraftingStationInteraction {
    pub entity: Entity,
    pub station_type: CraftingStationType,
}

#[derive(Event, Debug, Clone)]
pub struct MerchantInteraction {
    pub entity: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ChestMined {
    pub entity: Entity,
    pub pos: Vec2,
}

#[derive(Event, Debug, Clone)]
pub struct RockMined {
    pub entity: Entity,
    pub pos: Vec2,
    pub rock_type: RockType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MineableEntityType {
    Chest,
    Rock { rock_type: RockType },
}

#[derive(Message, Debug, Clone)]
pub struct MiningResult {
    pub mineable_type: MineableEntityType,
    pub loot_drops: Vec<LootDrop>,
}

#[derive(Resource, Default, Debug)]
pub struct OverlappingCraftingStation(pub Option<Entity>);
