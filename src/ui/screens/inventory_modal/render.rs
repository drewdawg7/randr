use bevy::prelude::*;

use crate::assets::GameFonts;
use crate::inventory::{EquipmentSlot, Inventory, InventoryItem, ManagesEquipment, ManagesItems};
use crate::stats::StatType;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::screens::modal::spawn_modal_overlay;
use crate::ui::screens::InfoPanelSource;
use crate::ui::widgets::{
    ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry, ItemGridFocusPanel,
    ItemStatsDisplay, OutlinedText,
};

use super::state::{BackpackGrid, EquipmentGrid, InventoryModalRoot};

/// Sync system that reactively updates grids when inventory changes.
/// Uses Bevy's native change detection via `is_changed()`.
pub fn sync_inventory_to_grids(
    inventory: Res<Inventory>,
    mut equipment_grids: Query<&mut ItemGrid, (With<EquipmentGrid>, Without<BackpackGrid>)>,
    mut backpack_grids: Query<&mut ItemGrid, (With<BackpackGrid>, Without<EquipmentGrid>)>,
) {
    if !inventory.is_changed() {
        return;
    }

    if let Ok(mut eq_grid) = equipment_grids.get_single_mut() {
        eq_grid.items = get_equipment_items(&inventory)
            .iter()
            .map(|inv_item| ItemGridEntry {
                sprite_sheet_key: inv_item.item.item_id.sprite_sheet_key(),
                sprite_name: inv_item.item.item_id.sprite_name().to_string(),
                quantity: inv_item.quantity,
            })
            .collect();
        if !eq_grid.items.is_empty() {
            eq_grid.selected_index = eq_grid.selected_index.min(eq_grid.items.len() - 1);
        } else {
            eq_grid.selected_index = 0;
        }
    }

    if let Ok(mut bp_grid) = backpack_grids.get_single_mut() {
        bp_grid.items = get_backpack_items(&inventory)
            .iter()
            .map(|inv_item| ItemGridEntry {
                sprite_sheet_key: inv_item.item.item_id.sprite_sheet_key(),
                sprite_name: inv_item.item.item_id.sprite_name().to_string(),
                quantity: inv_item.quantity,
            })
            .collect();
        if !bp_grid.items.is_empty() {
            bp_grid.selected_index = bp_grid.selected_index.min(bp_grid.items.len() - 1);
        } else {
            bp_grid.selected_index = 0;
        }
    }
}

/// Returns equipment items in slot order. Each entry corresponds to an EquipmentSlot.
/// Only populated slots produce entries.
pub fn get_equipment_items(inventory: &Inventory) -> Vec<&InventoryItem> {
    EquipmentSlot::all()
        .iter()
        .filter_map(|slot| inventory.get_equipped_item(*slot))
        .collect()
}

/// Gets comparison stats from the equipped item in the same slot as the given item.
/// Returns None if the item is not equipment, or Some(empty vec) if slot is empty.
fn get_comparison_stats(
    item: &crate::item::Item,
    inventory: &Inventory,
) -> Option<Vec<(StatType, i32)>> {
    // Only compare equipment items
    let slot = item.item_type.equipment_slot()?;

    // Get equipped item stats (empty vec if slot is empty)
    let comparison: Vec<_> = inventory
        .get_equipped_item(slot)
        .map(|equipped| {
            equipped
                .item
                .stats
                .stats()
                .iter()
                .map(|(t, si)| (*t, si.current_value))
                .collect()
        })
        .unwrap_or_default();

    Some(comparison)
}

/// Returns backpack (non-equipped) items.
pub fn get_backpack_items(inventory: &Inventory) -> Vec<&InventoryItem> {
    inventory.get_inventory_items().iter().collect()
}

