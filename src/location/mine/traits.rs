use std::collections::HashMap;
use std::time::Duration;

use bevy::time::{Timer, TimerMode};

use crate::location::{Location, LocationId, Refreshable};

use super::cave::CaveLayout;
use super::definition::{Mine, MINE_REGENERATION_INTERVAL, ROCK_RESPAWN_INTERVAL};
use super::rock::RockId;

impl Default for Mine {
    fn default() -> Self {
        let mut rock_weights = HashMap::new();
        rock_weights.insert(RockId::Gold, 2);
        rock_weights.insert(RockId::Coal, 2);
        rock_weights.insert(RockId::Iron, 2);
        rock_weights.insert(RockId::Mixed, 1);
        Self {
            location_id: LocationId::VillageMine,
            name: "Village Mine".to_string(),
            description: String::new(),
            rock_weights,
            current_rock: None,
            cave: Some(CaveLayout::generate()),
            rock_respawn_timer: Timer::new(ROCK_RESPAWN_INTERVAL, TimerMode::Repeating),
            regeneration_timer: Timer::new(MINE_REGENERATION_INTERVAL, TimerMode::Repeating),
        }
    }
}

impl Location for Mine {
    fn id(&self) -> LocationId {
        self.location_id()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        Mine::description(self)
    }
}

impl Refreshable for Mine {
    fn tick(&mut self, elapsed: Duration) {
        self.tick_timers(elapsed);
        self.check_and_regenerate();
        self.check_and_respawn_rock();
    }

    fn refresh(&mut self) {
        self.cave = Some(CaveLayout::generate());
        self.regeneration_timer.reset();
        self.rock_respawn_timer.reset();
    }

    fn time_until_refresh(&self) -> Duration {
        Duration::from_secs(self.time_until_regeneration())
    }
}
