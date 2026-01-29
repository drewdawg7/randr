use bevy::prelude::*;

use crate::crafting_station::CraftingStationType;
use crate::dungeon::{DungeonEntity, DungeonLayout, FloorType, GridPosition, GridSize};
use crate::input::NavigationDirection;
use crate::loot::LootDrop;
use crate::mob::MobId;

#[derive(Event, Debug, Clone)]
pub struct PlayerMoveIntent {
    pub direction: NavigationDirection,
}

#[derive(Event, Debug, Clone)]
pub enum MoveResult {
    Moved {
        new_pos: GridPosition,
    },
    Blocked,
    TriggeredCombat {
        mob_id: MobId,
        entity: Entity,
        pos: GridPosition,
    },
    TriggeredStairs,
    TriggeredDoor,
}

/// Floor transition event (UI -> Game Logic).
#[derive(Event, Debug, Clone)]
pub enum FloorTransition {
    /// Player stepped on stairs - advance to next floor.
    AdvanceFloor,
    /// Player entered a door - enter main dungeon.
    EnterDoor,
    /// Dungeon cleared - return to home.
    ReturnToHome,
}

/// Result when a floor transition completes (Game Logic -> UI).
#[derive(Event, Debug, Clone)]
pub struct FloorReady {
    pub layout: DungeonLayout,
    pub player_pos: GridPosition,
    pub player_size: GridSize,
    pub floor_type: FloorType,
}

/// Player interacted with an NPC.
#[derive(Event, Debug, Clone)]
pub struct NpcInteraction {
    pub mob_id: MobId,
}

/// Player interacted with a crafting station.
#[derive(Event, Debug, Clone)]
pub struct CraftingStationInteraction {
    pub entity: Entity,
    pub station_type: CraftingStationType,
}

/// Player mined an entity (chest or rock).
#[derive(Event, Debug, Clone)]
pub struct MineEntity {
    pub entity: Entity,
    pub pos: GridPosition,
    pub entity_type: DungeonEntity,
}

/// Result of mining an entity (Game Logic -> UI).
#[derive(Event, Debug, Clone)]
pub struct MiningResult {
    pub entity_type: DungeonEntity,
    pub loot_drops: Vec<LootDrop>,
}
