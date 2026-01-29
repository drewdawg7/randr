mod attack;
pub mod events;
pub mod plugin;
mod result;
mod system;
mod traits;
mod tests;

pub(crate) use attack::*;
pub use events::{DealDamage, EntityDied};
pub use plugin::CombatPlugin;
pub(crate) use result::*;
pub(crate) use system::*;
pub(crate) use traits::*;
