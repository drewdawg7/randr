use bevy::prelude::*;

use crate::states::AppState;

/// SystemSets for organizing Town screen systems by function.
/// Configured to run in order: Input -> Logic -> UI
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TownSystemSet {
    /// Handle user input (tab navigation, back action)
    Input,
    /// Update UI based on state changes
    Ui,
}

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::Town)]
pub enum TownTab {
    #[default]
    Store,
}

impl TownTab {
    pub fn name(&self) -> &'static str {
        match self {
            TownTab::Store => "Store",
        }
    }

    pub fn all() -> [TownTab; 1] {
        [TownTab::Store]
    }

    pub fn next(&self) -> Self {
        TownTab::Store
    }

    pub fn prev(&self) -> Self {
        TownTab::Store
    }
}
