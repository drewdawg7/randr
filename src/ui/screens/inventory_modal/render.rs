use bevy::prelude::*;

use crate::assets::GameFonts;
use crate::inventory::{EquipmentSlot, Inventory, InventoryItem, ManagesEquipment, ManagesItems};
use crate::ui::screens::modal::spawn_modal_overlay;
use crate::ui::screens::InfoPanelSource;
use crate::ui::widgets::{ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry, ItemStatsDisplay};

use super::state::{BackpackGrid, EquipmentGrid, InventoryModalRoot};

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
    let equipment_entries: Vec<ItemGridEntry> = get_equipment_items(inventory)
        .iter()
        .map(|inv_item| ItemGridEntry {
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
        })
        .collect();

    let backpack_entries: Vec<ItemGridEntry> = get_backpack_items(inventory)
        .iter()
        .map(|inv_item| ItemGridEntry {
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
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
                        ItemGrid {
                            items: equipment_entries,
                            selected_index: 0,
                            is_focused: true,
                            grid_size: 3,
                        },
                    ));
                    // Backpack grid (4x4)
                    row.spawn((
                        BackpackGrid,
                        ItemGrid {
                            items: backpack_entries,
                            selected_index: 0,
                            is_focused: false,
                            grid_size: 4,
                        },
                    ));
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Inventory { selected_index: 0 },
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
    equipment_grids: Query<&ItemGrid, With<EquipmentGrid>>,
    backpack_grids: Query<&ItemGrid, With<BackpackGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    // Determine which grid is focused and build the appropriate source
    let source = if let Ok(grid) = equipment_grids.get_single() {
        if grid.is_focused {
            InfoPanelSource::Equipment { selected_index: grid.selected_index }
        } else if let Ok(bp_grid) = backpack_grids.get_single() {
            InfoPanelSource::Inventory { selected_index: bp_grid.selected_index }
        } else {
            return;
        }
    } else if let Ok(grid) = backpack_grids.get_single() {
        if grid.is_focused {
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

    // Check if we need to update
    let needs_initial = children.is_none();
    if pane.source == source && !needs_initial {
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
        // Item name (quality-colored)
        parent.spawn((
            Text::new(&item.name),
            game_fonts.pixel_font(16.0),
            TextColor(item.quality.color()),
        ));

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

        // Stats display
        let stats: Vec<_> = item
            .stats
            .stats()
            .iter()
            .map(|(t, si)| (*t, si.current_value))
            .collect();
        if !stats.is_empty() {
            parent.spawn(
                ItemStatsDisplay::from_stats_iter(stats)
                    .with_font_size(14.0)
                    .with_color(Color::srgb(0.85, 0.85, 0.85)),
            );
        }
    });
}
