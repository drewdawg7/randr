use bevy::prelude::*;

use crate::inventory::Inventory;
use crate::player::PlayerMarker;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;
use crate::ui::widgets::{DetailPaneContext, ItemGridSelection};
use crate::ui::InfoPanelSource;

/// Marker component for the anvil modal root entity.
#[derive(Component)]
pub struct AnvilModalRoot;

/// Marker for the recipe grid (left side).
#[derive(Component)]
pub struct AnvilRecipeGrid;

/// Marker for the player inventory grid (right side).
#[derive(Component)]
pub struct AnvilPlayerGrid;

pub struct AnvilDetailPane;

impl DetailPaneContext for AnvilDetailPane {
    type LeftGridMarker = AnvilRecipeGrid;
    type RightGridMarker = AnvilPlayerGrid;

    const LEFT_FOCUS: FocusPanel = FocusPanel::RecipeGrid;
    const RIGHT_FOCUS: FocusPanel = FocusPanel::AnvilInventory;

    fn source_from_left_grid(selection: &ItemGridSelection) -> InfoPanelSource {
        InfoPanelSource::Recipe {
            selected_index: selection.selected_index,
        }
    }

    fn source_from_right_grid(selection: &ItemGridSelection) -> InfoPanelSource {
        InfoPanelSource::Inventory {
            selected_index: selection.selected_index,
        }
    }
}

/// Tracks which anvil entity the modal is open for.
#[derive(Resource)]
pub struct ActiveAnvilEntity(pub Entity);

/// Implements RegisteredModal for the anvil modal.
pub struct AnvilModal;

impl RegisteredModal for AnvilModal {
    type Root = AnvilModalRoot;
    const MODAL_TYPE: ModalType = ModalType::AnvilModal;

    fn spawn(world: &mut World) {
        if let Err(e) = world.run_system_cached(do_spawn_anvil_modal) {
            tracing::error!("Failed to spawn anvil modal: {:?}", e);
        }
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<ActiveAnvilEntity>();
        world.remove_resource::<FocusState>();
    }
}

/// System that spawns the anvil modal UI.
fn do_spawn_anvil_modal(
    commands: Commands,
    game_sprites: Res<crate::assets::GameSprites>,
    game_fonts: Res<crate::assets::GameFonts>,
    player_query: Query<&Inventory, With<PlayerMarker>>,
    registry: Res<crate::item::ItemRegistry>,
) {
    let Ok(inventory) = player_query.single() else {
        tracing::error!("No player inventory found for anvil modal");
        return;
    };
    super::render::spawn_anvil_modal_impl(
        commands,
        &game_sprites,
        &game_fonts,
        inventory,
        &registry,
    );
}
