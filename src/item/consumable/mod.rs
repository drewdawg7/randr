pub(crate) mod definition;
pub(crate) mod enums;
pub(crate) mod traits;
pub(crate) mod system;

pub(crate) use definition::{ApplyEffect, ConsumableEffect, ConsumableRegistry};
pub(crate) use enums::ConsumableError;
pub(crate) use system::use_consumable;
