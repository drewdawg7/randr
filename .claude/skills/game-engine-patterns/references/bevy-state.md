# Bevy State Management

Overview of Bevy 0.18 state management for controlling game flow and screen transitions.

## Quick Reference

```rust
// Define states
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Menu,
    Playing,
    Paused,
}

// Register state
app.init_state::<AppState>();

// State schedules
app.add_systems(OnEnter(AppState::Playing), setup_game);
app.add_systems(OnExit(AppState::Playing), cleanup_game);

// Run conditions
app.add_systems(Update, game_logic.run_if(in_state(AppState::Playing)));

// Change state
fn start_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Playing);
}
```

## State Types

### Basic States

Standard states changed manually via `NextState<S>`:

```rust
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AppState {
    #[default]
    Menu,
    Dungeon,
    Profile,
}

app.init_state::<AppState>();
```

**Codebase example** from `src/states/app_state.rs`:
```rust
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Dungeon,
    Profile,
    Keybinds,
}
```

### SubStates

Child states that only exist when parent state meets conditions:

```rust
#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(AppState = AppState::InGame)]  // Only exists in InGame
#[states(scoped_entities)]               // Auto-despawn entities
enum GamePhase {
    #[default]
    Playing,
    Paused,
}

app.init_state::<AppState>()
    .add_sub_state::<GamePhase>();
```

### ComputedStates

Automatically derived from other states (cannot be manually changed):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct InGame;

impl ComputedStates for InGame {
    type SourceStates = AppState;

    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            AppState::Dungeon | AppState::Profile => Some(InGame),
            _ => None,
        }
    }
}

app.add_computed_state::<InGame>();
```

## State Transitions

### Changing State

Use `NextState<S>` resource:

| Method | Behavior |
|--------|----------|
| `set(state)` | Always triggers OnEnter/OnExit (even same state) |
| `set_if_neq(state)` | Only triggers if different from current |
| `reset()` | Cancels pending transition |

```rust
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Menu);
    }
}
```

**Codebase example** from `src/states/app_state.rs`:
```rust
fn handle_state_transition_requests(
    mut requests: MessageReader<StateTransitionRequest>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for request in requests.read() {
        next_state.set(request.target_state());
    }
}
```

### Reading Current State

```rust
fn check_state(state: Res<State<AppState>>) {
    println!("Current: {:?}", state.get());
}
```

## State Schedules

### Schedule Timing

The `StateTransition` schedule runs:
- **Before** `PreStartup` on first frame (only once)
- **After** `PreUpdate` each frame
- **Before** `Update`

### OnEnter / OnExit

```rust
app.add_systems(OnEnter(AppState::Menu), spawn_menu);
app.add_systems(OnExit(AppState::Menu), despawn_menu);
```

**Codebase example** from `src/ui/screens/main_menu.rs`:
```rust
app.add_systems(OnEnter(AppState::Menu), (spawn_main_menu, reset_menu_selection).chain())
    .add_systems(OnExit(AppState::Menu), despawn_main_menu);
```

**Codebase example** from `src/ui/screens/dungeon/plugin.rs`:
```rust
.add_systems(OnEnter(AppState::Dungeon), enter_dungeon)
.add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
```

### OnTransition

Runs during specific state-to-state transitions:

```rust
app.add_systems(
    OnTransition {
        exited: AppState::Menu,
        entered: AppState::Dungeon
    },
    play_transition_animation,
);
```

**Transition Order**:
1. `StateTransitionEvent<S>` sent
2. `OnExit(old_state)` runs
3. `OnTransition { exited, entered }` runs
4. `OnEnter(new_state)` runs

## Run Conditions

### in_state

Gate systems to specific state:

```rust
app.add_systems(Update, game_logic.run_if(in_state(AppState::Playing)));
```

**Codebase example** from `src/ui/screens/main_menu.rs`:
```rust
app.add_systems(
    Update,
    (
        handle_menu_navigation,
        handle_menu_selection,
        update_sprite_menu_items,
    )
        .run_if(in_state(AppState::Menu)),
);
```

### state_changed

React to state transitions:

```rust
app.add_systems(Update, log_change.run_if(state_changed::<AppState>));
```

**Codebase example** from `src/states/app_state.rs`:
```rust
app.add_systems(
    StateTransition,
    track_state_transitions.run_if(state_changed::<AppState>),
);
```

### state_exists

Check if state resource exists:

```rust
app.add_systems(Update, substate_logic.run_if(state_exists::<GamePhase>));
```

## State-Scoped Entities

### DespawnOnExit

Automatically despawn entities when exiting state:

```rust
commands.spawn((
    Sprite::default(),
    DespawnOnExit(AppState::Playing),
));
```

### DespawnOnEnter

Despawn when entering a state:

```rust
commands.spawn((
    LoadingScreen,
    DespawnOnEnter(AppState::Playing),
));
```

### Enabling Scoped Entities

**Method 1: Derive attribute** (recommended):
```rust
#[derive(States, ...)]
#[states(scoped_entities)]
enum AppState { ... }
```

**Method 2: App method**:
```rust
app.enable_state_scoped_entities::<AppState>();
```

## This Codebase

### Pattern: Message-Based Transitions

State transitions via typed messages:

```rust
// From src/states/app_state.rs
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
```

### Pattern: Marker Component Cleanup

Use marker components for efficient despawning:

```rust
// From src/ui/screens/main_menu.rs
#[derive(Component)]
struct MainMenuRoot;

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((MainMenuRoot, /* ... */));
}

fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    if let Ok(entity) = query.single() {
        commands.entity(entity).despawn();
    }
}
```

### Pattern: Previous State Tracking

```rust
// From src/states/app_state.rs
#[derive(Resource, Default)]
pub struct PreviousState {
    pub state: Option<AppState>,
    pub just_entered: bool,
}

fn track_state_transitions(
    current: Res<State<AppState>>,
    mut previous: ResMut<PreviousState>,
) {
    previous.state = Some(**current);
    previous.just_entered = true;
}
```

## Common Mistakes

### Forgetting to register state
```rust
// Wrong: state not registered
app.add_systems(OnEnter(AppState::Menu), setup);

// Correct: register first
app.init_state::<AppState>();
app.add_systems(OnEnter(AppState::Menu), setup);
```

### Using set() when set_if_neq() is needed
```rust
// Bevy 0.18: triggers OnEnter/OnExit even for same state
next_state.set(AppState::Menu);

// Use this to skip redundant transitions
next_state.set_if_neq(AppState::Menu);
```

### Missing state condition on Update systems
```rust
// Wrong: runs in all states
app.add_systems(Update, dungeon_logic);

// Correct: state-gated
app.add_systems(Update, dungeon_logic.run_if(in_state(AppState::Dungeon)));
```

### Expensive OnEnter setup
```rust
// Wrong: blocking asset loading in OnEnter
app.add_systems(OnEnter(AppState::Playing), load_all_assets);

// Correct: load in Startup, spawn in OnEnter
app.add_systems(Startup, load_assets);
app.add_systems(OnEnter(AppState::Playing), spawn_level);
```
