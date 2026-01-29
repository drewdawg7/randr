//! Combat plugin - registers combat events and resources.

use bevy::prelude::*;

use super::events::{DealDamage, EntityDied};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DealDamage>().add_event::<EntityDied>();
    }
}
