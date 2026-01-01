use std::collections::HashMap;

use rand::Rng;

use crate::{
    entities::{mob::MobId, Mob},
    game_state,
    location::{FieldData, LocationId, LocationSpec},
};

use super::enums::FieldError;

pub struct Field {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
    pub mob_weights: HashMap<MobId, i32>,
}

impl Field {
    /// Create a Field from a LocationSpec
    pub fn from_spec(spec: &LocationSpec, data: &FieldData) -> Self {
        Field {
            location_id: spec.location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            mob_weights: data.mob_weights.clone(),
        }
    }

    pub fn new(name: String, mob_weights: HashMap<MobId, i32>) -> Self {
        Self {
            location_id: LocationId::VillageField,
            name,
            description: String::new(),
            mob_weights,
        }
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

    // Location trait accessors
    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
