# Modal Registry System

Type-safe, command-based API for toggling modals.

**File:** `src/ui/modal_registry.rs`

## Quick Reference

```rust
use crate::ui::ModalCommands;
use crate::ui::screens::inventory_modal::InventoryModal;

// Toggle a modal (open if closed, close if open)
commands.toggle_modal::<InventoryModal>();

// Close a specific modal
commands.close_modal::<InventoryModal>();
```

## Core Types

### RegisteredModal Trait

```rust
pub trait RegisteredModal: 'static + Send + Sync {
    /// The root marker component for this modal.
    type Root: Component;

    /// The associated ModalType enum variant.
    const MODAL_TYPE: ModalType;

    /// Spawn the modal UI (called when toggle opens the modal).
    fn spawn(world: &mut World);

    /// Clean up resources when modal closes (optional).
    fn cleanup(_world: &mut World) {}
}
```

### ModalCommands Extension Trait

```rust
pub trait ModalCommands {
    fn toggle_modal<M: RegisteredModal>(&mut self);
    fn close_modal<M: RegisteredModal>(&mut self);
}

impl ModalCommands for Commands<'_, '_> { ... }
```

## Implementing RegisteredModal

### Basic Pattern (Resource Trigger)

The recommended pattern uses a trigger resource that a system watches for:

```rust
// In state.rs
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

/// Marker resource to trigger spawning.
#[derive(Resource)]
pub struct SpawnMyModal;

/// Type-safe handle for the modal.
pub struct MyModal;

impl RegisteredModal for MyModal {
    type Root = MyModalRoot;
    const MODAL_TYPE: ModalType = ModalType::MyModal;

    fn spawn(world: &mut World) {
        // Reset any selection state
        world.resource_mut::<MySelection>().reset();
        // Insert trigger resource
        world.insert_resource(SpawnMyModal);
    }

    // Optional: cleanup when modal closes
    fn cleanup(world: &mut World) {
        world.remove_resource::<SomeTemporaryResource>();
    }
}
```

### Plugin System

```rust
// In plugin.rs
impl Plugin for MyModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MySelection>()
            .add_systems(Update, (
                handle_close,
                handle_navigation,
                trigger_spawn_modal.run_if(resource_exists::<SpawnMyModal>),
            ));
    }
}

fn trigger_spawn_modal(
    mut commands: Commands,
    // ... resources needed for spawning
) {
    commands.remove_resource::<SpawnMyModal>();
    spawn_my_modal(&mut commands, /* ... */);
}
```

## Registered Modals

| Modal | Type | Root Component | Location |
|-------|------|----------------|----------|
| Inventory | `InventoryModal` | `InventoryModalRoot` | `inventory_modal/state.rs` |
| Profile | `ProfileModal` | `ProfileModalRoot` | `profile_modal.rs` |
| Monster Compendium | `MonsterCompendiumModal` | `MonsterCompendiumRoot` | `monster_compendium/state.rs` |

## Close Handlers

Use the generic `modal_close_system::<M>` instead of writing per-modal close handlers:

```rust
use crate::ui::modal_registry::modal_close_system;

// In your plugin:
app.add_systems(Update, (
    modal_close_system::<MyModal>,
    // ... other systems
));
```

The generic system (defined in `src/ui/modal_registry.rs`) listens for `GameAction::CloseModal`
and calls `commands.close_modal::<M>()` when the modal is active.

**Note:** Modals with custom close logic (e.g., fight modal which removes extra resources,
results modal which closes on both Select and CloseModal) should still use custom handlers.

## Navigation System Integration

The navigation system uses `ModalCommands` for toggling:

```rust
// In navigation/systems.rs
fn handle_modal_toggle(commands: &mut Commands, modal_type: ModalType) {
    match modal_type {
        ModalType::Inventory => commands.toggle_modal::<InventoryModal>(),
        ModalType::Profile => commands.toggle_modal::<ProfileModal>(),
        ModalType::MonsterCompendium => commands.toggle_modal::<MonsterCompendiumModal>(),
        // ...
    }
}
```

## Adding a New Modal

1. Add variant to `ModalType` enum in `src/ui/screens/modal.rs`
2. Create `SpawnMyModal` trigger resource
3. Implement `RegisteredModal` for a new `MyModal` struct
4. Add spawn trigger system in plugin
5. Add match arm in `handle_modal_toggle` in `navigation/systems.rs`
6. Export `MyModal` from module

## Benefits

- **Type-safe**: Compile-time checks for modal operations
- **Centralized**: Modal lifecycle managed by the trait
- **Simplified navigation**: `handle_modal_toggle` reduced from 12+ params to 2
- **Consistent cleanup**: `cleanup()` method called automatically on close
- **Familiar pattern**: Follows Bevy's `Commands` extension pattern

## Re-exports

From `src/ui/mod.rs`:
```rust
pub use modal_registry::{ModalCommands, RegisteredModal};
```

## Related Files

- `src/ui/screens/modal.rs` - `ActiveModal`, `ModalType`, base modal helpers
- `src/ui/modal_builder.rs` - `Modal` builder for UI
- `src/navigation/systems.rs` - Central navigation system using `ModalCommands`
