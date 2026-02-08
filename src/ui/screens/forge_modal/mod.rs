mod components;

mod plugin;
mod spawning;
pub mod state;
mod systems;

pub use plugin::ForgeModalPlugin;
pub use state::{ActiveForgeEntity, ForgeModal, ForgeModalState, ForgePlayerGrid, ForgeSlotIndex};
