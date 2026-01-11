use bevy::prelude::*;

use crate::game::calculate_upgrade_cost;
use crate::inventory::{FindsItems, Inventory};
use crate::item::recipe::{Recipe, RecipeId};
use crate::item::ItemId;
use crate::screens::town::shared::{spawn_empty_state, spawn_menu};
use crate::ui::{spawn_navigation_hint, UiText};
use crate::screens::town::TabContent;
use crate::ui::{selection_colors, selection_prefix};

use super::constants::MENU_OPTIONS;
use super::state::{BlacksmithMode, BlacksmithModeKind, BlacksmithSelections};

/// Marker component for selectable items with their index and name.
#[derive(Component)]
pub struct BlacksmithListItem {
    pub index: usize,
    pub name: String,
}

/// Marker for the text of a blacksmith list item.
#[derive(Component)]
pub struct BlacksmithListItemText;

/// Spawn a recipe list UI with selection highlighting.
fn spawn_recipe_list(parent: &mut ChildBuilder, recipes: &[RecipeId], selected_index: usize, inventory: &Inventory) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.0),
            ..default()
        })
        .with_children(|list| {
            for (i, recipe_id) in recipes.iter().enumerate() {
                let is_selected = i == selected_index;

                if let Ok(recipe) = Recipe::new(*recipe_id) {
                    let can_craft = recipe.can_craft(inventory);

                    let (bg_color, text_color) = selection_colors(is_selected);

                    let prefix = selection_prefix(is_selected);

                    let recipe_name = recipe.name().to_string();

                    list.spawn((
                        BlacksmithListItem {
                            index: i,
                            name: recipe_name.clone(),
                        },
                        Node {
                            padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        },
                        BackgroundColor(bg_color),
                    ))
                    .with_children(|item_row| {
                        // Recipe name
                        item_row.spawn((
                            BlacksmithListItemText,
                            Text::new(format!("{}{}", prefix, recipe_name)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(text_color),
                            Node {
                                width: Val::Px(200.0),
                                ..default()
                            },
                        ));

                        // Ingredients
                        let ingredients_text = recipe
                            .ingredients()
                            .iter()
                            .map(|(item_id, qty)| {
                                let owned = inventory
                                    .find_item_by_id(*item_id)
                                    .map(|inv| inv.quantity)
                                    .unwrap_or(0);
                                let item_name = &item_id.spec().name;
                                format!("{}: {}/{}", item_name, owned, qty)
                            })
                            .collect::<Vec<_>>()
                            .join(", ");

                        let ingredients_color = if can_craft {
                            Color::srgb(0.5, 0.9, 0.5)
                        } else {
                            Color::srgb(0.8, 0.3, 0.3)
                        };

                        item_row.spawn((
                            Text::new(ingredients_text),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(ingredients_color),
                        ));
                    });
                }
            }
        });
}

/// Spawn the blacksmith UI based on current mode.
pub fn spawn_blacksmith_ui(
    commands: &mut Commands,
    content_entity: Entity,
    blacksmith_mode: &BlacksmithMode,
    blacksmith_selections: &BlacksmithSelections,
    gold: i32,
    inventory: &Inventory,
) {
    commands.entity(content_entity).with_children(|parent| {
        parent
            .spawn((
                TabContent,
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(20.0),
                    ..default()
                },
            ))
            .with_children(|content| {
                // Render content based on mode
                match blacksmith_mode.mode {
                    BlacksmithModeKind::Menu => {
                        spawn_menu(
                            content,
                            MENU_OPTIONS,
                            blacksmith_selections.menu.selected,
                            Some("Blacksmith"),
                        );

                        // Navigation hint
                        spawn_navigation_hint(
                            content,
                            "[↑↓] Navigate  [Enter] Select  [←→] Switch Tab",
                        );
                    }
                    BlacksmithModeKind::Upgrade => {
                        spawn_upgrade_ui(content, blacksmith_selections, gold, inventory);
                    }
                    BlacksmithModeKind::Quality => {
                        spawn_quality_ui(content, blacksmith_selections, inventory);
                    }
                    BlacksmithModeKind::Smelt => {
                        spawn_smelt_ui(content, blacksmith_selections, inventory);
                    }
                    BlacksmithModeKind::Forge => {
                        spawn_forge_ui(content, blacksmith_selections, inventory);
                    }
                }
            });
    });
}

