use bevy::prelude::*;

use crate::ui::focus::FocusState;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

#[derive(Component)]
pub struct ForgeModalRoot;

#[derive(Component)]
pub struct ForgeSlotsGrid;

#[derive(Component)]
pub struct ForgePlayerGrid;

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

#[derive(Resource, Default, Clone)]
pub struct ForgeModalState {
    pub selected_slot: ForgeSlotIndex,
}

#[derive(Resource)]
pub struct ActiveForgeEntity(pub Entity);

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
        world.remove_resource::<FocusState>();
    }
}

fn do_spawn_forge_modal(
    commands: Commands,
    game_sprites: Res<crate::assets::GameSprites>,
    game_fonts: Res<crate::assets::GameFonts>,
    inventory: Res<crate::inventory::Inventory>,
    forge_state_query: Query<&crate::crafting_station::ForgeCraftingState>,
    active_forge: Res<ActiveForgeEntity>,
    modal_state: Res<ForgeModalState>,
) {
    super::spawning::spawn_forge_modal_impl(
        commands,
        &game_sprites,
        &game_fonts,
        &inventory,
        &forge_state_query,
        &active_forge,
        &modal_state,
    );
}
