# Modal System

Modals are full-screen UI overlays that block game interaction until closed.

## Core Infrastructure

**File:** `src/ui/screens/modal.rs`

### Resources
- `ActiveModal` - Tracks which modal is currently open (only one at a time)
- `ModalType` - Enum of all modal types

### Helpers
- `spawn_modal_overlay(commands)` - Creates the semi-transparent background overlay
- `create_modal_container()` - Standard modal container node
- `create_modal_title(title)` - Title text bundle
- `create_modal_section(text, color)` - Section text bundle
- `create_modal_instruction(text)` - Instruction text bundle

## Modal Registry System

**File:** `src/ui/modal_registry.rs`

The modal registry provides type-safe commands for toggling modals:

```rust
use crate::ui::ModalCommands;
use crate::ui::screens::inventory_modal::InventoryModal;

// Toggle a modal (open if closed, close if open)
commands.toggle_modal::<InventoryModal>();

// Close a specific modal
commands.close_modal::<InventoryModal>();
```

See [modal-registry.md](modal-registry.md) for full documentation.

## Modal Module Structure

Modals should be organized as modules with separate files:

```
src/ui/screens/my_modal/
├── mod.rs        # Module declarations and re-exports only
├── plugin.rs     # Plugin struct and impl
├── constants.rs  # UI dimension constants
├── state.rs      # Components, resources, RegisteredModal impl
├── input.rs      # Input handling systems
└── render.rs     # Spawning and display systems
```

### mod.rs

Only module declarations and re-exports:

```rust
mod constants;
mod input;
mod plugin;
mod render;
mod state;

pub use plugin::MyModalPlugin;
pub use state::MyModal;  // Export RegisteredModal type
```

### state.rs

Components, resources, and `RegisteredModal` implementation:

```rust
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

/// Component marker for the modal root UI
#[derive(Component)]
pub struct MyModalRoot;

/// Selection state resource
#[derive(Resource, Default)]
pub struct MyModalSelection {
    pub selected: usize,
}

/// Marker resource to trigger spawn
#[derive(Resource)]
pub struct SpawnMyModal;

/// Type-safe handle for the modal
pub struct MyModal;

impl RegisteredModal for MyModal {
    type Root = MyModalRoot;
    const MODAL_TYPE: ModalType = ModalType::MyModal;

    fn spawn(world: &mut World) {
        world.resource_mut::<MyModalSelection>().reset();
        world.insert_resource(SpawnMyModal);
    }

    // Optional: cleanup when modal closes
    fn cleanup(world: &mut World) {
        world.remove_resource::<SomeTemporaryResource>();
    }
}
```

### input.rs

**Modal opening is handled by the navigation system** (see [navigation.md](navigation.md)).

The input.rs file only needs to handle:
1. **Internal navigation** - Up/down, select actions within the modal

Close handling is automatic via `modal_close_system::<MyModal>` registered in the plugin.
See [modal-registry.md](modal-registry.md) for details.

```rust
use crate::ui::screens::modal::{ActiveModal, ModalType};

/// Handle internal modal navigation
pub fn handle_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut selection: ResMut<MyModalSelection>,
) {
    if active_modal.modal != Some(ModalType::MyModal) {
        return;
    }

    for action in action_reader.read() {
        match action {
            GameAction::Navigate(dir) => { /* update selection */ }
            GameAction::Select => { /* perform action */ }
            _ => {}
        }
    }
}
```

### render.rs

Spawning and display systems:

```rust
pub fn spawn_modal(mut commands: Commands, ...) {
    commands.remove_resource::<SpawnMyModal>();

    let overlay = spawn_modal_overlay(&mut commands);
    commands
        .entity(overlay)
        .insert(MyModalRoot)
        .with_children(|parent| {
            // Modal content
        });
}
```

### plugin.rs

Plugin struct (note: no toggle handler, handled by NavigationPlugin):

```rust
use crate::ui::modal_registry::modal_close_system;

pub struct MyModalPlugin;

impl Plugin for MyModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyModalSelection>()
            .add_systems(Update, (
                modal_close_system::<MyModal>,
                handle_navigation,
                update_display,
                trigger_spawn_modal.run_if(resource_exists::<SpawnMyModal>),
            ));
    }
}

fn trigger_spawn_modal(
    mut commands: Commands,
    // ... resources needed for spawning
) {
    commands.remove_resource::<SpawnMyModal>();
    spawn_modal(&mut commands, /* ... */);
}
```

## Examples

| Modal | Files | RegisteredModal Type |
|-------|-------|---------------------|
| Inventory | `src/ui/screens/inventory_modal/` | `InventoryModal` |
| Profile | `src/ui/screens/profile_modal.rs` | `ProfileModal` |
| Monster Compendium | `src/ui/screens/monster_compendium/` | `MonsterCompendiumModal` |

## Input Blocking

**Important:** Non-modal input handlers (town tabs, etc.) must check if a modal is open and return early:

```rust
pub fn handle_tab_input(
    // ... other params
    active_modal: Res<ActiveModal>,
) {
    if active_modal.modal.is_some() {
        return;
    }
    // ... handle input
}
```

Files that implement this pattern:
- `src/ui/screens/town/systems.rs` - `handle_tab_navigation`
- `src/ui/screens/town/tabs/blacksmith/input.rs` - `handle_blacksmith_input`
- `src/ui/screens/town/tabs/store/input.rs` - `handle_store_input`
- `src/ui/screens/town/tabs/alchemist/input.rs` - `handle_alchemist_input`
- `src/ui/screens/town/tabs/field.rs` - `handle_field_input`

## Adding a New Modal

1. Add variant to `ModalType` enum in `src/ui/screens/modal.rs`
2. Create module directory structure (see above)
3. Implement `RegisteredModal` for a `MyModal` struct in `state.rs`
4. Add spawn trigger resource and system in plugin
5. Add match arm in `handle_modal_toggle` in `src/navigation/systems.rs`:
   ```rust
   ModalType::MyModal => commands.toggle_modal::<MyModal>(),
   ```
6. Add `GameAction` variant for opening (if needed) in `src/input/actions.rs`
7. Configure navigation in `NavigationPlugin` in `src/plugins/game.rs`:
   ```rust
   NavigationPlugin::new()
       .state(AppState::Town)
           .on(GameAction::OpenMyModal, ModalType::MyModal)
       .build()
   ```
8. Register modal plugin in `src/plugins/game.rs`
9. Export `MyModal` from `src/ui/screens/mod.rs`

See [navigation.md](navigation.md) for full navigation system documentation.
See [modal-registry.md](modal-registry.md) for full modal registry documentation.
