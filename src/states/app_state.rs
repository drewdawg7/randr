use bevy::prelude::*;
use bevy::state::state::StateTransitionEvent;

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
            AppState::Loading => panic!("Cannot create StateTransitionRequest from Loading state"),
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
    Loading,
    Menu,
    Dungeon,
    Profile,
    Keybinds,
}

#[derive(Resource, Default)]
pub struct PreviousState {
    pub state: Option<AppState>,
}

pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<PreviousState>()
            .add_message::<StateTransitionRequest>()
            .add_systems(StateTransition, track_state_transitions)
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
    mut events: MessageReader<StateTransitionEvent<AppState>>,
    mut previous: ResMut<PreviousState>,
) {
    for transition in events.read() {
        if let (Some(from), Some(to)) = (transition.exited, transition.entered) {
            if from != to {
                previous.state = Some(from);
                info!("State transition: {:?} -> {:?}", from, to);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn previous_state_default() {
        let prev = PreviousState::default();
        assert!(prev.state.is_none());
    }

    #[test]
    fn app_state_default_is_loading() {
        let state = AppState::default();
        assert_eq!(state, AppState::Loading);
    }
}

