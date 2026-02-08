//! Anvil modal for crafting equipment from ingots.


mod plugin;
pub(crate) mod render;
mod state;

pub use plugin::AnvilModalPlugin;
pub use state::{ActiveAnvilEntity, AnvilModal, AnvilPlayerGrid, AnvilRecipeGrid};
