//! Crafting station types for dungeon entities.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CraftingStationType {
    Forge,
}

impl CraftingStationType {
    /// Returns the sprite slice name for this crafting station type.
    pub fn sprite_name(&self) -> &'static str {
        match self {
            Self::Forge => "forge_1_idle",
        }
    }

    /// Returns the display name for this crafting station type.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Forge => "Forge",
        }
    }
}
