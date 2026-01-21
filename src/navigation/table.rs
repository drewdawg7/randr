use std::collections::HashMap;

use bevy::prelude::*;

use crate::input::GameAction;
use crate::states::AppState;
use crate::ui::screens::modal::ModalType;

/// Target of a navigation action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationTarget {
    /// Navigate to an app state.
    State(AppState),
    /// Open/toggle a modal.
    Modal(ModalType),
}

impl From<AppState> for NavigationTarget {
    fn from(state: AppState) -> Self {
        NavigationTarget::State(state)
    }
}

impl From<ModalType> for NavigationTarget {
    fn from(modal: ModalType) -> Self {
        NavigationTarget::Modal(modal)
    }
}

/// Resource storing all navigation transitions.
#[derive(Resource, Default)]
pub struct NavigationTable {
    /// State-specific transitions: (current_state, action) -> target
    pub(crate) state_transitions: HashMap<(AppState, GameAction), NavigationTarget>,
    /// Global transitions: action -> target (applies in any state)
    pub(crate) global_transitions: HashMap<GameAction, NavigationTarget>,
}

impl NavigationTable {
    /// Look up a navigation target for the given action and current state.
    /// Checks state-specific transitions first, then global.
    pub fn lookup(&self, state: AppState, action: GameAction) -> Option<NavigationTarget> {
        self.state_transitions
            .get(&(state, action))
            .copied()
            .or_else(|| self.global_transitions.get(&action).copied())
    }
}
