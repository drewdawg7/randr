use bevy::prelude::*;

/// Marker for the text of a store list item.
#[derive(Component)]
pub struct StoreListItemText;

/// Source of items for the info panel.
#[derive(Clone, Copy)]
pub enum InfoPanelSource {
    /// Display item from store's inventory
    Store { selected_index: usize },
    /// Display item from player's inventory
    Inventory { selected_index: usize },
}
