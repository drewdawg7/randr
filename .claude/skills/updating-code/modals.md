# Modal System

Modals are full-screen UI overlays that block game interaction until closed.

## Core Infrastructure

**File:** `src/screens/modal.rs`

### Resources
- `ActiveModal` - Tracks which modal is currently open (only one at a time)
- `ModalType` - Enum of all modal types

### Helpers
- `spawn_modal_overlay(commands)` - Creates the semi-transparent background overlay
- `create_modal_container()` - Standard modal container node
- `create_modal_title(title)` - Title text bundle
- `create_modal_section(text, color)` - Section text bundle
- `create_modal_instruction(text)` - Instruction text bundle

## Modal Module Structure

Modals should be organized as modules with separate files:

```
src/screens/my_modal/
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

Input handling systems:

```rust
/// Toggle modal open/close
pub fn handle_toggle(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    existing: Query<Entity, With<MyModalRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::OpenMyModal {
            if let Ok(entity) = existing.get_single() {
                // Close if open
                commands.entity(entity).despawn_recursive();
                active_modal.modal = None;
            } else if active_modal.modal.is_none() {
                // Open if no modal active
                commands.insert_resource(SpawnMyModal);
                active_modal.modal = Some(ModalType::MyModal);
            }
        }
    }
}

/// Close with Escape
pub fn handle_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    query: Query<Entity, With<MyModalRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal
            && active_modal.modal == Some(ModalType::MyModal)
        {
            if let Ok(entity) = query.get_single() {
                commands.entity(entity).despawn_recursive();
                active_modal.modal = None;
            }
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

Plugin struct:

```rust
pub struct MyModalPlugin;

impl Plugin for MyModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyModalSelection>()
            .add_systems(Update, (
                handle_toggle,
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
| Monster Compendium | `src/screens/monster_compendium/` | Book-style with animated mob sprite |
| Inventory | `src/screens/inventory_modal/` | Two-panel with item details |

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
- `src/screens/town/systems.rs` - `handle_tab_navigation`
- `src/screens/town/tabs/blacksmith/input.rs` - `handle_blacksmith_input`
- `src/screens/town/tabs/store/input.rs` - `handle_store_input`
- `src/screens/town/tabs/alchemist/input.rs` - `handle_alchemist_input`
- `src/screens/town/tabs/field.rs` - `handle_field_input`

## Adding a New Modal

1. Add variant to `ModalType` enum in `src/screens/modal.rs`
2. Create module directory structure
3. Add `GameAction` variant for opening (if needed)
4. Register plugin in `src/plugins/game.rs`
5. Export from `src/screens/mod.rs`
