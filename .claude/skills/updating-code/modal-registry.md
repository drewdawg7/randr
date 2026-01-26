# Modal Registry System

Type-safe, command-based API for toggling modals with observer-based lifecycle events.

**File:** `src/ui/modal_registry.rs`

## Quick Reference

```rust
use crate::ui::ModalCommands;
use crate::ui::screens::inventory_modal::InventoryModal;
use crate::ui::screens::modal::{OpenModal, CloseModal, ModalType};

// Toggle a modal (open if closed, close if open)
commands.toggle_modal::<InventoryModal>();

// Close a specific modal
commands.close_modal::<InventoryModal>();

// Direct event-based triggering
commands.trigger(OpenModal(ModalType::Inventory));
commands.trigger(CloseModal(ModalType::Inventory));
```

## Core Types

### OpenModal / CloseModal Events

Events for modal lifecycle. Defined in `src/ui/screens/modal.rs`:

```rust
/// Event to request opening a modal.
#[derive(Event, Debug, Clone, Copy)]
pub struct OpenModal(pub ModalType);

/// Event to request closing a modal.
#[derive(Event, Debug, Clone, Copy)]
pub struct CloseModal(pub ModalType);
```

### RegisteredModal Trait

```rust
pub trait RegisteredModal: 'static + Send + Sync {
    /// The root marker component for this modal.
    type Root: Component;

    /// The associated ModalType enum variant.
    const MODAL_TYPE: ModalType;

    /// Spawn the modal UI (called via OpenModal observer).
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

### RegisterModalExt Extension Trait

```rust
pub trait RegisterModalExt {
    fn register_modal<M: RegisteredModal>(&mut self) -> &mut Self;
}

impl RegisterModalExt for App { ... }
```

## Implementing RegisteredModal

### Observer-Based Pattern (Preferred)

The recommended pattern uses `run_system_cached` to spawn UI:

```rust
// In state.rs
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

/// Type-safe handle for the modal.
pub struct MyModal;

impl RegisteredModal for MyModal {
    type Root = MyModalRoot;
    const MODAL_TYPE: ModalType = ModalType::MyModal;

    fn spawn(world: &mut World) {
        world.run_system_cached(do_spawn_my_modal).ok();
    }

    // Optional: cleanup when modal closes
    fn cleanup(world: &mut World) {
        world.remove_resource::<SomeTemporaryResource>();
    }
}

/// System that spawns the modal UI.
fn do_spawn_my_modal(
    commands: Commands,
    inventory: Res<Inventory>,
    // ... other resources
) {
    spawn_my_modal_impl(commands, &inventory);
}
```

### Plugin Registration

```rust
// In plugin.rs
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};

impl Plugin for MyModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<MyModal>()  // Registers OpenModal/CloseModal observers
            .add_systems(Update, (
                modal_close_system::<MyModal>,
                handle_navigation.run_if(in_my_modal),
            ));
    }
}
```

### Render Function Pattern

The `spawn` function should NOT set `ActiveModal` - that's handled by the observer:

```rust
// In render.rs
pub fn spawn_my_modal_impl(
    mut commands: Commands,
    inventory: &Inventory,
) {
    // Initialize focus
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::MyPanel),
    });

    // Spawn UI entities
    let overlay = spawn_modal_overlay(&mut commands);
    commands.entity(overlay)
        .insert(MyModalRoot)
        .with_children(|parent| {
            // ... UI hierarchy
        });
}
```

## Registered Modals

| Modal | Type | Root Component | Location |
|-------|------|----------------|----------|
| Inventory | `InventoryModal` | `InventoryModalRoot` | `inventory_modal/state.rs` |
| Profile | `ProfileModal` | `ProfileModalRoot` | `profile_modal.rs` |
| Monster Compendium | `MonsterCompendiumModal` | `MonsterCompendiumRoot` | `monster_compendium/state.rs` |
| Merchant | `MerchantModal` | `MerchantModalRoot` | `merchant_modal/state.rs` |
| Forge | `ForgeModal` | `ForgeModalRoot` | `forge_modal/state.rs` |
| Anvil | `AnvilModal` | `AnvilModalRoot` | `anvil_modal/state.rs` |

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

## Opening Modals from Other Systems

Use the event-based approach for opening modals from other systems (e.g., dungeon interactions):

```rust
use crate::ui::screens::modal::{OpenModal, ModalType};

fn handle_interaction(mut commands: Commands) {
    // Set up any required resources first
    commands.insert_resource(ActiveForgeEntity(entity_id));

    // Trigger the modal open
    commands.trigger(OpenModal(ModalType::ForgeModal));
}
```

## Reactive UI Updates (Changed Detection)

Instead of resource triggers for UI refresh, use Bevy's native change detection:

```rust
// Before (resource trigger):
commands.insert_resource(ForgeSlotRefresh);

// After (Changed detection in system):
pub fn refresh_forge_slots(
    forge_state_query: Query<&ForgeCraftingState, Changed<ForgeCraftingState>>,
    // ...
) {
    // Query only returns results when ForgeCraftingState changed
    let Ok(forge_state) = forge_state_query.get(active_forge.0) else {
        return;
    };
    // ... update UI
}
```

## Adding a New Modal

1. Add variant to `ModalType` enum in `src/ui/screens/modal.rs`
2. Create root component marker (e.g., `MyModalRoot`)
3. Implement `RegisteredModal` for a new `MyModal` struct
4. Create `spawn_my_modal_impl` render function (no ActiveModal handling)
5. Add `app.register_modal::<MyModal>()` in plugin
6. Add match arm in `handle_modal_toggle` in `navigation/systems.rs`
7. Export `MyModal` from module

## Benefits

- **Type-safe**: Compile-time checks for modal operations
- **Event-driven**: Observers handle lifecycle without polling systems
- **No resource triggers**: Replaced `SpawnMyModal` resources with `OpenModal` events
- **Reactive updates**: Use `Changed<T>` instead of refresh triggers
- **Simplified navigation**: `handle_modal_toggle` reduced from 12+ params to 2
- **Consistent cleanup**: `cleanup()` method called automatically on close
- **Familiar pattern**: Follows Bevy's `Commands` extension pattern

## Re-exports

From `src/ui/mod.rs`:
```rust
pub use modal_registry::{ModalCommands, RegisteredModal, RegisterModalExt};
```

From `src/ui/screens/mod.rs`:
```rust
pub use modal::{ActiveModal, CloseModal, ModalPlugin, ModalType, OpenModal};
```

## Related Files

- `src/ui/screens/modal.rs` - `ActiveModal`, `ModalType`, `OpenModal`, `CloseModal`, base modal helpers
- `src/ui/modal_builder.rs` - `Modal` builder for UI
- `src/navigation/systems.rs` - Central navigation system using `ModalCommands`
