use bevy::prelude::*;

// ============================================================================
// State Transition Events
// ============================================================================

/// Event requesting transition to Mine state.
#[derive(Event, Debug, Clone)]
pub struct RequestMineEvent;

// ============================================================================
// Application State
// ============================================================================

/// Application states matching the original screen system.
/// Mirrors `src/ui/screen/common.rs::Id`.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Town,
    Dungeon,
    Profile,
    Mine,
    Keybinds,
}

/// Resource tracking the previous state for "came_from" logic.
/// Mirrors `ScreenLifecycle.previous_screen` from the original system.
#[derive(Resource, Default)]
pub struct PreviousState {
    /// The state we transitioned from.
    pub state: Option<AppState>,
    /// Whether we just entered the current state this frame.
    pub just_entered: bool,
}

impl PreviousState {
    /// Check if we just came from a specific state.
    pub fn came_from(&self, state: AppState) -> bool {
        self.just_entered && self.state == Some(state)
    }

    /// Acknowledge entry to clear the just_entered flag.
    pub fn acknowledge_entry(&mut self) {
        self.just_entered = false;
    }
}

/// Plugin that manages state transitions and tracks previous state.
pub struct StateTransitionPlugin;

impl Plugin for StateTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<PreviousState>()
            .add_event::<RequestMineEvent>()
            .add_systems(
                StateTransition,
                track_state_transitions.run_if(state_changed::<AppState>),
            )
            .add_systems(Update, handle_state_transition_requests);
    }
}

/// System that handles state transition request events.
fn handle_state_transition_requests(
    mut mine_events: EventReader<RequestMineEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Process mine requests
    if mine_events.read().next().is_some() {
        next_state.set(AppState::Mine);
    }
}

/// System that updates PreviousState when AppState changes.
fn track_state_transitions(
    current: Res<State<AppState>>,
    mut previous: ResMut<PreviousState>,
) {
    // Store what will become the previous state
    let old_state = previous.state;
    previous.state = Some(**current);
    previous.just_entered = true;

    // Log transition for debugging
    if let Some(from) = old_state {
        info!("State transition: {:?} -> {:?}", from, **current);
    }
}

