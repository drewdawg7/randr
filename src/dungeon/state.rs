use bevy::prelude::*;

use crate::dungeon::floor::FloorId;
use crate::dungeon::spawn::SpawnTable;
use crate::dungeon::DungeonRegistry;
use crate::location::LocationId;

#[derive(Resource, Clone, Copy, Debug)]
pub struct TileWorldSize(pub f32);

impl Default for TileWorldSize {
    fn default() -> Self {
        Self(32.0)
    }
}

#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TilemapInfo {
    pub tile_size: Vec2,
    pub world_size: Vec2,
    pub center: Vec2,
}

#[derive(Resource, Default)]
pub struct DungeonState {
    pub current_location: Option<LocationId>,
    pub floor_index: usize,
    pub floor_sequence: Vec<FloorId>,
    sequence_location: Option<LocationId>,
    pub dungeon_cleared: bool,
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
    }

    pub fn get_spawn_config(&self) -> Option<SpawnTable> {
        let floor_id = self.current_floor()?;
        let spec = floor_id.spec();
        Some(spec.spawn_table.clone())
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
