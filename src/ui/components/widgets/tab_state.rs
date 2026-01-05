//! State machine traits for tab components.
//!
//! Provides `TabState` and `StatefulTab` to unify state handling patterns
//! across tab components (AlchemistTab, BlacksmithTab, StoreTab).

/// Trait for state enums in tab components.
///
/// Each tab has an internal state enum (e.g., `AlchemistState`, `BlacksmithState`)
/// and a corresponding `StateChange` enum for transitions.
pub(crate) trait TabState: Clone + Copy + PartialEq + Default {
    /// The state change enum type for this component.
    type Change;

    /// Apply a state change, returning the new state.
    fn apply_change(current: Self, change: Self::Change) -> Self;
}

/// Trait for tabs that manage internal state machines.
///
/// Provides a consistent interface for state transitions and selection resets.
pub(crate) trait StatefulTab {
    /// The state type for this tab.
    type State: TabState;

    /// Get the current state.
    fn current_state(&self) -> Self::State;

    /// Set the current state.
    fn set_state(&mut self, state: Self::State);

    /// Reset selection state after transition.
    fn reset_selection(&mut self);

    /// Transition to a new state with selection reset.
    fn transition_to(&mut self, new_state: Self::State) {
        self.set_state(new_state);
        self.reset_selection();
    }

    /// Apply a state change with selection reset.
    fn apply_state_change(&mut self, change: <Self::State as TabState>::Change) {
        let new_state = Self::State::apply_change(self.current_state(), change);
        self.transition_to(new_state);
    }
}
