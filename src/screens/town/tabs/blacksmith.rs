use bevy::prelude::*;

use crate::game::PlayerResource;
use crate::input::{GameAction, NavigationDirection};
use crate::states::AppState;
use crate::{FindsItems, ManagesItems};
use crate::item::recipe::{Recipe, RecipeId};
use crate::item::ItemId;

use super::super::shared::{spawn_menu, MenuOption, SelectionState};
use super::super::{ContentArea, CurrentTab, TabContent, TownTab};

/// Plugin for the Blacksmith tab.
pub struct BlacksmithTabPlugin;

impl Plugin for BlacksmithTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlacksmithTabState>().add_systems(
            Update,
            (
                handle_blacksmith_input,
                render_blacksmith_content.run_if(resource_changed::<BlacksmithTabState>),
            )
                .run_if(in_state(AppState::Town))
                .run_if(|tab: Res<CurrentTab>| tab.tab == TownTab::Blacksmith),
        );
    }
}

/// The different modes/screens in the Blacksmith tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlacksmithMode {
    #[default]
    Menu,
    Upgrade,
    Quality,
    Smelt,
    Forge,
}

/// Blacksmith tab state - tracks current mode and selection.
#[derive(Resource)]
pub struct BlacksmithTabState {
    pub mode: BlacksmithMode,
    pub menu_selection: SelectionState,
    pub upgrade_selection: SelectionState,
    pub quality_selection: SelectionState,
    pub smelt_selection: SelectionState,
    pub forge_selection: SelectionState,
}

impl Default for BlacksmithTabState {
    fn default() -> Self {
        Self {
            mode: BlacksmithMode::Menu,
            menu_selection: SelectionState {
                selected: 0,
                count: MENU_OPTIONS.len(),
                scroll_offset: 0,
                visible_count: 10,
            },
            upgrade_selection: SelectionState::new(0),
            quality_selection: SelectionState::new(0),
            smelt_selection: SelectionState::new(0),
            forge_selection: SelectionState::new(0),
        }
    }
}

const MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        label: "Upgrade",
        description: Some("Upgrade your equipment"),
    },
    MenuOption {
        label: "Quality",
        description: Some("Improve item quality"),
    },
    MenuOption {
        label: "Smelt",
        description: Some("Break down items for materials"),
    },
    MenuOption {
        label: "Forge",
        description: Some("Craft new items"),
    },
];

/// Handle input for the Blacksmith tab.
fn handle_blacksmith_input(
    mut blacksmith_state: ResMut<BlacksmithTabState>,
    mut player: ResMut<PlayerResource>,
    mut action_events: EventReader<GameAction>,
) {
    for action in action_events.read() {
        match blacksmith_state.mode {
            BlacksmithMode::Menu => handle_menu_input(&mut blacksmith_state, action),
            BlacksmithMode::Upgrade => handle_upgrade_input(&mut blacksmith_state, &mut player, action),
            BlacksmithMode::Quality => handle_quality_input(&mut blacksmith_state, &mut player, action),
            BlacksmithMode::Smelt => handle_smelt_input(&mut blacksmith_state, &mut player, action),
            BlacksmithMode::Forge => handle_forge_input(&mut blacksmith_state, &mut player, action),
        }
    }
}

/// Handle input for the main menu.
fn handle_menu_input(blacksmith_state: &mut BlacksmithTabState, action: &GameAction) {
    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_state.menu_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_state.menu_selection.move_down();
        }
        GameAction::Select => {
            blacksmith_state.mode = match blacksmith_state.menu_selection.selected {
                0 => BlacksmithMode::Upgrade,
                1 => BlacksmithMode::Quality,
                2 => BlacksmithMode::Smelt,
                3 => BlacksmithMode::Forge,
                _ => BlacksmithMode::Menu,
            };
        }
        _ => {}
    }
}

