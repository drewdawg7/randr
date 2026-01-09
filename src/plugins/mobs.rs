use bevy::prelude::*;

use crate::mob::Mob;

/// Resource that holds the mob currently in active combat
#[derive(Resource)]
pub struct CurrentMob {
    pub mob: Mob,
}

/// Event fired when a new mob is spawned
#[derive(Event)]
pub struct MobSpawned {
    pub mob: Mob,
}

/// Event fired when a mob takes damage
#[derive(Event)]
pub struct MobDamaged {
    pub damage: i32,
    pub remaining_health: i32,
}

/// Event fired when a mob is defeated
#[derive(Event)]
pub struct MobDefeated {
    pub mob: Mob,
}

/// Plugin that registers mob-related events and resources
pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MobSpawned>()
            .add_event::<MobDamaged>()
            .add_event::<MobDefeated>();
    }
}
