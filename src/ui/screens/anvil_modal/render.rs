use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites};
use crate::inventory::{Inventory, ManagesItems};
use crate::item::recipe::RecipeId;
use crate::player::PlayerMarker;
use crate::ui::focus::FocusPanel;
use crate::ui::modal_content_row;
use crate::ui::InfoPanelSource;
use crate::ui::widgets::{
    ItemDetailDisplay, ItemDetailPane, ItemDetailPaneContent, ItemGrid, ItemGridEntry,
    ItemGridFocusPanel, ItemGridSelection, ItemStatsDisplay, OutlinedText,
};
use crate::ui::{FocusState, Modal, ModalBackground, SpawnModalExt};

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


/// Spawn the anvil modal UI with recipe grid, player inventory, and detail pane.
/// Called from RegisteredModal::spawn via run_system_cached.
pub fn spawn_anvil_modal_impl(
    mut commands: Commands,
    _game_sprites: &GameSprites,
    _game_fonts: &GameFonts,
    inventory: &Inventory,
) {
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::RecipeGrid),
    });

    let recipe_entries = get_recipe_entries(inventory);
    let player_entries = ItemGridEntry::from_inventory(inventory);

    commands.spawn_modal(
        Modal::builder()
            .background(ModalBackground::None)
            .root_marker(Box::new(|e| {
                e.insert(AnvilModalRoot);
            }))
            .content(Box::new(move |c| {
                c.spawn(modal_content_row()).with_children(|row| {
                    row.spawn((
                        AnvilRecipeGrid,
                        ItemGridFocusPanel(FocusPanel::RecipeGrid),
                        ItemGrid {
                            items: recipe_entries,
                            grid_size: 5,
                        },
                        ItemGridSelection::default(),
                    ));
                    row.spawn((
                        AnvilPlayerGrid,
                        ItemGridFocusPanel(FocusPanel::AnvilInventory),
                        ItemGrid {
                            items: player_entries,
                            grid_size: 5,
                        },
                        ItemGridSelection::default(),
                    ));
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::Recipe { selected_index: 0 },
                    });
                });
            }))
            .build(),
    );
}

pub fn populate_anvil_detail_pane_content(
    mut commands: Commands,
    game_fonts: Res<GameFonts>,
    player: Query<&Inventory, With<PlayerMarker>>,
    panes: Query<Ref<ItemDetailPane>>,
    content_query: Query<(Entity, Option<&Children>), With<ItemDetailPaneContent>>,
) {
    let Ok(inventory) = player.single() else {
        return;
    };

    for pane in &panes {
        if !pane.is_changed() {
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

        let detail_info: Option<RecipeOrItem> = match pane.source {
            InfoPanelSource::Recipe { selected_index } => {
                let recipes = RecipeId::all_forging_recipes();
                recipes.get(selected_index).map(|recipe_id| {
                    let spec = recipe_id.spec();
                    let can_craft = spec
                        .ingredients
                        .iter()
                        .all(|(item_id, required)| inventory.count_item(*item_id) >= *required);
                    RecipeOrItem::Recipe {
                        recipe_id: *recipe_id,
                        can_craft,
                    }
                })
            }
            InfoPanelSource::Inventory { selected_index } => inventory
                .get_inventory_items()
                .get(selected_index)
                .map(|inv_item| RecipeOrItem::Item {
                    item_id: inv_item.item.item_id,
                    quantity: inv_item.quantity,
                }),
            _ => None,
        };

        let Some(detail_info) = detail_info else {
            continue;
        };

        commands.entity(content_entity).with_children(|parent| {
            match detail_info {
                RecipeOrItem::Recipe {
                    recipe_id,
                    can_craft,
                } => {
                    let spec = recipe_id.spec();
                    let output_item = spec.output.spawn();

                    let name_color = if can_craft {
                        Color::srgb(0.3, 0.9, 0.3)
                    } else {
                        Color::srgb(0.6, 0.6, 0.6)
                    };
                    parent.spawn(
                        OutlinedText::builder(spec.name)
                            .font_size(16.0)
                            .text_color(name_color)
                            .build(),
                    );

                    parent.spawn((
                        Text::new(format!("{}", output_item.item_type)),
                        game_fonts.pixel_font(14.0),
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));

                    parent.spawn((
                        Text::new("Ingredients:"),
                        game_fonts.pixel_font(14.0),
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        Node {
                            margin: UiRect::top(Val::Px(8.0)),
                            ..default()
                        },
                    ));

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
                        let display = ItemStatsDisplay::builder(stats)
                            .font_size(12.0)
                            .text_color(Color::srgb(0.85, 0.85, 0.85))
                            .build();
                        parent.spawn(display);
                    }
                }
                RecipeOrItem::Item { item_id, quantity } => {
                    let item = item_id.spawn();
                    let display = ItemDetailDisplay::builder(&item).quantity(quantity).build();
                    parent.spawn(display);
                }
            }
        });
    }
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
