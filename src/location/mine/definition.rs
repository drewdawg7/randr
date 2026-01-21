use std::collections::HashMap;
use std::time::Duration;

use bevy::time::{Timer, TimerMode};

use crate::{
    location::{LocationId, LocationSpec, MineData},
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
    /// Timer for rock respawns
    pub(crate) rock_respawn_timer: Timer,
    /// Timer for mine regeneration
    pub(crate) regeneration_timer: Timer,
}

impl Mine {
    /// Create a Mine from a LocationSpec
    pub fn from_spec(location_id: LocationId, spec: &LocationSpec, data: &MineData) -> Self {
        Mine {
            location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            rock_weights: data.rock_weights.clone(),
            current_rock: None,
            cave: Some(CaveLayout::generate()),
            rock_respawn_timer: Timer::new(ROCK_RESPAWN_INTERVAL, TimerMode::Repeating),
            regeneration_timer: Timer::new(MINE_REGENERATION_INTERVAL, TimerMode::Repeating),
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
            cave: Some(CaveLayout::generate()),
            rock_respawn_timer: Timer::new(ROCK_RESPAWN_INTERVAL, TimerMode::Repeating),
            regeneration_timer: Timer::new(MINE_REGENERATION_INTERVAL, TimerMode::Repeating),
        }
    }

    /// Spawn a new rock based on weighted random selection
    pub fn spawn_rock(&mut self) {
        if let Some(rock_id) = weighted_select(&self.rock_weights) {
            self.current_rock = Some(rock_id.spawn());
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

    // Location trait accessors
    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    /// Check and perform rock respawn if timer finished.
    /// Call tick_timers() first to advance the timers.
    pub fn check_and_respawn_rock(&mut self) {
        if self.rock_respawn_timer.just_finished() {
            if let Some(cave) = &mut self.cave {
                cave.spawn_rock();
            }
        }
    }

    /// Check and regenerate the entire mine if timer finished.
    /// Call tick_timers() first to advance the timers.
    pub fn check_and_regenerate(&mut self) {
        if self.regeneration_timer.just_finished() {
            self.cave = Some(CaveLayout::generate());
            self.rock_respawn_timer.reset(); // Reset rock respawn timer too
        }
    }

    /// Tick all mine timers with the given delta time.
    /// Should be called from the Refreshable::tick implementation.
    pub fn tick_timers(&mut self, delta: Duration) {
        self.rock_respawn_timer.tick(delta);
        self.regeneration_timer.tick(delta);
    }

    /// Returns seconds until next mine regeneration
    pub fn time_until_regeneration(&self) -> u64 {
        self.regeneration_timer.remaining_secs() as u64
    }

    /// Returns seconds until next rock respawn
    pub fn time_until_rock_respawn(&self) -> u64 {
        self.rock_respawn_timer.remaining_secs() as u64
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
            self.regeneration_timer.reset();
            self.rock_respawn_timer.reset();
        }
    }
}
