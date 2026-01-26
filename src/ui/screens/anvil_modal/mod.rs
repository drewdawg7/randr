//! Anvil modal for crafting equipment from ingots.

mod input;
mod plugin;
mod render;
mod state;

pub use plugin::AnvilModalPlugin;
pub use state::{ActiveAnvilEntity, AnvilModal, SpawnAnvilModal};