/// Spawn the Upgrade mode UI.
fn spawn_upgrade_ui(
    parent: &mut ChildBuilder,
    blacksmith_selections: &BlacksmithSelections,
    gold: i32,
    inventory: &Inventory,
) {
    // Title
    parent.spawn(UiText::section("Upgrade Equipment").build_with_node());

    // Get equipment items
    let equipment_items: Vec<_> = inventory.equipment_items().collect();

    if equipment_items.is_empty() {
        spawn_empty_state(parent, "You have no equipment to upgrade.");
    } else {
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|list| {
                for (i, inv_item) in equipment_items.iter().enumerate() {
                    let is_selected = i == blacksmith_selections.upgrade.selected;
                    let upgrade_cost = calculate_upgrade_cost(&inv_item.item);
                    let can_upgrade = inv_item.item.num_upgrades < inv_item.item.max_upgrades;
                    let can_afford = gold >= upgrade_cost;

                    let (bg_color, text_color) = selection_colors(is_selected);

                    let prefix = selection_prefix(is_selected);

                    let item_name = inv_item.item.name.clone();
                    list.spawn((
                        BlacksmithListItem {
                            index: i,
                            name: item_name.clone(),
                        },
                        Node {
                            padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        },
                        BackgroundColor(bg_color),
                    ))
                    .with_children(|item_row| {
                        // Item name
                        item_row.spawn((
                            BlacksmithListItemText,
                            Text::new(format!("{}{}", prefix, item_name)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(text_color),
                            Node {
                                width: Val::Px(200.0),
                                ..default()
                            },
                        ));

                        // Upgrade level
                        item_row.spawn((
                            Text::new(format!(
                                "+{}/{}",
                                inv_item.item.num_upgrades, inv_item.item.max_upgrades
                            )),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            Node {
                                width: Val::Px(80.0),
                                ..default()
                            },
                        ));

                        // Quality
                        item_row.spawn((
                            Text::new(inv_item.item.quality.display_name()),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.8, 0.9)),
                            Node {
                                width: Val::Px(120.0),
                                ..default()
                            },
                        ));

                        // Cost
                        let cost_color = if can_afford && can_upgrade {
                            Color::srgb(0.9, 0.8, 0.3)
                        } else {
                            Color::srgb(0.8, 0.3, 0.3)
                        };

                        let cost_text = if can_upgrade {
                            format!("{} gold", upgrade_cost)
                        } else {
                            "MAX".to_string()
                        };

                        item_row.spawn((
                            Text::new(cost_text),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(cost_color),
                        ));
                    });
                }
            });
    }

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Upgrade  [Backspace] Back");
}