/// Handle input for Upgrade mode.
fn handle_upgrade_input(
    blacksmith_state: &mut BlacksmithTabState,
    player: &mut PlayerResource,
    action: &GameAction,
) {
    // Get equipment items and update selection count
    let equipment_count = player
        .inventory
        .items
        .iter()
        .filter(|inv_item| inv_item.item.item_type.is_equipment())
        .count();
    blacksmith_state.upgrade_selection.set_count(equipment_count);

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_state.upgrade_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_state.upgrade_selection.move_down();
        }
        GameAction::Select => {
            // Get equipment items
            let equipment_items: Vec<_> = player
                .inventory
                .items
                .iter()
                .filter(|inv_item| inv_item.item.item_type.is_equipment())
                .collect();

            if let Some(inv_item) = equipment_items.get(blacksmith_state.upgrade_selection.selected) {
                let item_uuid = inv_item.item.item_uuid;

                // Calculate upgrade cost first
                let upgrade_cost = calculate_upgrade_cost(&inv_item.item);
                let can_upgrade = inv_item.item.num_upgrades < inv_item.item.max_upgrades;

                // Check if player has enough gold and item can be upgraded
                if player.gold >= upgrade_cost && can_upgrade {
                    // Deduct gold first
                    player.gold -= upgrade_cost;

                    // Find the mutable item and upgrade
                    if let Some(inv_item_mut) = player.find_item_by_uuid_mut(item_uuid) {
                        // Upgrade the item
                        if let Ok(result) = inv_item_mut.item.upgrade() {
                            info!("Upgraded {} to level {}", inv_item_mut.item.name, result.new_level);
                        }
                    }
                } else if player.gold < upgrade_cost {
                    info!("Not enough gold to upgrade");
                } else {
                    info!("Item is already at max upgrade level");
                }
            }
        }
        GameAction::Back => {
            blacksmith_state.mode = BlacksmithMode::Menu;
            blacksmith_state.upgrade_selection.reset();
        }
        _ => {}
    }
}

/// Handle input for Quality mode.
fn handle_quality_input(
    blacksmith_state: &mut BlacksmithTabState,
    player: &mut PlayerResource,
    action: &GameAction,
) {
    // Get equipment items and update selection count
    let equipment_count = player
        .inventory
        .items
        .iter()
        .filter(|inv_item| inv_item.item.item_type.is_equipment())
        .count();
    blacksmith_state.quality_selection.set_count(equipment_count);

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_state.quality_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_state.quality_selection.move_down();
        }
        GameAction::Select => {
            // Get equipment items
            let equipment_items: Vec<_> = player
                .inventory
                .items
                .iter()
                .filter(|inv_item| inv_item.item.item_type.is_equipment())
                .collect();

            if let Some(inv_item) = equipment_items.get(blacksmith_state.quality_selection.selected) {
                let item_uuid = inv_item.item.item_uuid;

                // Check if player has QualityUpgradeStone
                if let Some(stone_inv) = player.find_item_by_id(ItemId::QualityUpgradeStone).cloned() {
                    // Find the mutable item to upgrade
                    if let Some(inv_item_mut) = player.find_item_by_uuid_mut(item_uuid) {
                        // Upgrade quality
                        if let Ok(new_quality) = inv_item_mut.item.upgrade_quality() {
                            let item_name = inv_item_mut.item.name.clone();

                            // Consume the stone
                            player.decrease_item_quantity(&stone_inv, 1);
                            info!("Upgraded {} to {:?} quality", item_name, new_quality);
                        } else {
                            info!("Item is already at max quality");
                        }
                    }
                } else {
                    info!("You need a Magic Rock (Quality Upgrade Stone) to improve quality");
                }
            }
        }
        GameAction::Back => {
            blacksmith_state.mode = BlacksmithMode::Menu;
            blacksmith_state.quality_selection.reset();
        }
        _ => {}
    }
}

