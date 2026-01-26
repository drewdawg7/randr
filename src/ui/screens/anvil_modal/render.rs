//! Rendering for the anvil modal.

use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites};
use crate::inventory::{Inventory, ManagesItems};
use crate::item::recipe::RecipeId;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::screens::modal::spawn_modal_overlay;
use crate::ui::screens::InfoPanelSource;
use crate::ui::widgets::{ItemDetailPane, ItemGrid, ItemGridEntry, ItemGridFocusPanel, OutlinedText};

use super::state::{AnvilModalRoot, AnvilPlayerGrid, AnvilRecipeGrid};

/// Convert forging recipes to grid entries for display.
pub fn get_recipe_entries(inventory: &Inventory) -> Vec<ItemGridEntry> {
    RecipeId::all_forging_recipes()
        .iter()
        .map(|recipe_id| {
            let spec = recipe_id.spec();
            let can_craft = spec
                .ingredients
                .iter()
                .all(|(item_id, required)| inventory.count_item(*item_id) >= *required);

            ItemGridEntry {
                sprite_sheet_key: spec.output.sprite_sheet_key(),
                sprite_name: spec.output.sprite_name().to_string(),
                quantity: if can_craft { 1 } else { 0 }, // 0 = grayed out
            }
        })
        .collect()
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

/// Spawn the anvil modal UI with recipe grid, player inventory, and detail pane.
/// Called from RegisteredModal::spawn via run_system_cached.
pub fn spawn_anvil_modal_impl(
    mut commands: Commands,
    _game_sprites: &GameSprites,
    _game_fonts: &GameFonts,
    inventory: &Inventory,
) {
    // Initialize focus on recipe grid (default)
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::RecipeGrid),
    });

    let recipe_entries = get_recipe_entries(inventory);
    let player_entries = get_player_inventory_entries(inventory);

    let overlay = spawn_modal_overlay(&mut commands);
    commands
        .entity(overlay)
        .insert(AnvilModalRoot)
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|row| {
                    // Recipe grid (left side) - focused by default
                    row.spawn((
                        AnvilRecipeGrid,
                        ItemGridFocusPanel(FocusPanel::RecipeGrid),
                        ItemGrid {
                            items: recipe_entries,
                            selected_index: 0,
                            grid_size: 5,
                        },
                    ));

                    // Player inventory grid (5x5)
                    row.spawn((
                        AnvilPlayerGrid,
                        ItemGridFocusPanel(FocusPanel::AnvilInventory),
                        ItemGrid {
                            items: player_entries,
                            selected_index: 0,
                            grid_size: 5,
                        },
                    ));

                    // Item detail pane (right side) - use Recipe source for initial
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Recipe { selected_index: 0 },
                    });
                });
        });
}

/// Populates the item detail pane with recipe or inventory item info.
pub fn populate_anvil_detail_pane(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    inventory: Res<Inventory>,
    focus_state: Option<Res<FocusState>>,
    recipe_grids: Query<&ItemGrid, With<AnvilRecipeGrid>>,
    player_grids: Query<&ItemGrid, With<AnvilPlayerGrid>>,
    mut panes: Query<&mut ItemDetailPane>,
    content_query: Query<(Entity, Option<&Children>), With<crate::ui::widgets::ItemDetailPaneContent>>,
) {
    let Some(focus_state) = focus_state else {
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

    // Determine source based on focus
    let (source, detail_info): (InfoPanelSource, Option<RecipeOrItem>) =
        if focus_state.is_focused(FocusPanel::RecipeGrid) {
            // Recipe grid focused - use new InfoPanelSource::Recipe variant
            let grid = recipe_grids.get_single().ok();
            let selected_index = grid.map(|g| g.selected_index).unwrap_or(0);
            let source = InfoPanelSource::Recipe { selected_index };

            let recipes = RecipeId::all_forging_recipes();
            let recipe_info = recipes.get(selected_index).map(|recipe_id| {
                let spec = recipe_id.spec();
                let can_craft = spec
                    .ingredients
                    .iter()
                    .all(|(item_id, required)| inventory.count_item(*item_id) >= *required);
                RecipeOrItem::Recipe {
                    recipe_id: *recipe_id,
                    can_craft,
                }
            });

            (source, recipe_info)
        } else if focus_state.is_focused(FocusPanel::AnvilInventory) {
            // Player inventory focused
            let grid = player_grids.get_single().ok();
            let selected_index = grid.map(|g| g.selected_index).unwrap_or(0);
            let source = InfoPanelSource::Inventory { selected_index };

            let item_info = inventory
                .get_inventory_items()
                .get(selected_index)
                .map(|inv_item| RecipeOrItem::Item {
                    item_id: inv_item.item.item_id,
                    quantity: inv_item.quantity,
                });

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

    // Spawn new content
    let Some(detail_info) = detail_info else {
        return;
    };

    commands.entity(content_entity).with_children(|parent| {
        match detail_info {
            RecipeOrItem::Recipe {
                recipe_id,
                can_craft,
            } => {
                let spec = recipe_id.spec();
                let output_item = spec.output.spawn();

                // Recipe name (with black outline)
                let name_color = if can_craft {
                    Color::srgb(0.3, 0.9, 0.3)
                } else {
                    Color::srgb(0.6, 0.6, 0.6)
                };
                parent.spawn(
                    OutlinedText::new(spec.name)
                        .with_font_size(16.0)
                        .with_color(name_color),
                );

                // Output item type
                parent.spawn((
                    Text::new(format!("{}", output_item.item_type)),
                    game_fonts.pixel_font(14.0),
                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                ));

                // Ingredients header
                parent.spawn((
                    Text::new("Ingredients:"),
                    game_fonts.pixel_font(14.0),
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    Node {
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                ));

                // List ingredients with have/need counts
                for (item_id, required) in &spec.ingredients {
                    let have = inventory.count_item(*item_id);
                    let item = item_id.spawn();
                    let color = if have >= *required {
                        Color::srgb(0.3, 0.9, 0.3)
                    } else {
                        Color::srgb(0.9, 0.3, 0.3)
                    };
                    parent.spawn((
                        Text::new(format!("  {} ({}/{})", item.name, have, required)),
                        game_fonts.pixel_font(12.0),
                        TextColor(color),
                    ));
                }

                // Output stats
                let stats: Vec<_> = output_item
                    .stats
                    .stats()
                    .iter()
                    .map(|(t, si)| (*t, si.current_value))
                    .collect();
                if !stats.is_empty() {
                    parent.spawn((
                        Text::new("Stats:"),
                        game_fonts.pixel_font(14.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        Node {
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                    ));
                    let display = crate::ui::widgets::ItemStatsDisplay::from_stats_iter(stats)
                        .with_font_size(12.0)
                        .with_color(Color::srgb(0.85, 0.85, 0.85));
                    parent.spawn(display);
                }
            }
            RecipeOrItem::Item { item_id, quantity } => {
                let item = item_id.spawn();

                // Item name (with black outline)
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
                    let display = crate::ui::widgets::ItemStatsDisplay::from_stats_iter(stats)
                        .with_font_size(14.0)
                        .with_color(Color::srgb(0.85, 0.85, 0.85));
                    parent.spawn(display);
                }
            }
        }
    });
}

/// Helper enum for detail pane content.
enum RecipeOrItem {
    Recipe {
        recipe_id: RecipeId,
        can_craft: bool,
    },
    Item {
        item_id: crate::item::ItemId,
        quantity: u32,
    },
}
