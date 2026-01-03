use std::collections::HashMap;

use rand::Rng;

use crate::{
    entities::{mob::MobId, Mob, Player},
    game_state,
    location::{FieldData, LocationId, LocationSpec},
    magic::effect::PassiveEffect,
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

    pub fn spawn_mob(&self, player: &Player) -> Result<Mob, FieldError> {
        // Start with base weights
        let mut adjusted_weights = self.mob_weights.clone();

        // Apply MobSpawnWeight passive effects
        for effect in player.tome_passive_effects() {
            if let PassiveEffect::MobSpawnWeight(mob_id, weight_mod) = effect {
                let entry = adjusted_weights.entry(*mob_id).or_insert(0);
                *entry = (*entry + weight_mod).max(0); // Don't go negative
            }
        }

        let total_weight: i32 = adjusted_weights.values().sum();
        if total_weight == 0 {
            return Err(FieldError::MobSpawnError);
        }

        let mut rng = rand::thread_rng();
        let mut roll = rng.gen_range(0..total_weight);

        for (mob_kind, weight) in &adjusted_weights {
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
