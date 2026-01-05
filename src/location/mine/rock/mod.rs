pub mod art;
mod definition;
pub mod definitions;
mod traits;

pub use definition::Rock;
// RockId now comes from definitions (macro-generated)
pub use definitions::RockId;
