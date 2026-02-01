//! Modal Registry System
//!
//! Provides a type-safe, command-based API for toggling modals.
//!
//! # Example
//!
//! ```ignore
//! use crate::ui::modal_registry::{ModalCommands, RegisteredModal};
//!
//! // In a system:
//! commands.toggle_modal::<InventoryModal>();
//! commands.close_modal::<InventoryModal>();
//!
//! // Or via events:
//! commands.trigger(OpenModal(ModalType::Inventory));
//! commands.trigger(CloseModal(ModalType::Inventory));
//! ```

use std::marker::PhantomData;

use bevy::ecs::system::Command;
use bevy::prelude::*;

use crate::input::GameAction;
use crate::ui::screens::modal::{ActiveModal, CloseModal, ModalType, OpenModal};

/// Trait for modal types that can be toggled via commands.
///
/// Implement this for each modal to enable `commands.toggle_modal::<M>()`.
pub trait RegisteredModal: 'static + Send + Sync {
    /// The root marker component for this modal.
    type Root: Component;

    /// The associated ModalType enum variant.
    const MODAL_TYPE: ModalType;

    /// Spawn the modal UI.
    ///
    /// Called when the modal should be opened (triggered via OpenModal event).
    /// Has mutable World access for reading resources and spawning entities.
    fn spawn(world: &mut World);

    /// Clean up resources when the modal is closed.
    ///
    /// Called after the modal entity is despawned.
    /// Override this to remove any modal-specific resources.
    fn cleanup(_world: &mut World) {}
}

/// Extension trait for Commands to toggle modals.
pub trait ModalCommands {
    /// Toggle a modal open/closed.
    ///
    /// - If the modal is open, closes it
    /// - If no modal is open, opens this modal
    /// - If a different modal is open, does nothing
    fn toggle_modal<M: RegisteredModal>(&mut self);

    /// Close a modal if it's currently open.
    ///
    /// Does nothing if this modal isn't active.
    fn close_modal<M: RegisteredModal>(&mut self);
}

impl ModalCommands for Commands<'_, '_> {
    fn toggle_modal<M: RegisteredModal>(&mut self) {
        self.queue(ToggleModalCommand::<M>(PhantomData));
    }

    fn close_modal<M: RegisteredModal>(&mut self) {
        self.queue(CloseModalCommand::<M>(PhantomData));
    }
}

/// Command that toggles a modal open/closed.
struct ToggleModalCommand<M: RegisteredModal>(PhantomData<M>);

impl<M: RegisteredModal> Command for ToggleModalCommand<M> {
    fn apply(self, world: &mut World) {
        // Check if this modal is currently open
        let mut query = world.query_filtered::<Entity, With<M::Root>>();
        let modal_entity = query.iter(world).next();

        if modal_entity.is_some() {
            // Modal is open - trigger close event
            world.trigger(CloseModal(M::MODAL_TYPE));
        } else {
            // Check if any modal is open
            let active_modal = world.resource::<ActiveModal>();
            if active_modal.modal.is_none() {
                // No modal open - trigger open event
                world.trigger(OpenModal(M::MODAL_TYPE));
            }
        }
    }
}

/// Generic close handler system for any `RegisteredModal`.
///
/// Listens for `GameAction::CloseModal` and closes the modal if it's active.
/// Register as `modal_close_system::<MyModal>` in your plugin instead of
/// writing a per-modal close handler.
pub fn modal_close_system<M: RegisteredModal>(
    mut commands: Commands,
    mut action_reader: MessageReader<GameAction>,
    active_modal: Res<ActiveModal>,
) {
    if active_modal.modal != Some(M::MODAL_TYPE) {
        return;
    }

    for action in action_reader.read() {
        if *action == GameAction::CloseModal {
            commands.close_modal::<M>();
        }
    }
}

/// Command that closes a specific modal.
struct CloseModalCommand<M: RegisteredModal>(PhantomData<M>);

impl<M: RegisteredModal> Command for CloseModalCommand<M> {
    fn apply(self, world: &mut World) {
        // Only close if this specific modal is active
        let is_active = world.resource::<ActiveModal>().modal == Some(M::MODAL_TYPE);
        if !is_active {
            return;
        }

        // Trigger close event
        world.trigger(CloseModal(M::MODAL_TYPE));
    }
}

// ============================================================================
// Modal Lifecycle Observers
// ============================================================================

/// Observer that handles OpenModal events.
///
/// Register per-modal observers via `register_modal_observer::<M>`.
pub fn on_open_modal<M: RegisteredModal>(
    trigger: On<OpenModal>,
    mut commands: Commands,
) {
    if trigger.event().0 != M::MODAL_TYPE {
        return;
    }
    commands.queue(SpawnModalCommand::<M>(PhantomData));
}

/// Observer that handles CloseModal events.
///
/// Register per-modal observers via `register_modal_observer::<M>`.
pub fn on_close_modal<M: RegisteredModal>(
    trigger: On<CloseModal>,
    mut commands: Commands,
) {
    if trigger.event().0 != M::MODAL_TYPE {
        return;
    }
    commands.queue(DespawnModalCommand::<M>(PhantomData));
}

/// Command that spawns a modal.
struct SpawnModalCommand<M: RegisteredModal>(PhantomData<M>);

impl<M: RegisteredModal> Command for SpawnModalCommand<M> {
    fn apply(self, world: &mut World) {
        // Set active modal first
        world.resource_mut::<ActiveModal>().modal = Some(M::MODAL_TYPE);
        // Then spawn
        M::spawn(world);
    }
}

/// Command that despawns a modal and runs cleanup.
struct DespawnModalCommand<M: RegisteredModal>(PhantomData<M>);

impl<M: RegisteredModal> Command for DespawnModalCommand<M> {
    fn apply(self, world: &mut World) {
        // Find and despawn the modal entity if it exists
        let mut query = world.query_filtered::<Entity, With<M::Root>>();
        if let Some(entity) = query.iter(world).next() {
            world.entity_mut(entity).despawn();
        }
        // Always clear ActiveModal and run cleanup, even if entity wasn't found.
        // This prevents ActiveModal from getting stuck if spawn failed or entity was already despawned.
        world.resource_mut::<ActiveModal>().modal = None;
        M::cleanup(world);
    }
}

/// Extension trait for App to register modal observers.
pub trait RegisterModalExt {
    /// Register observers for a modal type.
    ///
    /// This sets up the OpenModal and CloseModal event handlers for the modal.
    fn register_modal<M: RegisteredModal>(&mut self) -> &mut Self;
}

impl RegisterModalExt for App {
    fn register_modal<M: RegisteredModal>(&mut self) -> &mut Self {
        self.add_observer(on_open_modal::<M>)
            .add_observer(on_close_modal::<M>)
    }
}
