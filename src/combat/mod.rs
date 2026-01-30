mod attack;
pub mod events;
pub mod plugin;
mod result;
mod system;
mod tests;

pub use events::{DealDamage, EntityDied, PlayerAttackMob};
pub use plugin::{CombatPlugin, PendingVictory};

#[cfg(test)]
pub(crate) use system::{apply_defense, calculate_damage_reduction};
