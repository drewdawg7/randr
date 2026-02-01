use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TilePos;

use crate::crafting_station::CraftingStationType;
use crate::dungeon::{DungeonEntity, FloorType, GridSize};
use crate::input::NavigationDirection;
use crate::loot::LootDrop;
use crate::mob::MobId;

#[derive(Message, Debug, Clone)]
pub struct PlayerMoveIntent {
    pub direction: NavigationDirection,
}

#[derive(Message, Debug, Clone)]
pub enum MoveResult {
    Moved {
        new_pos: TilePos,
    },
    Blocked,
    TriggeredCombat {
        mob_id: MobId,
        entity: Entity,
        pos: TilePos,
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
    pub player_pos: TilePos,
    pub player_size: GridSize,
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

#[derive(Message, Debug, Clone)]
pub struct MineEntity {
    pub entity: Entity,
    pub pos: TilePos,
    pub entity_type: DungeonEntity,
}

#[derive(Message, Debug, Clone)]
pub struct MiningResult {
    pub entity_type: DungeonEntity,
    pub loot_drops: Vec<LootDrop>,
}
