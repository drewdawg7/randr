use bevy::prelude::*;

use crate::states::AppState;

/// Different view modes within the Dungeon screen.
/// Each mode represents a different UI state with its own systems and visuals.
#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::Dungeon)]
pub enum DungeonMode {
    /// Navigation mode - compass + minimap for room selection
    #[default]
    Navigation,
    /// Room entry mode - display room type and available actions
    RoomEntry,
    /// Rest mode - healing interface
    Rest,
    /// Boss mode - boss fight initiation
    Boss,
}

/// Resource tracking the menu selection state within the Dungeon screen.
#[derive(Resource, Default)]
pub struct DungeonSelectionState {
    /// Index of the currently selected action (for menu navigation)
    pub selected_action: usize,
    /// Number of available actions in the current mode
    pub action_count: usize,
}

impl DungeonSelectionState {
    /// Reset selection for a new mode
    pub fn reset(&mut self, action_count: usize) {
        self.selected_action = 0;
        self.action_count = action_count;
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_action > 0 {
            self.selected_action -= 1;
        }
    }

    /// Move selection down
    pub fn move_down(&mut self) {
        if self.selected_action + 1 < self.action_count {
            self.selected_action += 1;
        }
    }
}