/// Spawn the Quality mode UI.
fn spawn_quality_ui(
    parent: &mut ChildBuilder,
    blacksmith_selections: &BlacksmithSelections,
    inventory: &Inventory,
) {
    // Title
    parent.spawn(UiText::section("Improve Quality").build_with_node());

    // Show stone count
    let stone_count = inventory
        .find_item_by_id(ItemId::QualityUpgradeStone)
        .map(|inv| inv.quantity)
        .unwrap_or(0);

    parent.spawn((
        Text::new(format!("Magic Rocks: {}", stone_count)),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.5, 0.9)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

    // Get equipment items
    let equipment_items: Vec<_> = inventory.equipment_items().collect();

    if equipment_items.is_empty() {
        spawn_empty_state(parent, "You have no equipment to improve.");
    } else {
        parent
            .spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|list| {
                for (i, inv_item) in equipment_items.iter().enumerate() {
                    let is_selected = i == blacksmith_selections.quality.selected;
                    let has_stone = stone_count > 0;
                    let can_upgrade = inv_item.item.quality.next_quality().is_some();

                    let (bg_color, text_color) = selection_colors(is_selected);

                    let prefix = selection_prefix(is_selected);

                    let item_name = inv_item.item.name.clone();
                    list.spawn((
                        BlacksmithListItem {
                            index: i,
                            name: item_name.clone(),
                        },
                        Node {
                            padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            ..default()
                        },
                        BackgroundColor(bg_color),
                    ))
                    .with_children(|item_row| {
                        // Item name
                        item_row.spawn((
                            BlacksmithListItemText,
                            Text::new(format!("{}{}", prefix, item_name)),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(text_color),
                            Node {
                                width: Val::Px(200.0),
                                ..default()
                            },
                        ));

                        // Current quality
                        item_row.spawn((
                            Text::new(inv_item.item.quality.display_name()),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.8, 0.9)),
                            Node {
                                width: Val::Px(120.0),
                                ..default()
                            },
                        ));

                        // Next quality
                        let next_quality_text =
                            if let Some(next) = inv_item.item.quality.next_quality() {
                                format!("→ {}", next.display_name())
                            } else {
                                "MAX".to_string()
                            };

                        let next_quality_color = if has_stone && can_upgrade {
                            Color::srgb(0.5, 0.9, 0.5)
                        } else {
                            Color::srgb(0.8, 0.3, 0.3)
                        };

                        item_row.spawn((
                            Text::new(next_quality_text),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(next_quality_color),
                        ));
                    });
                }
            });
    }

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Improve  [Backspace] Back");
}

/// Spawn the Smelt mode UI.
fn spawn_smelt_ui(
    parent: &mut ChildBuilder,
    blacksmith_selections: &BlacksmithSelections,
    inventory: &Inventory,
) {
    // Title
    parent.spawn(UiText::section("Smelt Ores").build_with_node());

    let recipes = RecipeId::all_smelting_recipes();

    if recipes.is_empty() {
        spawn_empty_state(parent, "No smelting recipes available.");
    } else {
        spawn_recipe_list(parent, &recipes, blacksmith_selections.smelt.selected, inventory);
    }

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Smelt  [Backspace] Back");
}

/// Spawn the Forge mode UI.
fn spawn_forge_ui(
    parent: &mut ChildBuilder,
    blacksmith_selections: &BlacksmithSelections,
    inventory: &Inventory,
) {
    // Title
    parent.spawn(UiText::section("Forge Equipment").build_with_node());

    let recipes = RecipeId::all_forging_recipes();

    if recipes.is_empty() {
        spawn_empty_state(parent, "No forging recipes available.");
    } else {
        spawn_recipe_list(parent, &recipes, blacksmith_selections.forge.selected, inventory);
    }

    // Navigation hint
    spawn_navigation_hint(parent, "[↑↓] Navigate  [Enter] Forge  [Backspace] Back");
}

/// Update list selection highlighting reactively.
pub fn update_blacksmith_list_selection<F1, F2>(
    selected_index: usize,
    list_query: &mut Query<(&BlacksmithListItem, &mut BackgroundColor, &Children), F1>,
    text_query: &mut Query<(&mut Text, &mut TextColor), F2>,
)
where
    F1: bevy::ecs::query::QueryFilter,
    F2: bevy::ecs::query::QueryFilter,
{
    for (item, mut bg_color, children) in list_query.iter_mut() {
        let is_selected = item.index == selected_index;
        let (new_bg, text_color) = selection_colors(is_selected);
        *bg_color = new_bg.into();

        // Update child text
        for &child in children.iter() {
            if let Ok((mut text, mut color)) = text_query.get_mut(child) {
                let prefix = selection_prefix(is_selected);
                **text = format!("{}{}", prefix, item.name);
                *color = text_color.into();
            }
        }
    }
}
