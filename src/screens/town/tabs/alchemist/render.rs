//! Alchemist tab UI rendering.

use bevy::prelude::*;

use crate::game::Player;
use crate::inventory::FindsItems;
use crate::item::recipe::Recipe;
use crate::item::ItemId;
use crate::ui::{selection_colors, selection_prefix};

use super::super::super::shared::{spawn_menu, spawn_navigation_hint};
use super::super::super::TabContent;
use super::state::{AlchemistMode, AlchemistModeKind, AlchemistSelections};
use super::ALCHEMIST_MENU_OPTIONS;

/// Marker component for alchemist recipe list items.
#[derive(Component)]
pub struct AlchemistRecipeItem {
    pub index: usize,
    pub name: String,
}

/// Marker for the text of an alchemist recipe item.
#[derive(Component)]
pub struct AlchemistRecipeItemText;

/// Spawn the alchemist UI based on current mode.
pub fn spawn_alchemist_ui(
    commands: &mut Commands,
    content_entity: Entity,
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
    player: &Player,
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
                    spawn_brew_mode(content, alchemist_mode, alchemist_selections, player)
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
        alchemist_selections.menu,
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
    player: &Player,
) {
    // Title
    content.spawn((
        Text::new("Brew Potions"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.5)),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    ));

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
            spawn_recipe_list(main_content, alchemist_mode, alchemist_selections, player);

            // Right side: Ingredient details for selected recipe
            spawn_ingredient_details(main_content, alchemist_mode, alchemist_selections, player);
        });

    // Navigation hint
    spawn_navigation_hint(content, "[↑↓] Navigate  [Enter] Brew  [Backspace] Back");
}

/// Spawn the recipe list panel.
fn spawn_recipe_list(
    parent: &mut ChildBuilder,
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
    player: &Player,
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
            list_container.spawn((
                Text::new("Available Recipes"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.5)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Recipe items
            for (i, &recipe_id) in alchemist_mode.available_recipes.iter().enumerate() {
                if let Ok(recipe) = Recipe::new(recipe_id) {
                    let is_selected = i == alchemist_selections.recipe.selected;
                    spawn_recipe_item(list_container, &recipe, i, is_selected, player);
                }
            }
        });
}

/// Spawn a single recipe list item.
fn spawn_recipe_item(parent: &mut ChildBuilder, recipe: &Recipe, index: usize, is_selected: bool, player: &Player) {
    // Determine if this recipe can be crafted
    let can_craft = recipe.can_craft(player);
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
            AlchemistRecipeItem {
                index,
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
    player: &Player,
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
            details_container.spawn((
                Text::new("Ingredients Required"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.5)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));

            // Get the selected recipe
            if let Some(&recipe_id) = alchemist_mode
                .available_recipes
                .get(alchemist_selections.recipe.selected)
            {
                if let Ok(recipe) = Recipe::new(recipe_id) {
                    // Display each ingredient with owned/required counts
                    for (&item_id, &required) in recipe.ingredients() {
                        spawn_ingredient_row(details_container, item_id, required, player);
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
fn spawn_ingredient_row(parent: &mut ChildBuilder, item_id: ItemId, required: u32, player: &Player) {
    // Get the item name from the spec
    let item_name = item_id.spec().name.clone();

    // Check how many the player has
    let owned = player
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

/// Update alchemist recipe selection highlighting reactively.
pub fn update_alchemist_recipe_selection<F1, F2>(
    selected_index: usize,
    recipe_query: &mut Query<(&AlchemistRecipeItem, &mut BackgroundColor, &Children), F1>,
    text_query: &mut Query<(&mut Text, &mut TextColor), F2>,
)
where
    F1: bevy::ecs::query::QueryFilter,
    F2: bevy::ecs::query::QueryFilter,
{
    for (item, mut bg_color, children) in recipe_query.iter_mut() {
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
