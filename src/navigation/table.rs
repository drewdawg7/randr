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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_target_from_app_state() {
        let target: NavigationTarget = AppState::Dungeon.into();
        assert_eq!(target, NavigationTarget::State(AppState::Dungeon));
    }

    #[test]
    fn navigation_target_from_modal_type() {
        let target: NavigationTarget = ModalType::Inventory.into();
        assert_eq!(target, NavigationTarget::Modal(ModalType::Inventory));
    }

    #[test]
    fn lookup_returns_state_specific_transition() {
        let mut table = NavigationTable::default();
        table.state_transitions.insert(
            (AppState::Dungeon, GameAction::Select),
            NavigationTarget::State(AppState::Dungeon),
        );

        let result = table.lookup(AppState::Dungeon, GameAction::Select);
        assert_eq!(result, Some(NavigationTarget::State(AppState::Dungeon)));
    }

    #[test]
    fn lookup_returns_none_for_missing_transition() {
        let table = NavigationTable::default();
        let result = table.lookup(AppState::Dungeon, GameAction::Select);
        assert!(result.is_none());
    }

    #[test]
    fn lookup_returns_global_transition_when_no_state_specific() {
        let mut table = NavigationTable::default();
        table.global_transitions.insert(
            GameAction::OpenInventory,
            NavigationTarget::Modal(ModalType::Inventory),
        );

        let result = table.lookup(AppState::Dungeon, GameAction::OpenInventory);
        assert_eq!(result, Some(NavigationTarget::Modal(ModalType::Inventory)));

        let result2 = table.lookup(AppState::Dungeon, GameAction::OpenInventory);
        assert_eq!(result2, Some(NavigationTarget::Modal(ModalType::Inventory)));
    }

    #[test]
    fn lookup_state_specific_overrides_global() {
        let mut table = NavigationTable::default();
        table.global_transitions.insert(
            GameAction::Select,
            NavigationTarget::State(AppState::Menu),
        );
        table.state_transitions.insert(
            (AppState::Dungeon, GameAction::Select),
            NavigationTarget::State(AppState::Dungeon),
        );

        let town_result = table.lookup(AppState::Dungeon, GameAction::Select);
        assert_eq!(town_result, Some(NavigationTarget::State(AppState::Dungeon)));

        let menu_result = table.lookup(AppState::Menu, GameAction::Select);
        assert_eq!(menu_result, Some(NavigationTarget::State(AppState::Menu)));
    }

    #[test]
    fn default_table_is_empty() {
        let table = NavigationTable::default();
        assert!(table.state_transitions.is_empty());
        assert!(table.global_transitions.is_empty());
    }
}
