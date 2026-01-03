use std::collections::HashMap;

use crate::{
    entities::Player,
    game_state,
    location::{LocationId, LocationSpec, MineData},
    magic::effect::PassiveEffect,
    utils::weighted_select,
};

use super::rock::{Rock, RockId};

pub struct Mine {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
    pub rock_weights: HashMap<RockId, i32>,
    pub current_rock: Option<Rock>,
}

impl Mine {
    /// Create a Mine from a LocationSpec
    pub fn from_spec(spec: &LocationSpec, data: &MineData) -> Self {
        Mine {
            location_id: spec.location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            rock_weights: data.rock_weights.clone(),
            current_rock: None,
        }
    }

    pub fn new(name: String) -> Self {
        let mut rock_weights = HashMap::new();
        rock_weights.insert(RockId::Copper, 50);
        rock_weights.insert(RockId::Coal, 30);
        rock_weights.insert(RockId::Tin, 20);

        Self {
            location_id: LocationId::VillageMine,
            name,
            description: String::new(),
            rock_weights,
            current_rock: None,
        }
    }

    /// Spawn a new rock based on weighted random selection
    pub fn spawn_rock(&mut self, player: &Player) {
        // Start with base weights
        let mut adjusted_weights = self.rock_weights.clone();

        // Apply RockSpawnWeight passive effects
        for effect in player.tome_passive_effects() {
            if let PassiveEffect::RockSpawnWeight(rock_id, weight_mod) = effect {
                let entry = adjusted_weights.entry(*rock_id).or_insert(0);
                *entry = (*entry + weight_mod).max(0); // Don't go negative
            }
        }

        if let Some(rock_id) = weighted_select(&adjusted_weights) {
            self.current_rock = game_state().spawn_rock(rock_id);
        }
    }

    /// Ensure a rock exists, spawning one if needed
    pub fn ensure_rock_exists(&mut self, player: &Player) {
        if self.current_rock.is_none() {
            self.spawn_rock(player);
        }
    }

    /// Get a mutable reference to the current rock
    pub fn current_rock_mut(&mut self) -> Option<&mut Rock> {
        self.current_rock.as_mut()
    }

    // Location trait accessors
    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