/// Handle input for Smelt mode.
fn handle_smelt_input(
    blacksmith_state: &mut BlacksmithTabState,
    player: &mut PlayerResource,
    action: &GameAction,
) {
    // Update selection count based on available recipes
    let recipe_count = RecipeId::all_smelting_recipes().len();
    blacksmith_state.smelt_selection.set_count(recipe_count);

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_state.smelt_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_state.smelt_selection.move_down();
        }
        GameAction::Select => 'select: {
            let recipes = RecipeId::all_smelting_recipes();
            let Some(recipe_id) = recipes.get(blacksmith_state.smelt_selection.selected) else {
                break 'select;
            };
            let Ok(recipe) = Recipe::new(*recipe_id) else {
                break 'select;
            };
            if !recipe.can_craft(&player.0) {
                info!("Not enough ingredients to smelt {}", recipe.name());
                break 'select;
            }
            let Ok(output_item_id) = recipe.craft(&mut player.0) else {
                break 'select;
            };
            let new_item = output_item_id.spawn();
            if player.add_to_inv(new_item).is_ok() {
                info!("Smelted {}", recipe.name());
            }
        }
        GameAction::Back => {
            blacksmith_state.mode = BlacksmithMode::Menu;
            blacksmith_state.smelt_selection.reset();
        }
        _ => {}
    }
}

/// Handle input for Forge mode.
fn handle_forge_input(
    blacksmith_state: &mut BlacksmithTabState,
    player: &mut PlayerResource,
    action: &GameAction,
) {
    // Update selection count based on available recipes
    let recipe_count = RecipeId::all_forging_recipes().len();
    blacksmith_state.forge_selection.set_count(recipe_count);

    match action {
        GameAction::Navigate(NavigationDirection::Up) => {
            blacksmith_state.forge_selection.move_up();
        }
        GameAction::Navigate(NavigationDirection::Down) => {
            blacksmith_state.forge_selection.move_down();
        }
        GameAction::Select => 'select: {
            let recipes = RecipeId::all_forging_recipes();
            let Some(recipe_id) = recipes.get(blacksmith_state.forge_selection.selected) else {
                break 'select;
            };
            let Ok(recipe) = Recipe::new(*recipe_id) else {
                break 'select;
            };
            if !recipe.can_craft(&player.0) {
                info!("Not enough ingredients to forge {}", recipe.name());
                break 'select;
            }
            let Ok(output_item_id) = recipe.craft(&mut player.0) else {
                break 'select;
            };
            let new_item = output_item_id.spawn();
            if player.add_to_inv(new_item).is_ok() {
                info!("Forged {}", recipe.name());
            }
        }
        GameAction::Back => {
            blacksmith_state.mode = BlacksmithMode::Menu;
            blacksmith_state.forge_selection.reset();
        }
        _ => {}
    }
}

/// Calculate the upgrade cost for an item.
fn calculate_upgrade_cost(item: &crate::item::Item) -> i32 {
    let base_cost = 100; // Base upgrade cost
    let quality_multiplier = item.quality.upgrade_cost_multiplier();
    (base_cost as f64 * (item.num_upgrades + 1) as f64 * quality_multiplier) as i32
}

/// Render blacksmith content when state changes.
fn render_blacksmith_content(
    mut commands: Commands,
    blacksmith_state: Res<BlacksmithTabState>,
    player: Res<PlayerResource>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
) {
    // Despawn existing tab content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Get content area
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    spawn_blacksmith_ui(&mut commands, content_entity, &blacksmith_state, &player);
}

/// Spawn the blacksmith UI based on current mode.
pub fn spawn_blacksmith_ui(
    commands: &mut Commands,
    content_entity: Entity,
    blacksmith_state: &BlacksmithTabState,
    player: &PlayerResource,
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
                // Player stats summary
                spawn_player_stats(content, player);

                // Render content based on mode
                match blacksmith_state.mode {
                    BlacksmithMode::Menu => {
                        spawn_menu(
                            content,
                            MENU_OPTIONS,
                            blacksmith_state.menu_selection.selected,
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
                    BlacksmithMode::Upgrade => {
                        spawn_upgrade_ui(content, blacksmith_state, player);
                    }
                    BlacksmithMode::Quality => {
                        spawn_quality_ui(content, blacksmith_state, player);
                    }
                    BlacksmithMode::Smelt => {
                        spawn_smelt_ui(content, blacksmith_state, player);
                    }
                    BlacksmithMode::Forge => {
                        spawn_forge_ui(content, blacksmith_state, player);
                    }
                }
            });
    });
}

