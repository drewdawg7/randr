mod combat;
mod definition;
pub mod definitions;
mod loot;
mod progression;
mod stats;

pub mod enums;

pub(crate) use definition::Mob;
// MobId now comes from definitions (macro-generated)
pub(crate) use definitions::MobId;
pub(crate) use definitions::MobRegistry;
