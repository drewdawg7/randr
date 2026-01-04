use std::collections::HashMap;

use crate::{
    game_state,
    location::{FieldData, LocationId, LocationSpec},
    magic::effect::PassiveEffect,
    mob::{Mob, MobId},
    player::Player,
    utils::weighted_select,
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

        weighted_select(&adjusted_weights)
            .and_then(|mob_id| game_state().spawn_mob(mob_id))
            .ok_or(FieldError::MobSpawnError)
    }

    // Location trait accessors
    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