/// Spawn player stats display.
fn spawn_player_stats(parent: &mut ChildBuilder, player: &PlayerResource) {
    use crate::stats::StatType;
    use crate::entities::Progression;

    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(5.0),
            ..default()
        },))
        .with_children(|stats| {
            // HP - Health stat has both current and max values
            if let Some(health_stat) = player.stats.stat(StatType::Health) {
                stats.spawn((
                    Text::new(format!(
                        "HP: {}/{}",
                        health_stat.current_value, health_stat.max_value
                    )),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.3, 0.3)),
                ));
            }

            // Level & XP
            stats.spawn((
                Text::new(format!(
                    "Level: {}  XP: {}/{}",
                    player.prog.level,
                    player.prog.xp,
                    Progression::xp_to_next_level(player.prog.level)
                )),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.8, 0.5)),
            ));

            // Gold
            stats.spawn((
                Text::new(format!("Gold: {}", player.gold)),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.3)),
            ));
        });
}

/// Spawn the Upgrade mode UI.
fn spawn_upgrade_ui(
    parent: &mut ChildBuilder,
    blacksmith_state: &BlacksmithTabState,
    player: &PlayerResource,
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
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_children(|list| {
                for (i, inv_item) in equipment_items.iter().enumerate() {
                    let is_selected = i == blacksmith_state.upgrade_selection.selected;
                    let upgrade_cost = calculate_upgrade_cost(&inv_item.item);
                    let can_upgrade = inv_item.item.num_upgrades < inv_item.item.max_upgrades;
                    let can_afford = player.gold >= upgrade_cost;

                    let (bg_color, text_color) = if is_selected {
                        (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
                    } else {
                        (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
                    };

                    let prefix = if is_selected { "> " } else { "  " };

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
    blacksmith_state: &BlacksmithTabState,
    player: &PlayerResource,
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
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_children(|list| {
                for (i, inv_item) in equipment_items.iter().enumerate() {
                    let is_selected = i == blacksmith_state.quality_selection.selected;
                    let has_stone = stone_count > 0;
                    let can_upgrade = inv_item.item.quality.next_quality().is_some();

                    let (bg_color, text_color) = if is_selected {
                        (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
                    } else {
                        (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
                    };

                    let prefix = if is_selected { "> " } else { "  " };

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
                        let next_quality_text = if let Some(next) = inv_item.item.quality.next_quality() {
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
    blacksmith_state: &BlacksmithTabState,
    player: &PlayerResource,
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
        parent
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_children(|list| {
                for (i, recipe_id) in recipes.iter().enumerate() {
                    let is_selected = i == blacksmith_state.smelt_selection.selected;

                    if let Ok(recipe) = Recipe::new(*recipe_id) {
                        let can_craft = recipe.can_craft(&player.0);

                        let (bg_color, text_color) = if is_selected {
                            (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
                        } else {
                            (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
                        };

                        let prefix = if is_selected { "> " } else { "  " };

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
    blacksmith_state: &BlacksmithTabState,
    player: &PlayerResource,
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
        parent
            .spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
            ))
            .with_children(|list| {
                for (i, recipe_id) in recipes.iter().enumerate() {
                    let is_selected = i == blacksmith_state.forge_selection.selected;

                    if let Ok(recipe) = Recipe::new(*recipe_id) {
                        let can_craft = recipe.can_craft(&player.0);

                        let (bg_color, text_color) = if is_selected {
                            (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
                        } else {
                            (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
                        };

                        let prefix = if is_selected { "> " } else { "  " };

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
