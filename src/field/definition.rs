use std::collections::HashMap;

use rand::Rng;

use crate::{entities::{mob::MobId, Mob}, field::enums::FieldError, game_state};

pub struct Field {
    pub name: String,
    pub mob_weights: HashMap<MobId, i32>,
}

impl Field {
    pub fn new(name: String, mob_weights: HashMap<MobId, i32>) -> Self {
        Self { name, mob_weights }
    }

    pub fn spawn_mob(&self) -> Result<Mob, FieldError> {
        let total_weight: i32 = self.mob_weights.values().sum();
        if total_weight == 0 {
            return Err(FieldError::MobSpawnError);
        }

        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..total_weight);

        for (mob_kind, weight) in &self.mob_weights {
            roll -= weight;
            if roll < 0 {
                return Ok(game_state().spawn_mob(*mob_kind));
            }
        }
        Err(FieldError::MobSpawnError)
    }
}
