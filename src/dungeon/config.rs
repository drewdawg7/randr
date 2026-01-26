use super::floor::{FloorId, WeightedFloorPool};

#[derive(Debug, Clone)]
pub enum DungeonConfig {
    Fixed(Vec<FloorId>),
    Generated {
        floor_count: usize,
        floor_pool: WeightedFloorPool,
    },
}

impl DungeonConfig {
    pub fn fixed(floors: Vec<FloorId>) -> Self {
        DungeonConfig::Fixed(floors)
    }

    pub fn generated(floor_count: usize, floor_pool: WeightedFloorPool) -> Self {
        DungeonConfig::Generated {
            floor_count,
            floor_pool,
        }
    }

    pub fn floor_count(&self) -> usize {
        match self {
            DungeonConfig::Fixed(floors) => floors.len(),
            DungeonConfig::Generated { floor_count, .. } => *floor_count,
        }
    }
}
