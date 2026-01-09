use bevy::prelude::*;

use crate::game::{calculate_upgrade_cost, Player};
use crate::item::recipe::{Recipe, RecipeId};
use crate::item::ItemId;
use crate::screens::town::shared::spawn_menu;
use crate::screens::town::TabContent;
use crate::ui::{selection_colors, selection_prefix};
use crate::FindsItems;

use super::constants::MENU_OPTIONS;
use super::state::{BlacksmithMode, BlacksmithModeKind, BlacksmithSelections};

/// Spawn a recipe list UI with selection highlighting.
fn spawn_recipe_list(parent: &mut ChildBuilder, recipes: &[RecipeId], selected_index: usize, player: &Player) {
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
                    let can_craft = recipe.can_craft(player);

                    let (bg_color, text_color) = selection_colors(is_selected);

                    let prefix = selection_prefix(is_selected);

                    list.spawn((
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
                            Text::new(format!("{}{}", prefix, recipe.name())),
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
                                let owned = player
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
                        content.spawn((
                            Text::new("[↑↓] Navigate  [Enter] Select  [←→] Switch Tab"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                            Node {
                                margin: UiRect::top(Val::Auto),
                                ..default()
                            },
                        ));
                    }
                    BlacksmithModeKind::Upgrade => {
                        spawn_upgrade_ui(content, blacksmith_selections, player);
                    }
                    BlacksmithModeKind::Quality => {
                        spawn_quality_ui(content, blacksmith_selections, player);
                    }
                    BlacksmithModeKind::Smelt => {
                        spawn_smelt_ui(content, blacksmith_selections, player);
                    }
                    BlacksmithModeKind::Forge => {
                        spawn_forge_ui(content, blacksmith_selections, player);
                    }
                }
            });
    });
}

/// Spawn the Upgrade mode UI.
fn spawn_upgrade_ui(
    parent: &mut ChildBuilder,
    blacksmith_selections: &BlacksmithSelections,
    player: &Player,
) {
    // Title
    parent.spawn((
        Text::new("Upgrade Equipment"),
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

    // Get equipment items
    let equipment_items: Vec<_> = player
        .inventory
        .items
        .iter()
        .filter(|inv_item| inv_item.item.item_type.is_equipment())
        .collect();

    if equipment_items.is_empty() {
        parent.spawn((
            Text::new("You have no equipment to upgrade."),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
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
                    let can_afford = player.gold >= upgrade_cost;

                    let (bg_color, text_color) = selection_colors(is_selected);

                    let prefix = selection_prefix(is_selected);

                    list.spawn((
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
                            Text::new(format!("{}{}", prefix, inv_item.item.name)),
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
    parent.spawn((
        Text::new("[↑↓] Navigate  [Enter] Upgrade  [Backspace] Back"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        Node {
            margin: UiRect::top(Val::Auto),
            ..default()
        },
    ));
}

/// Spawn the Quality mode UI.
fn spawn_quality_ui(
    parent: &mut ChildBuilder,
    blacksmith_selections: &BlacksmithSelections,
    player: &Player,
) {
    // Title
    parent.spawn((
        Text::new("Improve Quality"),
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

    // Show stone count
    let stone_count = player
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
    let equipment_items: Vec<_> = player
        .inventory
        .items
        .iter()
        .filter(|inv_item| inv_item.item.item_type.is_equipment())
        .collect();

    if equipment_items.is_empty() {
        parent.spawn((
            Text::new("You have no equipment to improve."),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
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

                    list.spawn((
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
                            Text::new(format!("{}{}", prefix, inv_item.item.name)),
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
    parent.spawn((
        Text::new("[↑↓] Navigate  [Enter] Improve  [Backspace] Back"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        Node {
            margin: UiRect::top(Val::Auto),
            ..default()
        },
    ));
}

/// Spawn the Smelt mode UI.
fn spawn_smelt_ui(
    parent: &mut ChildBuilder,
    blacksmith_selections: &BlacksmithSelections,
    player: &Player,
) {
    // Title
    parent.spawn((
        Text::new("Smelt Ores"),
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

    let recipes = RecipeId::all_smelting_recipes();

    if recipes.is_empty() {
        parent.spawn((
            Text::new("No smelting recipes available."),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    } else {
        spawn_recipe_list(parent, &recipes, blacksmith_selections.smelt.selected, player);
    }

    // Navigation hint
    parent.spawn((
        Text::new("[↑↓] Navigate  [Enter] Smelt  [Backspace] Back"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        Node {
            margin: UiRect::top(Val::Auto),
            ..default()
        },
    ));
}

/// Spawn the Forge mode UI.
fn spawn_forge_ui(
    parent: &mut ChildBuilder,
    blacksmith_selections: &BlacksmithSelections,
    player: &Player,
) {
    // Title
    parent.spawn((
        Text::new("Forge Equipment"),
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

    let recipes = RecipeId::all_forging_recipes();

    if recipes.is_empty() {
        parent.spawn((
            Text::new("No forging recipes available."),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    } else {
        spawn_recipe_list(parent, &recipes, blacksmith_selections.forge.selected, player);
    }

    // Navigation hint
    parent.spawn((
        Text::new("[↑↓] Navigate  [Enter] Forge  [Backspace] Back"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 0.5)),
        Node {
            margin: UiRect::top(Val::Auto),
            ..default()
        },
    ));
}
