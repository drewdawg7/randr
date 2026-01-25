//! Crafting station types for dungeon entities.

use bevy::prelude::*;

use crate::item::recipe::RecipeId;
use crate::item::ItemId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CraftingStationType {
    Forge,
    Anvil,
}

/// Crafting state attached to forge entities.
/// Tracks items in each slot and whether crafting is in progress.
#[derive(Component, Default, Clone)]
pub struct ForgeCraftingState {
    /// Item in the coal slot (item_id, quantity)
    pub coal_slot: Option<(ItemId, u32)>,
    /// Item in the ore slot (item_id, quantity)
    pub ore_slot: Option<(ItemId, u32)>,
    /// Item in the product slot (crafted ingots)
    pub product_slot: Option<(ItemId, u32)>,
    /// Whether crafting is currently in progress
    pub is_crafting: bool,
}

/// Crafting state attached to anvil entities.
/// Tracks the selected recipe and whether crafting is in progress.
#[derive(Component, Default, Clone)]
pub struct AnvilCraftingState {
    /// The recipe currently being crafted
    pub selected_recipe: Option<RecipeId>,
    /// Whether crafting is currently in progress
    pub is_crafting: bool,
}

impl AnvilCraftingState {
    /// Complete crafting: returns the output ItemId and resets state.
    pub fn complete_crafting(&mut self) -> Option<RecipeId> {
        if !self.is_crafting {
            return None;
        }
        let recipe = self.selected_recipe.take();
        self.is_crafting = false;
        recipe
    }
}

impl ForgeCraftingState {
    /// Check if ready to start crafting (has coal + ore, no product, not already crafting).
    pub fn can_start_crafting(&self) -> bool {
        self.coal_slot.is_some()
            && self.ore_slot.is_some()
            && self.product_slot.is_none()
            && !self.is_crafting
    }

    /// Determine the output ingot based on the ore type.
    pub fn get_output_item(&self) -> Option<ItemId> {
        self.ore_slot.as_ref().map(|(ore_id, _)| match ore_id {
            ItemId::CopperOre => ItemId::CopperIngot,
            ItemId::TinOre => ItemId::TinIngot,
            _ => ItemId::CopperIngot, // Fallback
        })
    }

    /// Complete crafting: consume inputs, produce output.
    /// Crafts min(coal_qty, ore_qty) ingots.
    pub fn complete_crafting(&mut self) {
        let Some(output_id) = self.get_output_item() else {
            return;
        };

        let coal_qty = self.coal_slot.as_ref().map(|(_, q)| *q).unwrap_or(0);
        let ore_qty = self.ore_slot.as_ref().map(|(_, q)| *q).unwrap_or(0);
        let output_qty = coal_qty.min(ore_qty);

        if output_qty > 0 {
            self.coal_slot = None;
            self.ore_slot = None;
            self.product_slot = Some((output_id, output_qty));
        }
        self.is_crafting = false;
    }
}

impl CraftingStationType {
    /// Returns the sprite slice name for this crafting station type.
    pub fn sprite_name(&self) -> &'static str {
        match self {
            Self::Forge => "forge_1_idle",
            Self::Anvil => "anvil_idle",
        }
    }

    /// Returns the display name for this crafting station type.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Forge => "Forge",
            Self::Anvil => "Anvil",
        }
    }
}
