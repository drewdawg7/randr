mod combat;
pub mod definition;
pub mod definitions;
mod loot;
mod progression;
mod stats;

pub mod enums;

pub use definition::Mob;
// MobId now comes from definitions (macro-generated)
pub use definitions::MobId;