/// Spawn the inventory modal UI with an equipment grid, backpack grid, and detail pane.
pub fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    // Initialize focus on equipment grid
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::EquipmentGrid),
    });

    let equipment_entries: Vec<ItemGridEntry> = get_equipment_items(inventory)
        .iter()
        .map(|inv_item| ItemGridEntry {
            sprite_sheet_key: inv_item.item.item_id.sprite_sheet_key(),
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            quantity: inv_item.quantity,
        })
        .collect();

    let backpack_entries: Vec<ItemGridEntry> = get_backpack_items(inventory)
        .iter()
        .map(|inv_item| ItemGridEntry {
            sprite_sheet_key: inv_item.item.item_id.sprite_sheet_key(),
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            quantity: inv_item.quantity,
        })
        .collect();

    let overlay = spawn_modal_overlay(commands);
    commands
        .entity(overlay)
        .insert(InventoryModalRoot)
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|row| {
                    // Equipment grid (3x3) - focused by default
                    row.spawn((
                        EquipmentGrid,
                        ItemGridFocusPanel(FocusPanel::EquipmentGrid),
                        ItemGrid {
                            items: equipment_entries,
                            selected_index: 0,
                            grid_size: 3,
                        },
                    ));
                    // Backpack grid (4x4)
                    row.spawn((
                        BackpackGrid,
                        ItemGridFocusPanel(FocusPanel::BackpackGrid),
                        ItemGrid {
                            items: backpack_entries,
                            selected_index: 0,
                            grid_size: 4,
                        },
                    ));
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Equipment { selected_index: 0 },
                    });
                });
        });
}

/// Populates the item detail pane with the currently selected item's information.
/// Runs when ItemGrid selection changes or when the content container is first created.
pub fn populate_item_detail_pane(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    inventory: Res<Inventory>,
    focus_state: Option<Res<FocusState>>,
    equipment_grids: Query<&ItemGrid, With<EquipmentGrid>>,
    backpack_grids: Query<&ItemGrid, With<BackpackGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    // Determine which grid is focused and build the appropriate source
    let source = if focus_state.is_focused(FocusPanel::EquipmentGrid) {
        if let Ok(grid) = equipment_grids.get_single() {
            InfoPanelSource::Equipment { selected_index: grid.selected_index }
        } else {
            return;
        }
    } else if focus_state.is_focused(FocusPanel::BackpackGrid) {
        if let Ok(grid) = backpack_grids.get_single() {
            InfoPanelSource::Inventory { selected_index: grid.selected_index }
        } else {
            return;
        }
    } else {
        return;
    };

    let Ok(mut pane) = panes.get_single_mut() else {
        return;
    };

    let Ok((content_entity, children)) = content_query.get_single() else {
        return;
    };

    // Check if we need to update:
    // - First render (no children yet)
    // - Source changed (different selection or focus)
    // - Inventory changed (item at same index may be different)
    let needs_initial = children.is_none();
    let inventory_changed = inventory.is_changed();
    if pane.source == source && !needs_initial && !inventory_changed {
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

    // Look up the selected item based on which grid is focused
    let inv_item = match source {
        InfoPanelSource::Equipment { selected_index } => {
            get_equipment_items(&inventory).into_iter().nth(selected_index)
        }
        InfoPanelSource::Inventory { selected_index } => {
            get_backpack_items(&inventory).into_iter().nth(selected_index)
        }
        _ => None,
    };

    let Some(inv_item) = inv_item else {
        return;
    };

    let item = &inv_item.item;

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
        if inv_item.quantity > 1 {
            parent.spawn((
                Text::new(format!("Qty: {}", inv_item.quantity)),
                game_fonts.pixel_font(14.0),
                TextColor(Color::srgb(0.3, 0.8, 0.3)),
            ));
        }

        // Stats display with comparison for backpack items
        let stats: Vec<_> = item
            .stats
            .stats()
            .iter()
            .map(|(t, si)| (*t, si.current_value))
            .collect();
        if !stats.is_empty() {
            let mut display = ItemStatsDisplay::from_stats_iter(stats)
                .with_font_size(14.0)
                .with_color(Color::srgb(0.85, 0.85, 0.85));

            // Add comparison for backpack items (not for already-equipped items)
            if matches!(source, InfoPanelSource::Inventory { .. }) {
                if let Some(comparison) = get_comparison_stats(item, &inventory) {
                    display = display.with_comparison(comparison);
                }
            }

            parent.spawn(display);
        }
    });
}
