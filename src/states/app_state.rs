use bevy::prelude::*;

/// Application states matching the original screen system.
/// Mirrors `src/ui/screen/common.rs::Id`.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Town,
    Fight,
    Profile,
    Mine,
    Dungeon,
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
            .add_systems(
                StateTransition,
                track_state_transitions.run_if(state_changed::<AppState>),
            );
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

/// Metadata about a screen for validation and documentation.
/// Ported from `src/ui/screen/lifecycle.rs::ScreenMetadata`.
#[derive(Debug, Clone)]
pub struct ScreenMetadata {
    pub state: AppState,
    pub name: &'static str,
    pub requires_combat: bool,
    pub requires_dungeon: bool,
    pub parent: Option<AppState>,
}

impl ScreenMetadata {
    pub fn for_state(state: AppState) -> Self {
        match state {
            AppState::Menu => Self {
                state,
                name: "Main Menu",
                requires_combat: false,
                requires_dungeon: false,
                parent: None,
            },
            AppState::Town => Self {
                state,
                name: "Town",
                requires_combat: false,
                requires_dungeon: false,
                parent: Some(AppState::Menu),
            },
            AppState::Fight => Self {
                state,
                name: "Combat",
                requires_combat: true,
                requires_dungeon: false,
                parent: None, // Dynamic based on CombatSource
            },
            AppState::Profile => Self {
                state,
                name: "Player Profile",
                requires_combat: false,
                requires_dungeon: false,
                parent: Some(AppState::Town),
            },
            AppState::Mine => Self {
                state,
                name: "Mine",
                requires_combat: false,
                requires_dungeon: false,
                parent: Some(AppState::Town),
            },
            AppState::Dungeon => Self {
                state,
                name: "Dungeon",
                requires_combat: false,
                requires_dungeon: true,
                parent: Some(AppState::Town),
            },
            AppState::Keybinds => Self {
                state,
                name: "Keybinds",
                requires_combat: false,
                requires_dungeon: false,
                parent: None, // Can be opened from any state
            },
        }
    }

    /// Get valid destination states from this state.
    pub fn valid_destinations(&self) -> Vec<AppState> {
        match self.state {
            AppState::Menu => vec![AppState::Town, AppState::Keybinds],
            AppState::Town => vec![
                AppState::Menu,
                AppState::Fight,
                AppState::Mine,
                AppState::Dungeon,
                AppState::Profile,
                AppState::Keybinds,
            ],
            AppState::Fight => vec![AppState::Town, AppState::Dungeon, AppState::Keybinds],
            AppState::Profile => vec![AppState::Town, AppState::Keybinds],
            AppState::Mine => vec![AppState::Town, AppState::Keybinds],
            AppState::Dungeon => vec![AppState::Town, AppState::Fight, AppState::Keybinds],
            AppState::Keybinds => vec![], // Managed by CloseModal action
        }
    }
}
