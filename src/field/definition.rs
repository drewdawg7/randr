use rand::seq::SliceRandom;

use crate::{entities::{mob::MobKind, Mob}, field::enums::FieldError, game_state};



pub struct Field {
    pub name: String,
    pub spawnable_mobs: Vec<MobKind>
}


impl Field {
    
    pub fn new(name: String, spawnable_mobs: Vec<MobKind>) -> Self {
        Self {
            name,
            spawnable_mobs
        }
    }

    pub fn spawn_mob(&self) -> Result<Mob, FieldError> {
        let mk = {
            match  self.spawnable_mobs.choose(&mut rand::thread_rng()) {
                Some(mk) => mk,
                None     => return Err(FieldError::MobSpawnError)
            }
        };
        Ok(game_state().spawn_mob(*mk))
    }
}
