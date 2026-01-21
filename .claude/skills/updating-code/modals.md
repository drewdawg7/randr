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
- `toggle_modal(commands, active_modal, query, modal_type)` - Generic toggle logic (see below)
- `close_modal(commands, active_modal, query, modal_type)` - Generic close logic (see below)

## Modal Module Structure

Modals should be organized as modules with separate files:

```
src/ui/screens/my_modal/
├── mod.rs        # Module declarations and re-exports only
├── plugin.rs     # Plugin struct and impl
├── constants.rs  # UI dimension constants
├── state.rs      # Components, resources, display structs
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
```

### constants.rs

Named constants for all UI dimensions:

```rust
// Container dimensions
pub const CONTAINER_WIDTH: f32 = 672.0;
pub const CONTAINER_HEIGHT: f32 = 399.0;

// Typography
pub const TITLE_FONT_SIZE: f32 = 24.0;

// Colors
pub const SELECTED_COLOR: Color = Color::srgb(0.5, 0.3, 0.1);
pub const NORMAL_COLOR: Color = Color::srgb(0.2, 0.15, 0.1);
```

### state.rs

Components and resources:

```rust
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

/// Display data (decoupled from game registries)
pub struct DisplayEntry {
    pub name: String,
    pub id: SomeId,
}
```

### input.rs

**Modal opening is handled by the navigation system** (see [navigation.md](navigation.md)).

The input.rs file only needs to handle:
1. **Close handler** - Escape key to close the modal
2. **Internal navigation** - Up/down, select actions within the modal

```rust
use crate::ui::screens::modal::{close_modal, ActiveModal, ModalType};

/// Close with Escape
pub fn handle_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    query: Query<Entity, With<MyModalRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal {
            if close_modal(&mut commands, &mut active_modal, &query, ModalType::MyModal) {
                // Optional: custom cleanup (remove resources, etc.)
            }
        }
    }
}

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

#### close_modal Helper

`close_modal<T: Component>(commands, active_modal, modal_query, modal_type) -> bool`

Returns `true` if the modal was closed, `false` if it wasn't active.

Use this for handling `GameAction::CloseModal` (Escape key).

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
pub struct MyModalPlugin;

impl Plugin for MyModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyModalSelection>()
            .add_systems(Update, (
                handle_close,
                handle_navigation,
                update_display,
                spawn_modal.run_if(resource_exists::<SpawnMyModal>),
            ));
    }
}
```

## Examples

| Modal | Files | Notes |
|-------|-------|-------|
| Monster Compendium | `src/ui/screens/monster_compendium/` | Book-style with animated mob sprite |
| Inventory | `src/ui/screens/inventory_modal/` | Two-panel with item details |

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
2. Create module directory structure
3. Add `GameAction` variant for opening (if needed) in `src/input/actions.rs`
4. Add handling in `handle_modal_toggle` in `src/navigation/systems.rs`
5. Configure navigation in `NavigationPlugin` in `src/plugins/game.rs`:
   ```rust
   NavigationPlugin::new()
       .state(AppState::Town)
           .on(GameAction::OpenMyModal, ModalType::MyModal)
       .build()
   ```
6. Register modal plugin in `src/plugins/game.rs`
7. Export from `src/ui/screens/mod.rs`

See [navigation.md](navigation.md) for full navigation system documentation.
