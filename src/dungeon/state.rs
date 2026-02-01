use bevy::prelude::*;

use crate::dungeon::floor::FloorId;
use crate::dungeon::systems::spawning::FloorSpawnConfig;
use crate::dungeon::{DungeonLayout, DungeonRegistry, GridPosition, GridSize};
use crate::location::LocationId;

#[derive(Resource, Default)]
pub struct DungeonState {
    pub current_location: Option<LocationId>,
    pub floor_index: usize,
    pub floor_sequence: Vec<FloorId>,
    sequence_location: Option<LocationId>,
    pub dungeon_cleared: bool,
    pub layout: Option<DungeonLayout>,
    pub player_pos: GridPosition,
    pub player_size: GridSize,
}

impl DungeonState {
    pub fn enter_dungeon(&mut self, location: LocationId, registry: &DungeonRegistry) {
        let Some(config) = registry.config(location) else {
            return;
        };

        self.current_location = Some(location);
        self.floor_index = 0;

        let different_location = self.sequence_location != Some(location);
        let should_generate = different_location || self.floor_sequence.is_empty() || self.dungeon_cleared;

        if should_generate {
            self.dungeon_cleared = false;
            self.sequence_location = Some(location);
            self.floor_sequence = config.floors().to_vec();
        }
    }

    pub fn current_floor(&self) -> Option<FloorId> {
        self.floor_sequence.get(self.floor_index).copied()
    }

    pub fn advance_floor(&mut self, registry: &DungeonRegistry) -> Option<FloorId> {
        let location = self.current_location?;
        let config = registry.config(location)?;
        let floor_count = config.floor_count();

        self.floor_index += 1;
        self.layout = None;

        if self.floor_index >= floor_count {
            self.dungeon_cleared = true;
            None
        } else {
            self.floor_sequence.get(self.floor_index).copied()
        }
    }

    pub fn is_current_floor_final(&self, registry: &DungeonRegistry) -> bool {
        let Some(location) = self.current_location else {
            return false;
        };
        let Some(config) = registry.config(location) else {
            return false;
        };
        self.floor_index == config.floor_count().saturating_sub(1)
    }

    pub fn exit_dungeon(&mut self) {
        self.current_location = None;
        self.floor_index = 0;
        self.layout = None;
        self.player_pos = GridPosition::default();
        self.player_size = GridSize::default();
    }

    pub fn load_floor_layout(&mut self) -> Option<(&DungeonLayout, FloorSpawnConfig)> {
        let floor_id = self.current_floor()?;
        let spec = floor_id.spec();
        let layout = spec.layout_id.layout();
        let spawn_config = spec.spawn_table.to_config();

        self.player_pos = layout
            .iter()
            .find(|(_, _, tile)| {
                matches!(
                    tile.tile_type,
                    crate::dungeon::TileType::PlayerSpawn | crate::dungeon::TileType::SpawnPoint
                )
            })
            .map_or(GridPosition::default(), |(x, y, _)| GridPosition::new(x, y));

        self.player_size = GridSize::single();
        self.layout = Some(layout);
        self.layout.as_ref().map(|l| (l, spawn_config))
    }

    pub fn is_in_dungeon(&self) -> bool {
        self.current_location.is_some()
    }

    pub fn reset_dungeon(&mut self) {
        self.floor_sequence.clear();
        self.sequence_location = None;
        self.dungeon_cleared = false;
    }
}
