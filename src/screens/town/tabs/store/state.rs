use bevy::prelude::*;

use crate::screens::town::shared::SelectionState;

/// Store mode kind - what submenu the player is in.
/// Flattened to avoid nested state dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StoreModeKind {
    #[default]
    Menu,
    Buy,
    Sell,
    StorageMenu,
    StorageView,
    StorageDeposit,
}

/// Which panel is focused in the buy screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuyFocus {
    #[default]
    Store,
    Inventory,
}

/// Store mode - tracks navigation state within the tab.
#[derive(Resource, Default)]
pub struct StoreMode {
    pub mode: StoreModeKind,
}

/// Store selections - tracks cursor positions in each mode.
#[derive(Resource)]
pub struct StoreSelections {
    pub menu: SelectionState,
    pub buy: SelectionState,
    pub buy_inventory: SelectionState,
    pub buy_focus: BuyFocus,
    pub sell: SelectionState,
    pub storage_menu: SelectionState,
    pub storage_view: SelectionState,
    pub deposit: SelectionState,
}

impl Default for StoreSelections {
    fn default() -> Self {
        Self {
            menu: SelectionState {
                selected: 0,
                count: 3, // Buy, Sell, Storage
                scroll_offset: 0,
                visible_count: 10,
            },
            buy: SelectionState::new(0),
            buy_inventory: SelectionState::new(0),
            buy_focus: BuyFocus::default(),
            sell: SelectionState::new(0),
            storage_menu: SelectionState {
                selected: 0,
                count: 2, // View Storage, Deposit Items
                scroll_offset: 0,
                visible_count: 10,
            },
            storage_view: SelectionState::new(0),
            deposit: SelectionState::new(0),
        }
    }
}
