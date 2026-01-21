use bevy::prelude::*;

/// SystemSets for organizing Town screen systems by function.
/// Configured to run in order: Input -> Logic -> UI
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TownSystemSet {
    /// Handle user input (back action)
    Input,
    /// Update UI based on state changes
    Ui,
}
