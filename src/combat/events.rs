//! Combat events for event-driven combat flow.
//!
//! These events enable extension and observation of combat actions.

use bevy::prelude::*;

/// Event fired when damage should be dealt to an entity.
#[derive(Event, Debug, Clone)]
pub struct DealDamage {
    /// The entity receiving damage
    pub target: Entity,
    /// Amount of damage to deal
    pub amount: i32,
    /// Name of the damage source (for combat log)
    pub source_name: String,
}

/// Event fired when an entity dies.
#[derive(Event, Debug, Clone)]
pub struct EntityDied {
    /// The entity that died
    pub entity: Entity,
    /// Whether this was the player
    pub is_player: bool,
}
