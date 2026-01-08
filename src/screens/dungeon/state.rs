use bevy::prelude::*;

/// Different view modes within the Dungeon screen.
/// Each mode represents a different UI state with its own systems and visuals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DungeonViewMode {
    /// Navigation mode - compass + minimap for room selection
    Navigation,
    /// Room entry mode - display room type and available actions
    RoomEntry,
    /// Rest mode - healing interface
    Rest,
    /// Boss mode - boss fight initiation
    Boss,
}

/// Resource tracking the current state within the Dungeon screen.
#[derive(Resource, Default)]
pub struct DungeonScreenState {
    /// Current view mode
    pub mode: DungeonViewMode,
    /// Index of the currently selected action (for menu navigation)
    pub selected_action: usize,
    /// Number of available actions in the current mode
    pub action_count: usize,
}

impl Default for DungeonViewMode {
    fn default() -> Self {
        Self::Navigation
    }
}

impl DungeonScreenState {
    /// Create a new state in Navigation mode
    pub fn new() -> Self {
        Self {
            mode: DungeonViewMode::Navigation,
            selected_action: 0,
            action_count: 0,
        }
    }

    /// Switch to a new mode and reset selection
    pub fn set_mode(&mut self, mode: DungeonViewMode, action_count: usize) {
        self.mode = mode;
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

    /// Check if currently in Navigation mode
    pub fn is_navigation(&self) -> bool {
        self.mode == DungeonViewMode::Navigation
    }

    /// Check if currently in RoomEntry mode
    pub fn is_room_entry(&self) -> bool {
        self.mode == DungeonViewMode::RoomEntry
    }

    /// Check if currently in Rest mode
    pub fn is_rest(&self) -> bool {
        self.mode == DungeonViewMode::Rest
    }

    /// Check if currently in Boss mode
    pub fn is_boss(&self) -> bool {
        self.mode == DungeonViewMode::Boss
    }
}
