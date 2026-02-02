use bevy::prelude::*;

#[derive(Message, Debug, Clone, Copy)]
pub enum StateTransitionRequest {
    Menu,
    Dungeon,
    Profile,
    Keybinds,
}

impl StateTransitionRequest {
    pub fn target_state(self) -> AppState {
        match self {
            StateTransitionRequest::Menu => AppState::Menu,
            StateTransitionRequest::Dungeon => AppState::Dungeon,
            StateTransitionRequest::Profile => AppState::Profile,
            StateTransitionRequest::Keybinds => AppState::Keybinds,
        }
    }
}

impl From<AppState> for StateTransitionRequest {
    fn from(state: AppState) -> Self {
        match state {
            AppState::Menu => StateTransitionRequest::Menu,
            AppState::Dungeon => StateTransitionRequest::Dungeon,
            AppState::Profile => StateTransitionRequest::Profile,
            AppState::Keybinds => StateTransitionRequest::Keybinds,
        }
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Dungeon,
    Profile,
    Keybinds,
}

#[derive(Resource, Default)]
pub struct PreviousState {
    pub state: Option<AppState>,
    pub just_entered: bool,
}

impl PreviousState {
    pub fn came_from(&self, state: AppState) -> bool {
        self.just_entered && self.state == Some(state)
    }

    pub fn acknowledge_entry(&mut self) {
        self.just_entered = false;
    }
}

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<PreviousState>()
            .add_message::<StateTransitionRequest>()
            .add_systems(
                StateTransition,
                track_state_transitions.run_if(state_changed::<AppState>),
            )
            .add_systems(
                PreUpdate,
                handle_state_transition_requests.run_if(on_message::<StateTransitionRequest>),
            );
    }
}

fn handle_state_transition_requests(
    mut requests: MessageReader<StateTransitionRequest>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for request in requests.read() {
        next_state.set(request.target_state());
    }
}

fn track_state_transitions(
    current: Res<State<AppState>>,
    mut previous: ResMut<PreviousState>,
) {
    let old_state = previous.state;
    previous.state = Some(**current);
    previous.just_entered = true;

    if let Some(from) = old_state {
        info!("State transition: {:?} -> {:?}", from, **current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_state_default() {
        let prev = PreviousState::default();
        assert!(prev.state.is_none());
        assert!(!prev.just_entered);
    }

    #[test]
    fn came_from_returns_true_when_just_entered_from_state() {
        let prev = PreviousState {
            state: Some(AppState::Dungeon),
            just_entered: true,
        };
        assert!(prev.came_from(AppState::Dungeon));
    }

    #[test]
    fn came_from_returns_false_when_not_just_entered() {
        let prev = PreviousState {
            state: Some(AppState::Dungeon),
            just_entered: false,
        };
        assert!(!prev.came_from(AppState::Dungeon));
    }

    #[test]
    fn came_from_returns_false_when_different_state() {
        let prev = PreviousState {
            state: Some(AppState::Menu),
            just_entered: true,
        };
        assert!(!prev.came_from(AppState::Dungeon));
    }

    #[test]
    fn came_from_returns_false_when_no_previous_state() {
        let prev = PreviousState {
            state: None,
            just_entered: true,
        };
        assert!(!prev.came_from(AppState::Dungeon));
    }

    #[test]
    fn acknowledge_entry_clears_just_entered() {
        let mut prev = PreviousState {
            state: Some(AppState::Dungeon),
            just_entered: true,
        };
        prev.acknowledge_entry();
        assert!(!prev.just_entered);
        assert_eq!(prev.state, Some(AppState::Dungeon));
    }

    #[test]
    fn app_state_default_is_menu() {
        let state = AppState::default();
        assert_eq!(state, AppState::Menu);
    }
}

