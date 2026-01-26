use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites, GridSlotSlice, SpriteSheetKey, UiSelectorsSlice};
use crate::crafting_station::ForgeCraftingState;
use crate::inventory::{Inventory, ManagesItems};
use crate::item::ItemId;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::screens::modal::spawn_modal_overlay;
use crate::ui::screens::InfoPanelSource;
use crate::ui::widgets::{ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry, ItemGridFocusPanel, ItemStatsDisplay, OutlinedText};

use super::state::{
    ActiveForgeEntity, ForgeModalRoot, ForgeModalState, ForgePlayerGrid, ForgeSlotIndex,
    ForgeSlotsGrid,
};

const SLOT_SIZE: f32 = 48.0;
const SLOT_GAP: f32 = 8.0;
const LABEL_FONT_SIZE: f32 = 12.0;

/// Marker for individual forge slots.
#[derive(Component)]
pub struct ForgeSlotCell {
    pub slot_type: ForgeSlotIndex,
}

/// Marker for item sprites inside forge slots.
#[derive(Component)]
pub struct ForgeSlotItemSprite;

/// Marker for quantity text inside forge slots.
#[derive(Component)]
pub struct ForgeSlotQuantityText;

/// Marker for the slot selector sprite.
#[derive(Component)]
pub struct ForgeSlotSelector {
    pub timer: Timer,
    pub frame: usize,
    pub frame_indices: [usize; 2],
}

/// Convert player inventory items to grid entries for display.
pub fn get_player_inventory_entries(inventory: &Inventory) -> Vec<ItemGridEntry> {
    inventory
        .get_inventory_items()
        .iter()
        .map(|inv_item| ItemGridEntry {
            sprite_sheet_key: inv_item.item.item_id.sprite_sheet_key(),
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            quantity: inv_item.quantity,
        })
        .collect()
}

/// Spawn the forge modal UI with crafting slots, player inventory grid, and detail pane.
/// Called from RegisteredModal::spawn via run_system_cached.
pub fn spawn_forge_modal_impl(
    mut commands: Commands,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    inventory: &Inventory,
    forge_state_query: &Query<&ForgeCraftingState>,
    active_forge: &ActiveForgeEntity,
    modal_state: &ForgeModalState,
) {
    // Initialize focus on player inventory (default)
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::ForgeInventory),
    });

    let player_entries = get_player_inventory_entries(inventory);

    // Get forge crafting state
    let forge_state = forge_state_query.get(active_forge.0).ok();

    let overlay = spawn_modal_overlay(&mut commands);
    commands
        .entity(overlay)
        .insert(ForgeModalRoot)
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|row| {
                    // Crafting slots container (left side)
                    spawn_crafting_slots(
                        row,
                        game_sprites,
                        game_fonts,
                        forge_state,
                        modal_state,
                    );

                    // Player inventory grid (5x5) - focused by default
                    row.spawn((
                        ForgePlayerGrid,
                        ItemGridFocusPanel(FocusPanel::ForgeInventory),
                        ItemGrid {
                            items: player_entries,
                            selected_index: 0,
                            grid_size: 5,
                        },
                    ));

                    // Item detail pane (right side)
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Inventory { selected_index: 0 },
                    });
                });
        });
}

