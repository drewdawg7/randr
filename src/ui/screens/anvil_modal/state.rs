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

/// Trigger resource to close modal and start crafting.
#[derive(Resource)]
pub struct CloseAnvilForCrafting;

/// Implements RegisteredModal for the anvil modal.
pub struct AnvilModal;

impl RegisteredModal for AnvilModal {
    type Root = AnvilModalRoot;
    const MODAL_TYPE: ModalType = ModalType::AnvilModal;

    fn spawn(world: &mut World) {
        world.run_system_cached(do_spawn_anvil_modal).ok();
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<ActiveAnvilEntity>();
    }
}

/// System that spawns the anvil modal UI.
fn do_spawn_anvil_modal(
    commands: Commands,
    game_sprites: Res<crate::assets::GameSprites>,
    game_fonts: Res<crate::assets::GameFonts>,
    inventory: Res<crate::inventory::Inventory>,
) {
    super::render::spawn_anvil_modal_impl(
        commands,
        &game_sprites,
        &game_fonts,
        &inventory,
    );
}
