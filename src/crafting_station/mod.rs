mod anvil;
mod events;
mod forge;
mod plugin;

pub use anvil::AnvilCraftingState;
pub use events::{
    AnvilCraftingStarted, ForgeCraftingStarted, TryStartAnvilCrafting, TryStartForgeCrafting,
};
pub use forge::ForgeCraftingState;
pub use plugin::CraftingStationPlugin;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CraftingStationType {
    Forge,
    Anvil,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ForgeTimerFinished {
    pub entity: Entity,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct AnvilTimerFinished {
    pub entity: Entity,
}

#[derive(Component)]
pub struct ForgeActiveTimer(pub Timer);

#[derive(Component)]
pub struct AnvilActiveTimer(pub Timer);

impl CraftingStationType {
    pub fn sprite_name(&self) -> &'static str {
        match self {
            Self::Forge => "forge_1_idle",
            Self::Anvil => "anvil_idle",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Forge => "Forge",
            Self::Anvil => "Anvil",
        }
    }
}