/// Spawn the 3-slot horizontal crafting area.
fn spawn_crafting_slots(
    parent: &mut ChildBuilder,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    forge_state: Option<&ForgeCraftingState>,
    _modal_state: &ForgeModalState,
) {
    // Calculate dimensions for the slots container
    let slots_width = 3.0 * SLOT_SIZE + 2.0 * SLOT_GAP + 32.0; // Extra padding
    let slots_height = SLOT_SIZE + 40.0; // Room for labels

    // Selector is not shown initially - update_forge_slot_selector will add it when needed
    parent
        .spawn((
            ForgeSlotsGrid,
            Node {
                width: Val::Px(slots_width),
                height: Val::Px(slots_height),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.08, 0.06, 0.9)),
            BorderColor(Color::srgb(0.5, 0.4, 0.3)),
            BorderRadius::all(Val::Px(4.0)),
        ))
        .with_children(|container| {
            // Row of 3 slots
            container
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(SLOT_GAP),
                    ..default()
                })
                .with_children(|slot_row| {
                    // Coal slot (not selected initially since inventory is focused)
                    spawn_slot(
                        slot_row,
                        game_sprites,
                        game_fonts,
                        ForgeSlotIndex::Coal,
                        "Coal",
                        forge_state.and_then(|s| s.coal_slot),
                        false,
                    );

                    // Ore slot
                    spawn_slot(
                        slot_row,
                        game_sprites,
                        game_fonts,
                        ForgeSlotIndex::Ore,
                        "Ore",
                        forge_state.and_then(|s| s.ore_slot),
                        false,
                    );

                    // Product slot
                    spawn_slot(
                        slot_row,
                        game_sprites,
                        game_fonts,
                        ForgeSlotIndex::Product,
                        "Ingot",
                        forge_state.and_then(|s| s.product_slot),
                        false,
                    );
                });
        });
}

