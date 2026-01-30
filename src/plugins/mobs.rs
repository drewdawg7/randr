use bevy::prelude::*;

use crate::mob::MobId;

/// Event fired when a mob is defeated (ECS-based combat)
#[derive(Event)]
pub struct MobDefeated {
    pub mob_id: MobId,
}

/// Plugin that registers mob-related events
pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MobDefeated>();
    }
}
