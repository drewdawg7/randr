use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites};
use crate::crafting_station::ForgeCraftingState;
use crate::inventory::{Inventory, ManagesItems};
use crate::item::ItemId;
use crate::player::PlayerMarker;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::widgets::{
    spawn_selector, AnimatedSelector, ItemDetailDisplay, ItemDetailPane, ItemDetailPaneContent,
    ItemGridSelection,
};
use crate::ui::InfoPanelSource;

use super::components::{ForgeSlotCell, ForgeSlotItemSprite, ForgeSlotQuantityText};
use super::spawning::spawn_slot_item;
use super::state::{ActiveForgeEntity, ForgeModalState, ForgePlayerGrid, ForgeSlotIndex};

pub fn refresh_forge_slots(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    game_fonts: Res<GameFonts>,
    active_forge: Option<Res<ActiveForgeEntity>>,
    forge_state_query: Query<&ForgeCraftingState, Changed<ForgeCraftingState>>,
    slot_cells: Query<(Entity, &ForgeSlotCell, Option<&Children>)>,
    item_sprites: Query<Entity, With<ForgeSlotItemSprite>>,
    quantity_texts: Query<Entity, With<ForgeSlotQuantityText>>,
) {
    let Some(active_forge) = active_forge else {
        return;
    };

    let Ok(forge_state) = forge_state_query.get(active_forge.0) else {
        return;
    };

    for (cell_entity, slot_cell, children) in &slot_cells {
        if let Some(children) = children {
            for child in children.iter() {
                if item_sprites.contains(child) || quantity_texts.contains(child) {
                    if commands.get_entity(child).is_ok() {
                        commands.entity(child).despawn();
                    }
                }
            }
        }

        let contents = match slot_cell.slot_type {
            ForgeSlotIndex::Coal => forge_state.coal_slot,
            ForgeSlotIndex::Ore => forge_state.ore_slot,
            ForgeSlotIndex::Product => forge_state.product_slot,
        };

        if let Some((item_id, quantity)) = contents {
            commands.entity(cell_entity).with_children(|cell| {
                spawn_slot_item(cell, &game_sprites, &game_fonts, item_id, quantity);
            });
        }
    }
}

pub fn update_forge_slot_selector(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    focus_state: Option<Res<FocusState>>,
    modal_state: Option<Res<ForgeModalState>>,
    slot_cells: Query<(Entity, &ForgeSlotCell, Option<&Children>)>,
    selectors: Query<Entity, With<AnimatedSelector>>,
) {
    let Some(modal_state) = modal_state else {
        return;
    };

    for (_, _, children) in &slot_cells {
        if let Some(children) = children {
            for child in children.iter() {
                if selectors.contains(child) {
                    if commands.get_entity(child).is_ok() {
                        commands.entity(child).despawn();
                    }
                }
            }
        }
    }

    let crafting_focused = focus_state
        .as_ref()
        .map(|s| s.is_focused(FocusPanel::ForgeCraftingSlots))
        .unwrap_or(false);

    if !crafting_focused {
        return;
    }

    for (cell_entity, slot_cell, _) in &slot_cells {
        if slot_cell.slot_type == modal_state.selected_slot {
            commands.entity(cell_entity).with_children(|cell| {
                spawn_selector(cell, &game_sprites);
            });
            break;
        }
    }
}

pub fn update_forge_detail_pane_source(
    focus_state: Option<Res<FocusState>>,
    modal_state: Option<Res<ForgeModalState>>,
    player_grids: Query<&ItemGridSelection, With<ForgePlayerGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    let Some(modal_state) = modal_state else {
        return;
    };

    let source = if focus_state.is_focused(FocusPanel::ForgeCraftingSlots) {
        Some(InfoPanelSource::ForgeSlot {
            slot: modal_state.selected_slot,
        })
    } else if focus_state.is_focused(FocusPanel::ForgeInventory) {
        player_grids
            .single()
            .ok()
            .map(|s| InfoPanelSource::Inventory {
                selected_index: s.selected_index,
            })
    } else {
        None
    };

    let Some(source) = source else {
        return;
    };

    for mut pane in &mut panes {
        if pane.source != source {
            pane.source = source;
        }
    }
}

pub fn populate_forge_detail_pane_content(
    mut commands: Commands,
    player: Query<&Inventory, With<PlayerMarker>>,
    active_forge: Option<Res<ActiveForgeEntity>>,
    forge_state_query: Query<Ref<ForgeCraftingState>>,
    panes: Query<Ref<ItemDetailPane>>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Ok(inventory) = player.single() else { return };
    let inventory_changed = false;
    let forge_state_changed = active_forge
        .as_ref()
        .and_then(|af| forge_state_query.get(af.0).ok())
        .map(|s| s.is_changed())
        .unwrap_or(false);

    for pane in &panes {
        if !pane.is_changed() && !inventory_changed && !forge_state_changed {
            continue;
        }

        let Ok((content_entity, children)) = content_query.single() else {
            continue;
        };

        if let Some(children) = children {
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        let item_info: Option<(ItemId, u32)> = match pane.source {
            InfoPanelSource::ForgeSlot { slot } => active_forge
                .as_ref()
                .and_then(|af| forge_state_query.get(af.0).ok())
                .and_then(|state| match slot {
                    ForgeSlotIndex::Coal => state.coal_slot,
                    ForgeSlotIndex::Ore => state.ore_slot,
                    ForgeSlotIndex::Product => state.product_slot,
                }),
            InfoPanelSource::Inventory { selected_index } => inventory
                .get_inventory_items()
                .get(selected_index)
                .map(|inv_item| (inv_item.item.item_id, inv_item.quantity)),
            _ => None,
        };

        let Some((item_id, quantity)) = item_info else {
            continue;
        };

        let item = item_id.spawn();
        let display = ItemDetailDisplay::builder(&item).quantity(quantity).build();

        commands.entity(content_entity).with_children(|parent| {
            parent.spawn(display);
        });
    }
}
