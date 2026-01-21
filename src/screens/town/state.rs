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
    Blacksmith,
    Alchemist,
}

impl TownTab {
    pub fn name(&self) -> &'static str {
        match self {
            TownTab::Store => "Store",
            TownTab::Blacksmith => "Blacksmith",
            TownTab::Alchemist => "Alchemist",
        }
    }

    pub fn all() -> [TownTab; 3] {
        [TownTab::Store, TownTab::Blacksmith, TownTab::Alchemist]
    }

    pub fn next(&self) -> Self {
        match self {
            TownTab::Store => TownTab::Blacksmith,
            TownTab::Blacksmith => TownTab::Alchemist,
            TownTab::Alchemist => TownTab::Store,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            TownTab::Store => TownTab::Alchemist,
            TownTab::Blacksmith => TownTab::Store,
            TownTab::Alchemist => TownTab::Blacksmith,
        }
    }
}
