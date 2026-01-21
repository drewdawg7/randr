mod empty_state;
mod list_widget;
mod menu;

pub use empty_state::spawn_empty_state;
pub use list_widget::ListState;
pub use menu::{
    spawn_menu, update_menu_selection, MenuOption, MenuOptionItem, MenuOptionText,
};

/// Source of items for the info panel.
#[derive(Clone, Copy)]
pub enum InfoPanelSource {
    /// Display item from store's inventory
    Store { selected_index: usize },
    /// Display item from player's inventory
    Inventory { selected_index: usize },
}
