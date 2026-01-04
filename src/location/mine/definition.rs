use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::{
    entities::Player,
    game_state,
    location::{LocationId, LocationSpec, MineData},
    magic::effect::PassiveEffect,
    utils::weighted_select,
};

use super::cave::CaveLayout;
use super::rock::{Rock, RockId};

/// How often a new rock spawns (2 minutes)
pub const ROCK_RESPAWN_INTERVAL: Duration = Duration::from_secs(120);

/// How often the entire mine regenerates (10 minutes)
pub const MINE_REGENERATION_INTERVAL: Duration = Duration::from_secs(600);

pub struct Mine {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
    pub rock_weights: HashMap<RockId, i32>,
    pub current_rock: Option<Rock>,
    /// The persistent cave layout
    pub cave: Option<CaveLayout>,
    /// Last time a rock was spawned
    pub(crate) last_rock_respawn: Instant,
    /// Last time the mine was regenerated
    pub(crate) last_regeneration: Instant,
}

impl Mine {
    /// Create a Mine from a LocationSpec
    pub fn from_spec(spec: &LocationSpec, data: &MineData) -> Self {
        let now = Instant::now();
        Mine {
            location_id: spec.location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            rock_weights: data.rock_weights.clone(),
            current_rock: None,
            cave: Some(CaveLayout::generate()),
            last_rock_respawn: now,
            last_regeneration: now,
        }
    }

    pub fn new(name: String) -> Self {
        let mut rock_weights = HashMap::new();
        rock_weights.insert(RockId::Copper, 50);
        rock_weights.insert(RockId::Coal, 30);
        rock_weights.insert(RockId::Tin, 20);

        let now = Instant::now();
        Self {
            location_id: LocationId::VillageMine,
            name,
            description: String::new(),
            rock_weights,
            current_rock: None,
            cave: Some(CaveLayout::generate()),
            last_rock_respawn: now,
            last_regeneration: now,
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

    /// Check and perform rock respawn if interval has elapsed
    /// Called by tick() in the game loop
    pub fn check_and_respawn_rock(&mut self) {
        if self.last_rock_respawn.elapsed() >= ROCK_RESPAWN_INTERVAL {
            if let Some(cave) = &mut self.cave {
                cave.spawn_rock();
            }
            self.last_rock_respawn = Instant::now();
        }
    }

    /// Check and regenerate the entire mine if interval has elapsed
    /// Called by tick() in the game loop
    pub fn check_and_regenerate(&mut self) {
        if self.last_regeneration.elapsed() >= MINE_REGENERATION_INTERVAL {
            self.cave = Some(CaveLayout::generate());
            self.last_regeneration = Instant::now();
            self.last_rock_respawn = Instant::now(); // Reset rock respawn timer too
        }
    }

    /// Returns seconds until next mine regeneration
    pub fn time_until_regeneration(&self) -> u64 {
        let elapsed = self.last_regeneration.elapsed();
        if elapsed >= MINE_REGENERATION_INTERVAL {
            0
        } else {
            (MINE_REGENERATION_INTERVAL - elapsed).as_secs()
        }
    }

    /// Returns seconds until next rock respawn
    pub fn time_until_rock_respawn(&self) -> u64 {
        let elapsed = self.last_rock_respawn.elapsed();
        if elapsed >= ROCK_RESPAWN_INTERVAL {
            0
        } else {
            (ROCK_RESPAWN_INTERVAL - elapsed).as_secs()
        }
    }

    /// Get a mutable reference to the cave layout
    pub fn cave_mut(&mut self) -> Option<&mut CaveLayout> {
        self.cave.as_mut()
    }

    /// Get a reference to the cave layout
    pub fn cave(&self) -> Option<&CaveLayout> {
        self.cave.as_ref()
    }

    /// Ensure the cave exists, generating one if needed
    pub fn ensure_cave_exists(&mut self) {
        if self.cave.is_none() {
            self.cave = Some(CaveLayout::generate());
            self.last_regeneration = Instant::now();
            self.last_rock_respawn = Instant::now();
        }
    }
}
