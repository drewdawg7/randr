use bevy::prelude::*;

use crate::assets::GameFonts;
use crate::inventory::{EquipmentSlot, Inventory, InventoryItem, ManagesEquipment, ManagesItems};
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::modal_content_row;
use crate::ui::screens::InfoPanelSource;
use crate::ui::widgets::{
    ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry, ItemGridFocusPanel,
    ItemStatsDisplay, OutlinedText,
};
use crate::ui::{Modal, ModalBackground, SpawnModalExt};

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
            .map(|inv_item| ItemGridEntry::from_inventory_item(inv_item))
            .collect();
        if !eq_grid.items.is_empty() {
            eq_grid.selected_index = eq_grid.selected_index.min(eq_grid.items.len() - 1);
        } else {
            eq_grid.selected_index = 0;
        }
    }

    if let Ok(mut bp_grid) = backpack_grids.get_single_mut() {
        bp_grid.items = ItemGridEntry::from_inventory(&inventory);
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

/// Returns backpack (non-equipped) items.
pub fn get_backpack_items(inventory: &Inventory) -> Vec<&InventoryItem> {
    inventory.get_inventory_items().iter().collect()
}

/// Spawn the inventory modal UI with an equipment grid, backpack grid, and detail pane.
pub fn spawn_inventory_modal(commands: &mut Commands, inventory: &Inventory) {
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::EquipmentGrid),
    });

    let equipment_entries: Vec<ItemGridEntry> = get_equipment_items(inventory)
        .iter()
        .map(|inv_item| ItemGridEntry::from_inventory_item(inv_item))
        .collect();

    let backpack_entries: Vec<ItemGridEntry> = ItemGridEntry::from_inventory(inventory);

    commands.spawn_modal(
        Modal::new()
            .background(ModalBackground::None)
            .with_root_marker(|e| {
                e.insert(InventoryModalRoot);
            })
            .content(move |c| {
                c.spawn(modal_content_row()).with_children(|row| {
                    row.spawn((
                        EquipmentGrid,
                        ItemGridFocusPanel(FocusPanel::EquipmentGrid),
                        ItemGrid {
                            items: equipment_entries,
                            selected_index: 0,
                            grid_size: 3,
                        },
                    ));
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
            }),
    );
}

/// Updates the detail pane source based on which grid is focused and selected.
/// Only runs when focus or grid selection changes.
pub fn update_inventory_detail_pane_source(
    focus_state: Option<Res<FocusState>>,
    equipment_grids: Query<Ref<ItemGrid>, With<EquipmentGrid>>,
    backpack_grids: Query<Ref<ItemGrid>, With<BackpackGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    // Check if focus or any grid changed
    let focus_changed = focus_state.is_changed();
    let eq_changed = equipment_grids
        .get_single()
        .map(|g| g.is_changed())
        .unwrap_or(false);
    let bp_changed = backpack_grids
        .get_single()
        .map(|g| g.is_changed())
        .unwrap_or(false);

    if !focus_changed && !eq_changed && !bp_changed {
        return;
    }

    // Determine source from focused grid
    let source = if focus_state.is_focused(FocusPanel::EquipmentGrid) {
        equipment_grids
            .get_single()
            .ok()
            .map(|g| InfoPanelSource::Equipment {
                selected_index: g.selected_index,
            })
    } else if focus_state.is_focused(FocusPanel::BackpackGrid) {
        backpack_grids
            .get_single()
            .ok()
            .map(|g| InfoPanelSource::Inventory {
                selected_index: g.selected_index,
            })
    } else {
        None
    };

    let Some(source) = source else {
        return;
    };

    // Update pane source (only if different to avoid unnecessary Changed trigger)
    for mut pane in &mut panes {
        if pane.source != source {
            pane.source = source;
        }
    }
}

/// Populates the detail pane content when the source or inventory changes.
/// Uses Ref<ItemDetailPane> for change detection.
pub fn populate_inventory_detail_pane_content(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    inventory: Res<Inventory>,
    panes: Query<Ref<ItemDetailPane>>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let inventory_changed = inventory.is_changed();

    for pane in &panes {
        // Check if we need to update: pane.source changed OR inventory changed
        if !pane.is_changed() && !inventory_changed {
            continue;
        }

        let Ok((content_entity, children)) = content_query.get_single() else {
            continue;
        };

        // Despawn existing content children
        if let Some(children) = children {
            for &child in children.iter() {
                commands.entity(child).despawn_recursive();
            }
        }

        // Look up the selected item based on source
        let inv_item = match pane.source {
            InfoPanelSource::Equipment { selected_index } => {
                get_equipment_items(&inventory).into_iter().nth(selected_index)
            }
            InfoPanelSource::Inventory { selected_index } => {
                get_backpack_items(&inventory).into_iter().nth(selected_index)
            }
            _ => None,
        };

        let Some(inv_item) = inv_item else {
            continue;
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
                if matches!(pane.source, InfoPanelSource::Inventory { .. }) {
                    if let Some(comparison) = inventory.get_comparison_stats(item) {
                        display = display.with_comparison(comparison);
                    }
                }

                parent.spawn(display);
            }
        });
    }
}
