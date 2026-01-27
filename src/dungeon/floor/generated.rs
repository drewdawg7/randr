use crate::dungeon::layouts::LayoutId;
use crate::dungeon::spawn::SpawnTable;

use super::definitions::FloorId;
use super::floor_type::FloorType;

#[derive(Debug, Clone)]
pub struct GeneratedFloor {
    pub floor_type: FloorType,
    pub floor_number: usize,
    pub is_final: bool,
}

impl GeneratedFloor {
    pub fn layout_id(&self) -> LayoutId {
        self.floor_type.layout_id(self.is_final)
    }

    pub fn spawn_table(&self) -> SpawnTable {
        self.floor_type.spawn_table(self.is_final)
    }

    pub fn name(&self) -> String {
        format!("Floor {}", self.floor_number)
    }
}

#[derive(Debug, Clone)]
pub enum FloorInstance {
    Fixed(FloorId),
    Generated(GeneratedFloor),
}

impl FloorInstance {
    pub fn layout_id(&self) -> LayoutId {
        match self {
            FloorInstance::Fixed(floor_id) => floor_id.spec().layout_id,
            FloorInstance::Generated(generated) => generated.layout_id(),
        }
    }

    pub fn spawn_table(&self) -> SpawnTable {
        match self {
            FloorInstance::Fixed(floor_id) => floor_id.spec().spawn_table.clone(),
            FloorInstance::Generated(generated) => generated.spawn_table(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            FloorInstance::Fixed(floor_id) => floor_id.spec().name.to_string(),
            FloorInstance::Generated(generated) => generated.name(),
        }
    }

    /// Returns the floor type for tileset rendering.
    /// Fixed floors use BasicDungeonFloor (standard dungeon tileset).
    pub fn floor_type(&self) -> FloorType {
        match self {
            FloorInstance::Fixed(_) => FloorType::BasicDungeonFloor,
            FloorInstance::Generated(generated) => generated.floor_type,
        }
    }
}
