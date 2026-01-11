//! Alchemist tab UI rendering.

use bevy::prelude::*;

use crate::inventory::{FindsItems, Inventory};
use crate::item::recipe::Recipe;
use crate::item::ItemId;
use crate::ui::widgets::AlchemistRecipeItem;
use crate::ui::{selection_colors, selection_prefix, spawn_navigation_hint, UiText};

use super::super::super::shared::spawn_menu;
use super::super::super::TabContent;
use super::state::{AlchemistMode, AlchemistModeKind, AlchemistSelections};
use super::ALCHEMIST_MENU_OPTIONS;

/// Marker for the text of an alchemist recipe item.
#[derive(Component)]
pub struct AlchemistRecipeItemText;

/// Spawn the alchemist UI based on current mode.
pub fn spawn_alchemist_ui(
    commands: &mut Commands,
    content_entity: Entity,
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
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
            .with_children(|content| match alchemist_mode.mode {
                AlchemistModeKind::Menu => spawn_menu_mode(content, alchemist_selections),
                AlchemistModeKind::Brew => {
                    spawn_brew_mode(content, alchemist_mode, alchemist_selections, inventory)
                }
            });
    });
}

/// Spawn the menu mode UI.
fn spawn_menu_mode(content: &mut ChildBuilder, alchemist_selections: &AlchemistSelections) {
    // Menu options
    spawn_menu(
        content,
        ALCHEMIST_MENU_OPTIONS,
        alchemist_selections.menu.selected,
        Some("Alchemist"),
    );

    // Navigation hint
    spawn_navigation_hint(content, "[↑↓] Navigate  [Enter] Select  [←→] Switch Tab");
}

/// Spawn the brew mode UI.
fn spawn_brew_mode(
    content: &mut ChildBuilder,
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
    inventory: &Inventory,
) {
    // Title
    content.spawn(UiText::section("Brew Potions").build_with_node());

    // Main content area with recipe list and details panel
    content
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.0),
            width: Val::Percent(100.0),
            height: Val::Auto,
            ..default()
        },))
        .with_children(|main_content| {
            // Left side: Recipe list
            spawn_recipe_list(main_content, alchemist_mode, alchemist_selections, inventory);

            // Right side: Ingredient details for selected recipe
            spawn_ingredient_details(main_content, alchemist_mode, alchemist_selections, inventory);
        });

    // Navigation hint
    spawn_navigation_hint(content, "[↑↓] Navigate  [Enter] Brew  [Backspace] Back");
}

/// Spawn the recipe list panel.
fn spawn_recipe_list(
    parent: &mut ChildBuilder,
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
    inventory: &Inventory,
) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(50.0),
            row_gap: Val::Px(5.0),
            ..default()
        },))
        .with_children(|list_container| {
            // Header
            list_container.spawn(
                UiText::new("Available Recipes")
                    .medium()
                    .yellow()
                    .margin_bottom(10.0)
                    .build_with_node(),
            );

            // Recipe items
            for (i, &recipe_id) in alchemist_mode.available_recipes.iter().enumerate() {
                if let Ok(recipe) = Recipe::new(recipe_id) {
                    let is_selected = i == alchemist_selections.recipe.selected;
                    spawn_recipe_item(list_container, &recipe, i, is_selected, inventory);
                }
            }
        });
}

/// Spawn a single recipe list item.
fn spawn_recipe_item(parent: &mut ChildBuilder, recipe: &Recipe, index: usize, is_selected: bool, inventory: &Inventory) {
    // Determine if this recipe can be crafted
    let can_craft = recipe.can_craft(inventory);
    let status_text = if can_craft { "[READY]" } else { "[MISSING]" };
    let status_color = if can_craft {
        Color::srgb(0.3, 0.8, 0.3)
    } else {
        Color::srgb(0.8, 0.3, 0.3)
    };

    let (bg_color, text_color) = selection_colors(is_selected);
    let prefix = selection_prefix(is_selected);
    let recipe_name = recipe.name().to_string();

    parent
        .spawn((
            AlchemistRecipeItem::new(index, recipe_name.clone()),
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
                AlchemistRecipeItemText,
                Text::new(format!("{}{}", prefix, recipe_name)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(text_color),
                Node {
                    width: Val::Px(250.0),
                    ..default()
                },
            ));

            // Status indicator
            item_row.spawn((
                Text::new(status_text),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(status_color),
            ));
        });
}

/// Spawn the ingredient details panel for the selected recipe.
fn spawn_ingredient_details(
    parent: &mut ChildBuilder,
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
    inventory: &Inventory,
) {
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(50.0),
            row_gap: Val::Px(5.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },))
        .with_children(|details_container| {
            // Header
            details_container.spawn(
                UiText::new("Ingredients Required")
                    .medium()
                    .yellow()
                    .margin_bottom(10.0)
                    .build_with_node(),
            );

            // Get the selected recipe
            if let Some(&recipe_id) = alchemist_mode
                .available_recipes
                .get(alchemist_selections.recipe.selected)
            {
                if let Ok(recipe) = Recipe::new(recipe_id) {
                    // Display each ingredient with owned/required counts
                    for (&item_id, &required) in recipe.ingredients() {
                        spawn_ingredient_row(details_container, item_id, required, inventory);
                    }
                }
            } else {
                details_container.spawn((
                    Text::new("No recipe selected"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                ));
            }
        });
}

/// Spawn a single ingredient row showing owned/required counts.
fn spawn_ingredient_row(parent: &mut ChildBuilder, item_id: ItemId, required: u32, inventory: &Inventory) {
    // Get the item name from the spec
    let item_name = item_id.spec().name.clone();

    // Check how many the player has
    let owned = inventory
        .find_item_by_id(item_id)
        .map(|inv_item| inv_item.quantity)
        .unwrap_or(0);

    let has_enough = owned >= required;
    let count_color = if has_enough {
        Color::srgb(0.3, 0.8, 0.3)
    } else {
        Color::srgb(0.8, 0.3, 0.3)
    };

    parent
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },))
        .with_children(|ingredient_row| {
            // Ingredient name
            ingredient_row.spawn((
                Text::new(item_name),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    width: Val::Px(200.0),
                    ..default()
                },
            ));

            // Count (owned/required)
            ingredient_row.spawn((
                Text::new(format!("{}/{}", owned, required)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(count_color),
            ));
        });
}