/// Spawn a single crafting slot with label.
fn spawn_slot(
    parent: &mut ChildBuilder,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    slot_type: ForgeSlotIndex,
    label: &str,
    contents: Option<(ItemId, u32)>,
    is_selected: bool,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|slot_column| {
            // Slot cell
            let mut slot_entity = slot_column.spawn((
                ForgeSlotCell { slot_type },
                Node {
                    width: Val::Px(SLOT_SIZE),
                    height: Val::Px(SLOT_SIZE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    ..default()
                },
            ));

            // Add cell background
            if let Some(cell_img) = game_sprites
                .get(SpriteSheetKey::GridSlot)
                .and_then(|s| s.image_node(GridSlotSlice::Slot.as_str()))
            {
                slot_entity.insert(cell_img);
            }

            slot_entity.with_children(|cell| {
                // Item sprite if slot has contents
                if let Some((item_id, quantity)) = contents {
                    spawn_slot_item(cell, game_sprites, game_fonts, item_id, quantity);
                }

                // Selector if selected
                if is_selected {
                    spawn_slot_selector(cell, game_sprites);
                }
            });

            // Label below slot
            slot_column.spawn((
                Text::new(label),
                game_fonts.pixel_font(LABEL_FONT_SIZE),
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

/// Spawn item sprite and quantity in a slot.
fn spawn_slot_item(
    cell: &mut ChildBuilder,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    item_id: ItemId,
    quantity: u32,
) {
    if let Some(icon_img) = game_sprites
        .get(item_id.sprite_sheet_key())
        .and_then(|s| s.image_node(item_id.sprite_name()))
    {
        cell.spawn((
            ForgeSlotItemSprite,
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            icon_img,
        ));

        // Quantity text if > 1
        if quantity > 1 {
            spawn_slot_quantity_text(cell, game_fonts, quantity);
        }
    }
}

/// Spawn selector sprite in a slot.
fn spawn_slot_selector(cell: &mut ChildBuilder, game_sprites: &GameSprites) {
    if let Some(selectors_sheet) = game_sprites.get(SpriteSheetKey::UiSelectors) {
        if let (Some(idx1), Some(idx2), Some(img)) = (
            selectors_sheet.get(UiSelectorsSlice::SelectorFrame1.as_str()),
            selectors_sheet.get(UiSelectorsSlice::SelectorFrame2.as_str()),
            selectors_sheet.image_node(UiSelectorsSlice::SelectorFrame1.as_str()),
        ) {
            cell.spawn((
                ForgeSlotSelector {
                    timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    frame: 0,
                    frame_indices: [idx1, idx2],
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(SLOT_SIZE),
                    height: Val::Px(SLOT_SIZE),
                    ..default()
                },
                img,
            ));
        }
    }
}

/// Spawn quantity text with outline for slot items.
fn spawn_slot_quantity_text(parent: &mut ChildBuilder, game_fonts: &GameFonts, quantity: u32) {
    let text = quantity.to_string();
    let font_size = 14.0;

    parent
        .spawn((
            ForgeSlotQuantityText,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(2.0),
                bottom: Val::Px(0.0),
                ..default()
            },
        ))
        .with_children(|text_container| {
            // Shadow layers (black outline)
            let offsets = [
                (-1.0, -1.0),
                (0.0, -1.0),
                (1.0, -1.0),
                (-1.0, 0.0),
                (1.0, 0.0),
                (-1.0, 1.0),
                (0.0, 1.0),
                (1.0, 1.0),
            ];
            for (x, y) in offsets {
                text_container.spawn((
                    Text::new(&text),
                    game_fonts.pixel_font(font_size),
                    TextColor(Color::BLACK),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(x),
                        top: Val::Px(y),
                        ..default()
                    },
                ));
            }

            // Main white text on top
            text_container.spawn((
                Text::new(&text),
                game_fonts.pixel_font(font_size),
                TextColor(Color::WHITE),
            ));
        });
}

/// Animate the forge slot selector.
pub fn animate_forge_slot_selector(
    time: Res<Time>,
    mut selectors: Query<(&mut ForgeSlotSelector, &mut ImageNode)>,
) {
    for (mut selector, mut image) in &mut selectors {
        selector.timer.tick(time.delta());
        if selector.timer.just_finished() {
            selector.frame = (selector.frame + 1) % 2;
            if let Some(ref mut atlas) = image.texture_atlas {
                atlas.index = selector.frame_indices[selector.frame];
            }
        }
    }
}

/// Refresh the forge slots display when ForgeCraftingState changes.
/// Uses Bevy's native change detection via `Changed<ForgeCraftingState>`.
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

    // Only refresh if forge state has changed (Changed filter)
    let Ok(forge_state) = forge_state_query.get(active_forge.0) else {
        return;
    };

    // Update each slot's contents
    for (cell_entity, slot_cell, children) in &slot_cells {
        // Remove existing item sprites and quantity text from this cell
        if let Some(children) = children {
            for &child in children.iter() {
                if item_sprites.contains(child) || quantity_texts.contains(child) {
                    if commands.get_entity(child).is_some() {
                        commands.entity(child).despawn_recursive();
                    }
                }
            }
        }

        // Get the contents for this slot
        let contents = match slot_cell.slot_type {
            ForgeSlotIndex::Coal => forge_state.coal_slot,
            ForgeSlotIndex::Ore => forge_state.ore_slot,
            ForgeSlotIndex::Product => forge_state.product_slot,
        };

        // Add new item sprite if slot has contents
        if let Some((item_id, quantity)) = contents {
            commands.entity(cell_entity).with_children(|cell| {
                spawn_slot_item(cell, &game_sprites, &game_fonts, item_id, quantity);
            });
        }
    }
}

/// Update forge slot selector position when modal state or focus changes.
pub fn update_forge_slot_selector(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    focus_state: Option<Res<FocusState>>,
    modal_state: Option<Res<ForgeModalState>>,
    slot_cells: Query<(Entity, &ForgeSlotCell, Option<&Children>)>,
    selectors: Query<Entity, With<ForgeSlotSelector>>,
) {
    let Some(modal_state) = modal_state else {
        return;
    };

    // Remove all existing selectors from forge slots
    for (_, _, children) in &slot_cells {
        if let Some(children) = children {
            for &child in children.iter() {
                if selectors.contains(child) {
                    if commands.get_entity(child).is_some() {
                        commands.entity(child).despawn_recursive();
                    }
                }
            }
        }
    }

    // Only add selector if crafting slots are focused
    let crafting_focused = focus_state
        .as_ref()
        .map(|s| s.is_focused(FocusPanel::ForgeCraftingSlots))
        .unwrap_or(false);

    if !crafting_focused {
        return;
    }

    // Find the selected slot and add selector
    for (cell_entity, slot_cell, _) in &slot_cells {
        if slot_cell.slot_type == modal_state.selected_slot {
            commands.entity(cell_entity).with_children(|cell| {
                spawn_slot_selector(cell, &game_sprites);
            });
            break;
        }
    }
}

/// Populates the item detail pane with the currently selected item.
/// Shows inventory item when inventory is focused, or forge slot item when crafting is focused.
pub fn populate_forge_item_detail_pane(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    inventory: Res<Inventory>,
    focus_state: Option<Res<FocusState>>,
    modal_state: Option<Res<ForgeModalState>>,
    active_forge: Option<Res<ActiveForgeEntity>>,
    forge_state_query: Query<&ForgeCraftingState>,
    player_grids: Query<&ItemGrid, With<ForgePlayerGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    let Some(modal_state) = modal_state else {
        return;
    };

    let Ok(mut pane) = panes.get_single_mut() else {
        return;
    };

    let Ok((content_entity, children)) = content_query.get_single() else {
        return;
    };

    // Force refresh when FocusState changes
    let focus_changed = focus_state.is_changed();

    // Determine source and get item info based on focus
    let (source, item_info): (InfoPanelSource, Option<(ItemId, u32)>) = if focus_state.is_focused(FocusPanel::ForgeCraftingSlots) {
        // Get item from forge slot - use new InfoPanelSource::ForgeSlot
        let source = InfoPanelSource::ForgeSlot { slot: modal_state.selected_slot };

        let item_info = active_forge
            .as_ref()
            .and_then(|af| forge_state_query.get(af.0).ok())
            .and_then(|state| match modal_state.selected_slot {
                ForgeSlotIndex::Coal => state.coal_slot,
                ForgeSlotIndex::Ore => state.ore_slot,
                ForgeSlotIndex::Product => state.product_slot,
            });

        (source, item_info)
    } else if focus_state.is_focused(FocusPanel::ForgeInventory) {
        // Get item from inventory
        let grid = player_grids.get_single().ok();
        let selected_index = grid.map(|g| g.selected_index).unwrap_or(0);
        let source = InfoPanelSource::Inventory { selected_index };

        let item_info = inventory
            .get_inventory_items()
            .get(selected_index)
            .map(|inv_item| (inv_item.item.item_id, inv_item.quantity));

        (source, item_info)
    } else {
        return;
    };

    // Check if we need to update (update on source change, first render, or focus change)
    let needs_initial = children.is_none();
    if pane.source == source && !needs_initial && !focus_changed {
        return;
    }

    // Update pane source
    pane.source = source;

    // Despawn existing content children
    if let Some(children) = children {
        for &child in children.iter() {
            commands.entity(child).despawn_recursive();
        }
    }

    // Get item details from ItemId
    let Some((item_id, quantity)) = item_info else {
        return;
    };

    let item = item_id.spawn();

    // Spawn item details
    commands.entity(content_entity).with_children(|parent| {
        // Item name (quality-colored with black outline)
        parent.spawn(
            OutlinedText::new(&item.name)
                .with_font_size(16.0)
                .with_color(item.quality.color()),
        );

        // Item type
        parent.spawn((
            Text::new(format!("{}", item.item_type)),
            game_fonts.pixel_font(14.0),
            TextColor(Color::srgb(0.7, 0.7, 0.7)),
        ));

        // Quality label
        parent.spawn((
            Text::new(item.quality.display_name()),
            game_fonts.pixel_font(14.0),
            TextColor(item.quality.color()),
        ));

        // Quantity
        if quantity > 1 {
            parent.spawn((
                Text::new(format!("Qty: {}", quantity)),
                game_fonts.pixel_font(14.0),
                TextColor(Color::srgb(0.3, 0.8, 0.3)),
            ));
        }

        // Stats display
        let stats: Vec<_> = item
            .stats
            .stats()
            .iter()
            .map(|(t, si)| (*t, si.current_value))
            .collect();
        if !stats.is_empty() {
            let display = ItemStatsDisplay::from_stats_iter(stats)
                .with_font_size(14.0)
                .with_color(Color::srgb(0.85, 0.85, 0.85));

            parent.spawn(display);
        }
    });
}
