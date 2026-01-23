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
//! commands.close_modal::<ProfileModal>();
//! ```

use std::marker::PhantomData;

use bevy::ecs::world::Command;
use bevy::prelude::*;

use crate::input::GameAction;
use crate::ui::screens::modal::{ActiveModal, ModalType};

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
    /// Called when toggle opens the modal (no other modal was active).
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

        if let Some(entity) = modal_entity {
            // Modal is open - close it
            world.entity_mut(entity).despawn_recursive();
            world.resource_mut::<ActiveModal>().modal = None;
            M::cleanup(world);
        } else {
            // Check if any modal is open
            let active_modal = world.resource::<ActiveModal>();
            if active_modal.modal.is_none() {
                // No modal open - spawn this one
                world.resource_mut::<ActiveModal>().modal = Some(M::MODAL_TYPE);
                M::spawn(world);
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
    mut action_reader: EventReader<GameAction>,
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

        // Find and despawn the modal entity
        let mut query = world.query_filtered::<Entity, With<M::Root>>();
        if let Some(entity) = query.iter(world).next() {
            world.entity_mut(entity).despawn_recursive();
            world.resource_mut::<ActiveModal>().modal = None;
            M::cleanup(world);
        }
    }
}
