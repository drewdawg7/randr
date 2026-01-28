use super::floor::FloorId;

#[derive(Debug, Clone)]
pub struct DungeonConfig {
    floors: Vec<FloorId>,
}

impl DungeonConfig {
    pub fn new(floors: Vec<FloorId>) -> Self {
        Self { floors }
    }

    pub fn floors(&self) -> &[FloorId] {
        &self.floors
    }

    pub fn floor_count(&self) -> usize {
        self.floors.len()
    }
}
