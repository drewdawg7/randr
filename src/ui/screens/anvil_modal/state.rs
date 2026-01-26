//! State types for the anvil modal.

use bevy::prelude::*;

use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

/// Marker component for the anvil modal root entity.
#[derive(Component)]
pub struct AnvilModalRoot;

/// Marker for the recipe grid (left side).
#[derive(Component)]
pub struct AnvilRecipeGrid;

/// Marker for the player inventory grid (right side).
#[derive(Component)]
pub struct AnvilPlayerGrid;

/// Tracks which anvil entity the modal is open for.
#[derive(Resource)]
pub struct ActiveAnvilEntity(pub Entity);

// AnvilModalState removed - focus is now tracked via FocusState resource

/// Trigger resource to spawn the anvil modal.
#[derive(Resource)]
pub struct SpawnAnvilModal;

/// Trigger resource to refresh the recipe grid display.
#[derive(Resource)]
pub struct AnvilRecipeRefresh;

/// Trigger resource to close modal and start crafting.
#[derive(Resource)]
pub struct CloseAnvilForCrafting;

/// Implements RegisteredModal for the anvil modal.
pub struct AnvilModal;

impl RegisteredModal for AnvilModal {
    type Root = AnvilModalRoot;
    const MODAL_TYPE: ModalType = ModalType::AnvilModal;

    fn spawn(world: &mut World) {
        world.insert_resource(SpawnAnvilModal);
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<ActiveAnvilEntity>();
        world.remove_resource::<AnvilRecipeRefresh>();
    }
}
