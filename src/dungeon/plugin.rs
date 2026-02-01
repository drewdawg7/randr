use std::collections::HashMap;

use bevy::prelude::*;

use crate::dungeon::config::DungeonConfig;
use crate::dungeon::events::{
    CraftingStationInteraction, FloorReady, FloorTransition, MineEntity, MiningResult, MoveResult,
    NpcInteraction, PlayerMoveIntent,
};
use crate::plugins::MobDefeated;
use crate::dungeon::floor::FloorId;
use crate::dungeon::state::DungeonState;
use crate::dungeon::systems::{
    handle_floor_transition, handle_mine_entity, handle_mob_defeated, handle_player_move,
    prepare_floor, track_entity_occupancy, SpawnFloor,
};
use crate::location::LocationId;

#[derive(Resource, Default)]
pub struct FloorMonsterCount(pub usize);

#[derive(Resource, Clone, Debug)]
pub struct DungeonRegistry {
    configs: HashMap<LocationId, DungeonConfig>,
}

impl DungeonRegistry {
    pub fn config(&self, location: LocationId) -> Option<&DungeonConfig> {
        self.configs.get(&location)
    }

    pub fn floors(&self, location: LocationId) -> &[FloorId] {
        self.configs
            .get(&location)
            .map(|c| c.floors())
            .unwrap_or(&[])
    }

    pub fn next_floor(&self, location: LocationId, current: FloorId) -> Option<FloorId> {
        let floors = self.floors(location);
        floors
            .iter()
            .position(|&f| f == current)
            .and_then(|idx| floors.get(idx + 1))
            .copied()
    }

    pub fn is_final_floor(&self, location: LocationId, floor: FloorId) -> bool {
        let floors = self.floors(location);
        floors.last() == Some(&floor)
    }
}

pub struct DungeonPlugin {
    registry: DungeonRegistry,
}

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.registry.clone())
            .init_resource::<DungeonState>()
            .add_message::<FloorTransition>()
            .add_message::<FloorReady>()
            .add_message::<SpawnFloor>()
            .add_message::<PlayerMoveIntent>()
            .add_message::<MoveResult>()
            .add_message::<NpcInteraction>()
            .add_message::<CraftingStationInteraction>()
            .add_message::<MineEntity>()
            .add_message::<MiningResult>()
            .add_observer(track_entity_occupancy)
            .add_systems(
                Update,
                (
                    prepare_floor.run_if(on_message::<SpawnFloor>),
                    handle_player_move.run_if(on_message::<PlayerMoveIntent>),
                    handle_floor_transition.run_if(on_message::<FloorTransition>),
                    handle_mine_entity.run_if(on_message::<MineEntity>),
                    handle_mob_defeated.run_if(on_message::<MobDefeated>),
                ),
            );
    }
}

impl DungeonPlugin {
    pub fn new() -> DungeonBuilder {
        DungeonBuilder {
            configs: HashMap::new(),
            current_location: None,
        }
    }
}

impl Default for DungeonPlugin {
    fn default() -> Self {
        Self {
            registry: DungeonRegistry {
                configs: HashMap::new(),
            },
        }
    }
}

pub struct DungeonBuilder {
    configs: HashMap<LocationId, DungeonConfig>,
    current_location: Option<LocationId>,
}

impl DungeonBuilder {
    pub fn location(mut self, id: LocationId) -> Self {
        self.current_location = Some(id);
        self.configs
            .entry(id)
            .or_insert(DungeonConfig::new(Vec::new()));
        self
    }

    pub fn floor(mut self, floor: FloorId) -> Self {
        let location = self
            .current_location
            .expect("floor() called before location()");
        if let Some(config) = self.configs.get_mut(&location) {
            let mut floors = config.floors().to_vec();
            floors.push(floor);
            *config = DungeonConfig::new(floors);
        }
        self
    }

    pub fn build(self) -> DungeonPlugin {
        assert!(
            !self.configs.is_empty(),
            "DungeonPlugin requires at least one location to be registered"
        );

        DungeonPlugin {
            registry: DungeonRegistry {
                configs: self.configs,
            },
        }
    }
}
