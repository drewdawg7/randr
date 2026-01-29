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
use crate::dungeon::tileset::{init_tileset_grid, TilesetGrid};
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
            .init_resource::<TilesetGrid>()
            .add_event::<FloorTransition>()
            .add_event::<FloorReady>()
            .add_event::<SpawnFloor>()
            .add_event::<PlayerMoveIntent>()
            .add_event::<MoveResult>()
            .add_event::<NpcInteraction>()
            .add_event::<CraftingStationInteraction>()
            .add_event::<MineEntity>()
            .add_event::<MiningResult>()
            .add_observer(track_entity_occupancy)
            .add_systems(Startup, init_tileset)
            .add_systems(
                Update,
                (
                    prepare_floor.run_if(on_event::<SpawnFloor>),
                    handle_player_move.run_if(on_event::<PlayerMoveIntent>),
                    handle_floor_transition.run_if(on_event::<FloorTransition>),
                    handle_mine_entity.run_if(on_event::<MineEntity>),
                    handle_mob_defeated.run_if(on_event::<MobDefeated>),
                ),
            );
    }
}

fn init_tileset(
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut tileset: ResMut<TilesetGrid>,
) {
    *tileset = init_tileset_grid(&asset_server, &mut layouts);
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
