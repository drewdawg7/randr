pub mod bundle;
mod combat;
pub mod components;
pub mod definition;
pub mod definitions;
mod loot;
mod progression;
mod stats;

pub mod enums;

pub use bundle::MobCombatBundle;
pub use components::{CombatStats, DeathProcessed, GoldReward, Health, MobLootTable, MobMarker, XpReward};
pub use definition::Mob;
// MobId now comes from definitions (macro-generated)
pub use definitions::MobId;
