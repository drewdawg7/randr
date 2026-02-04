use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct TryStartForgeCrafting {
    pub entity: Entity,
}

#[derive(Message, Debug, Clone)]
pub struct ForgeCraftingStarted {
    pub entity: Entity,
}

#[derive(Message, Debug, Clone)]
pub struct TryStartAnvilCrafting {
    pub entity: Entity,
}

#[derive(Message, Debug, Clone)]
pub struct AnvilCraftingStarted {
    pub entity: Entity,
}
