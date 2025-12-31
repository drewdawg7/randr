use std::collections::HashMap;
use rand::Rng;

use crate::mine::rock::{Rock, RockId};

pub struct Mine {
    pub name: String,
    pub rock_weights: HashMap<RockId, i32>,
    pub rocks: HashMap<RockId, Rock>,
    pub current_rock: Option<Rock>,
}

impl Mine {
    pub fn new(name: String) -> Self {
        let mut rock_weights = HashMap::new();
        rock_weights.insert(RockId::Copper, 50);
        rock_weights.insert(RockId::Coal, 30);
        rock_weights.insert(RockId::Tin, 20);

        let mut rocks = HashMap::new();
        rocks.insert(RockId::Copper, Rock::copper_rock());
        rocks.insert(RockId::Coal, Rock::coal_rock());
        rocks.insert(RockId::Tin, Rock::tin_rock());

        Self {
            name,
            rock_weights,
            rocks,
            current_rock: None,
        }
    }

    /// Spawn a new rock based on weighted random selection
    pub fn spawn_rock(&mut self) {
        let total_weight: i32 = self.rock_weights.values().sum();
        if total_weight == 0 {
            return;
        }

        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..total_weight);

        for (rock_id, weight) in &self.rock_weights {
            roll -= weight;
            if roll < 0 {
                if let Some(rock_template) = self.rocks.get(rock_id) {
                    self.current_rock = Some(rock_template.clone());
                }
                break;
            }
        }
    }

    /// Ensure a rock exists, spawning one if needed
    pub fn ensure_rock_exists(&mut self) {
        if self.current_rock.is_none() {
            self.spawn_rock();
        }
    }

    /// Get a mutable reference to the current rock
    pub fn current_rock_mut(&mut self) -> Option<&mut Rock> {
        self.current_rock.as_mut()
    }
}
