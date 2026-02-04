use bevy::prelude::*;

use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

/// Component marker for the forge modal UI root.
#[derive(Component)]
pub struct ForgeModalRoot;

/// Marker for the crafting slots container (left side).
#[derive(Component)]
pub struct ForgeSlotsGrid;

/// Marker for player inventory grid (right side).
#[derive(Component)]
pub struct ForgePlayerGrid;

/// Which forge slot is currently selected (when crafting panel is focused).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ForgeSlotIndex {
    #[default]
    Coal = 0,
    Ore = 1,
    Product = 2,
}

impl ForgeSlotIndex {
    pub fn next(self) -> Self {
        match self {
            Self::Coal => Self::Ore,
            Self::Ore => Self::Product,
            Self::Product => Self::Product,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Coal => Self::Coal,
            Self::Ore => Self::Coal,
            Self::Product => Self::Ore,
        }
    }

}

/// Tracks forge slot selection state.
#[derive(Resource, Default, Clone)]
pub struct ForgeModalState {
    /// Currently selected forge slot (when FocusPanel::ForgeCraftingSlots is focused)
    pub selected_slot: ForgeSlotIndex,
}

/// Resource to track which forge entity the modal is operating on.
#[derive(Resource)]
pub struct ActiveForgeEntity(pub Entity);

/// Type-safe handle for the forge modal.
pub struct ForgeModal;

impl RegisteredModal for ForgeModal {
    type Root = ForgeModalRoot;
    const MODAL_TYPE: ModalType = ModalType::ForgeModal;

    fn spawn(world: &mut World) {
        world.insert_resource(ForgeModalState::default());
        world.run_system_cached(do_spawn_forge_modal).ok();
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<ForgeModalState>();
        world.remove_resource::<ActiveForgeEntity>();
    }
}

/// System that spawns the forge modal UI.
fn do_spawn_forge_modal(
    commands: Commands,
    game_sprites: Res<crate::assets::GameSprites>,
    game_fonts: Res<crate::assets::GameFonts>,
    inventory: Res<crate::inventory::Inventory>,
    forge_state_query: Query<&crate::crafting_station::ForgeCraftingState>,
    active_forge: Res<ActiveForgeEntity>,
    modal_state: Res<ForgeModalState>,
) {
    super::render::spawn_forge_modal_impl(
        commands,
        &game_sprites,
        &game_fonts,
        &inventory,
        &forge_state_query,
        &active_forge,
        &modal_state,
    );
}
