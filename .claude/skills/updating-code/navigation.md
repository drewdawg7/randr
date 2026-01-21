# Navigation System

Declarative navigation graph for managing state transitions and modal toggles.

**Module:** `src/navigation/`

## Overview

The navigation system centralizes all navigation logic (state transitions, modal opens) into a single builder-based configuration. Instead of scattering navigation handlers across input files, all transitions are declared in one place.

## Core Types

### NavigationTarget

```rust
pub enum NavigationTarget {
    State(AppState),    // Navigate to an app state
    Modal(ModalType),   // Open/toggle a modal
}
```

Both `AppState` and `ModalType` implement `Into<NavigationTarget>` for ergonomic builder API.

### NavigationTable

```rust
#[derive(Resource)]
pub struct NavigationTable {
    state_transitions: HashMap<(AppState, GameAction), NavigationTarget>,
    global_transitions: HashMap<GameAction, NavigationTarget>,
}
```

The table stores:
- **State-specific transitions**: `(current_state, action) -> target`
- **Global transitions**: `action -> target` (applies in any state)

Lookup priority: state-specific first, then global.

### NavigationPlugin

The plugin provides a fluent builder API for configuring transitions:

```rust
NavigationPlugin::new()
    .state(AppState::Town)
        .on(GameAction::OpenInventory, ModalType::Inventory)
        .on(GameAction::OpenProfile, ModalType::Profile)
        .on(GameAction::OpenCompendium, ModalType::MonsterCompendium)
    .state(AppState::Dungeon)
        .on(GameAction::OpenInventory, ModalType::Inventory)
        .on(GameAction::OpenProfile, ModalType::Profile)
        .on(GameAction::OpenCompendium, ModalType::MonsterCompendium)
    .global()
        .on(GameAction::OpenKeybinds, AppState::Keybinds)
    .build()
```

## Files

| File | Purpose |
|------|---------|
| `src/navigation/mod.rs` | Module exports |
| `src/navigation/plugin.rs` | `NavigationPlugin` with builder API |
| `src/navigation/table.rs` | `NavigationTable` resource |
| `src/navigation/systems.rs` | Central `handle_navigation` system |

## Central Navigation System

The `handle_navigation` system in `src/navigation/systems.rs` reads all `GameAction` events and looks up transitions in the `NavigationTable`. For state transitions, it sets the `NextState`. For modal toggles, it uses `ModalCommands`:

```rust
pub fn handle_navigation(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    nav_table: Res<NavigationTable>,
) {
    for action in action_reader.read() {
        let Some(target) = nav_table.lookup(**current_state, *action) else {
            continue;
        };

        match target {
            NavigationTarget::State(state) => {
                if **current_state != state {
                    next_state.set(state);
                }
            }
            NavigationTarget::Modal(modal_type) => {
                handle_modal_toggle(&mut commands, modal_type);
            }
        }
    }
}

fn handle_modal_toggle(commands: &mut Commands, modal_type: ModalType) {
    match modal_type {
        ModalType::Inventory => commands.toggle_modal::<InventoryModal>(),
        ModalType::Profile => commands.toggle_modal::<ProfileModal>(),
        ModalType::MonsterCompendium => commands.toggle_modal::<MonsterCompendiumModal>(),
        ModalType::Keybinds | ModalType::FightModal => {
            // Handled differently
        }
    }
}
```

## Adding New Navigation

### State Transitions

To add a new global state transition:

```rust
NavigationPlugin::new()
    .global()
        .on(GameAction::OpenSettings, AppState::Settings)
    .build()
```

To add a state-specific transition:

```rust
NavigationPlugin::new()
    .state(AppState::Combat)
        .on(GameAction::Retreat, AppState::Dungeon)
    .build()
```

### Modal Navigation

1. Add the modal type to `ModalType` enum in `src/ui/screens/modal.rs`
2. Implement `RegisteredModal` for the modal (see [modal-registry.md](modal-registry.md))
3. Add handling in `handle_modal_toggle` in `src/navigation/systems.rs`:
   ```rust
   ModalType::NewModal => commands.toggle_modal::<NewModal>(),
   ```
4. Configure in the builder:
   ```rust
   NavigationPlugin::new()
       .state(AppState::Town)
           .on(GameAction::OpenNewModal, ModalType::NewModal)
       .build()
   ```

## Modal Close Handlers

Modals maintain their own close handlers (Escape key) using `ModalCommands`:

```rust
pub fn handle_modal_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
) {
    if active_modal.modal != Some(ModalType::MyModal) {
        return;
    }

    for action in action_reader.read() {
        if *action == GameAction::CloseModal {
            commands.close_modal::<MyModal>();
        }
    }
}
```

See [modal-registry.md](modal-registry.md) for full `ModalCommands` documentation.

## Configuration Location

The navigation configuration lives in `src/plugins/game.rs` within the `GamePlugin::build` method, registered alongside other core plugins.

## Hash Requirements

`GameAction` and `NavigationDirection` must derive `Hash` for use as HashMap keys:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub enum GameAction { ... }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub enum NavigationDirection { ... }
```

## Benefits

- All navigation logic in one declarative configuration
- Easy to see all possible transitions at a glance
- Simplified modal toggling via `ModalCommands`
- Removes duplicate toggle boilerplate from modal input handlers
- Enables future features like navigation history/back functionality
