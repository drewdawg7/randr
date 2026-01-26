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

    #[allow(dead_code)]
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Coal,
            1 => Self::Ore,
            _ => Self::Product,
        }
    }

    #[allow(dead_code)]
    pub fn as_index(self) -> usize {
        self as usize
    }
}

/// Tracks forge slot selection state.
#[derive(Resource, Default)]
pub struct ForgeModalState {
    /// Currently selected forge slot (when FocusPanel::ForgeCraftingSlots is focused)
    pub selected_slot: ForgeSlotIndex,
}

/// Resource to track which forge entity the modal is operating on.
#[derive(Resource)]
pub struct ActiveForgeEntity(pub Entity);

/// Marker resource to trigger spawning the forge modal.
#[derive(Resource)]
pub struct SpawnForgeModal;

/// Marker resource to force slot display refresh after transfers.
#[derive(Resource)]
pub struct ForgeSlotRefresh;

/// Type-safe handle for the forge modal.
pub struct ForgeModal;

impl RegisteredModal for ForgeModal {
    type Root = ForgeModalRoot;
    const MODAL_TYPE: ModalType = ModalType::ForgeModal;

    fn spawn(world: &mut World) {
        world.insert_resource(ForgeModalState::default());
        world.insert_resource(SpawnForgeModal);
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<ForgeModalState>();
        world.remove_resource::<ActiveForgeEntity>();
        world.remove_resource::<ForgeSlotRefresh>();
    }
}
