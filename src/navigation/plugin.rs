use bevy::prelude::*;

use crate::input::GameAction;
use crate::states::AppState;

use super::systems::handle_navigation;
use super::table::{NavigationTable, NavigationTarget};

/// Plugin that provides declarative navigation between states and modals.
pub struct NavigationPlugin {
    table: NavigationTable,
}

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.table.clone())
            .add_systems(Update, handle_navigation.run_if(on_message::<GameAction>));
    }
}

impl NavigationPlugin {
    /// Create a new navigation builder.
    pub fn new() -> NavigationBuilder {
        NavigationBuilder {
            table: NavigationTable::default(),
            current_state: None,
        }
    }
}

impl Default for NavigationPlugin {
    fn default() -> Self {
        Self {
            table: NavigationTable::default(),
        }
    }
}

/// Builder for configuring navigation transitions.
pub struct NavigationBuilder {
    table: NavigationTable,
    current_state: Option<AppState>,
}

impl NavigationBuilder {
    /// Configure transitions for a specific state.
    pub fn state(mut self, state: AppState) -> Self {
        self.current_state = Some(state);
        self
    }

    /// Configure global transitions (apply in any state).
    pub fn global(mut self) -> Self {
        self.current_state = None;
        self
    }

    /// Add a transition from the current context.
    pub fn on(mut self, action: GameAction, target: impl Into<NavigationTarget>) -> Self {
        let target = target.into();
        if let Some(state) = self.current_state {
            self.table.state_transitions.insert((state, action), target);
        } else {
            self.table.global_transitions.insert(action, target);
        }
        self
    }

    /// Build the navigation plugin.
    pub fn build(self) -> NavigationPlugin {
        NavigationPlugin { table: self.table }
    }
}

impl Clone for NavigationTable {
    fn clone(&self) -> Self {
        Self {
            state_transitions: self.state_transitions.clone(),
            global_transitions: self.global_transitions.clone(),
        }
    }
}
