use rand::Rng;

use super::floor_type::FloorType;

#[derive(Debug, Clone)]
pub struct WeightedFloorPool {
    entries: Vec<(FloorType, u32)>,
}

impl WeightedFloorPool {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(mut self, floor_type: FloorType, weight: u32) -> Self {
        self.entries.push((floor_type, weight));
        self
    }

    pub fn select(&self, rng: &mut impl Rng) -> FloorType {
        assert!(!self.entries.is_empty(), "Cannot select from empty pool");

        let total_weight: u32 = self.entries.iter().map(|(_, w)| w).sum();
        let mut roll = rng.gen_range(0..total_weight);

        for (floor_type, weight) in &self.entries {
            if roll < *weight {
                return *floor_type;
            }
            roll -= weight;
        }

        self.entries[0].0
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for WeightedFloorPool {
    fn default() -> Self {
        Self::new()
    }
}
