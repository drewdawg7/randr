use bevy::prelude::*;

use crate::entities::progression::HasProgression;
use crate::game::PlayerResource;
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::ManagesItems;
use crate::item::recipe::{Recipe, RecipeId};
use crate::item::ItemId;
use crate::stats::HasStats;
use crate::states::AppState;

use super::super::shared::{spawn_menu, MenuOption, SelectionState};
use super::super::{ContentArea, CurrentTab, TabContent, TownTab};

/// Plugin for the Alchemist tab.
pub struct AlchemistTabPlugin;

impl Plugin for AlchemistTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AlchemistTabState>().add_systems(
            Update,
            (
                handle_alchemist_input,
                render_alchemist_content.run_if(resource_changed::<AlchemistTabState>),
                render_alchemist_on_tab_change.run_if(resource_changed::<CurrentTab>),
            )
                .run_if(in_state(AppState::Town))
                .run_if(|tab: Res<CurrentTab>| tab.tab == TownTab::Alchemist),
        );
    }
}

/// The current mode of the Alchemist tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlchemistMode {
    #[default]
    Menu,
    Brew,
}

/// Alchemist tab state - tracks current mode and menu selection.
#[derive(Resource)]
pub struct AlchemistTabState {
    pub mode: AlchemistMode,
    pub selected_index: usize,
    pub recipe_selection: SelectionState,
    pub available_recipes: Vec<RecipeId>,
}

impl Default for AlchemistTabState {
    fn default() -> Self {
        // Get all alchemy recipes
        let available_recipes = RecipeId::all_alchemy_recipes();
        let recipe_count = available_recipes.len();

        Self {
            mode: AlchemistMode::Menu,
            selected_index: 0,
            recipe_selection: SelectionState {
                selected: 0,
                count: recipe_count,
                scroll_offset: 0,
                visible_count: 10,
            },
            available_recipes,
        }
    }
}

const ALCHEMIST_MENU_OPTIONS: &[MenuOption] = &[MenuOption {
    label: "Brew",
    description: Some("Brew potions from recipes"),
}];

/// Handle input for the Alchemist tab.
fn handle_alchemist_input(
    mut alchemist_state: ResMut<AlchemistTabState>,
    mut player: ResMut<PlayerResource>,
    mut action_events: EventReader<GameAction>,
) {
    for action in action_events.read() {
        match alchemist_state.mode {
            AlchemistMode::Menu => match action {
                GameAction::Navigate(NavigationDirection::Up) => {
                    if alchemist_state.selected_index > 0 {
                        alchemist_state.selected_index -= 1;
                    } else {
                        alchemist_state.selected_index = ALCHEMIST_MENU_OPTIONS.len() - 1;
                    }
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    alchemist_state.selected_index =
                        (alchemist_state.selected_index + 1) % ALCHEMIST_MENU_OPTIONS.len();
                }
                GameAction::Select => {
                    // Only one option currently: Brew
                    if alchemist_state.selected_index == 0 {
                        alchemist_state.mode = AlchemistMode::Brew;
                        alchemist_state.recipe_selection.reset();
                    }
                }
                _ => {}
            },
            AlchemistMode::Brew => match action {
                GameAction::Navigate(NavigationDirection::Up) => {
                    alchemist_state.recipe_selection.move_up();
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    alchemist_state.recipe_selection.move_down();
                }
                GameAction::Select => {
                    // Try to craft the selected recipe
                    if let Some(&recipe_id) = alchemist_state
                        .available_recipes
                        .get(alchemist_state.recipe_selection.selected)
                    {
                        if let Ok(recipe) = Recipe::new(recipe_id) {
                            if recipe.can_craft(&player) {
                                match recipe.craft(&mut player.0) {
                                    Ok(item_id) => {
                                        // Spawn the item and add to inventory
                                        let item = item_id.spawn();
                                        match player.add_to_inv(item) {
                                            Ok(_) => {
                                                info!("Crafted {}!", recipe.name());
                                            }
                                            Err(_) => {
                                                error!("Inventory is full");
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to craft: {:?}", e);
                                    }
                                }
                            } else {
                                info!("Missing ingredients for {}", recipe.name());
                            }
                        }
                    }
                }
                GameAction::Back => {
                    alchemist_state.mode = AlchemistMode::Menu;
                    alchemist_state.selected_index = 0;
                }
                _ => {}
            },
        }
    }
}

/// Render alchemist content when tab is changed to Alchemist.
fn render_alchemist_on_tab_change(
    mut commands: Commands,
    current_tab: Res<CurrentTab>,
    alchemist_state: Res<AlchemistTabState>,
    player: Res<PlayerResource>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
) {
    if current_tab.tab != TownTab::Alchemist {
        return;
    }

    // Despawn existing tab content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Get content area
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    spawn_alchemist_ui(&mut commands, content_entity, &alchemist_state, &player);
}

/// Render alchemist content when state changes.
fn render_alchemist_content(
    mut commands: Commands,
    alchemist_state: Res<AlchemistTabState>,
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

    spawn_alchemist_ui(&mut commands, content_entity, &alchemist_state, &player);
}

/// Spawn the alchemist UI based on current mode.
fn spawn_alchemist_ui(
    commands: &mut Commands,
    content_entity: Entity,
    alchemist_state: &AlchemistTabState,
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
                match alchemist_state.mode {
                    AlchemistMode::Menu => spawn_menu_mode(content, alchemist_state, player),
                    AlchemistMode::Brew => spawn_brew_mode(content, alchemist_state, player),
                }
            });
    });
}

/// Spawn the menu mode UI.
fn spawn_menu_mode(
    content: &mut ChildBuilder,
    alchemist_state: &AlchemistTabState,
    player: &PlayerResource,
) {
    // Player stats summary
    spawn_player_stats(content, player);

    // Menu options
    spawn_menu(
        content,
        ALCHEMIST_MENU_OPTIONS,
        alchemist_state.selected_index,
        Some("Alchemist"),
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

/// Spawn the brew mode UI.
fn spawn_brew_mode(
    content: &mut ChildBuilder,
    alchemist_state: &AlchemistTabState,
    player: &PlayerResource,
) {
    // Player stats summary
    spawn_player_stats(content, player);

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
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                width: Val::Percent(100.0),
                height: Val::Auto,
                ..default()
            },
        ))
        .with_children(|main_content| {
            // Left side: Recipe list
            spawn_recipe_list(main_content, alchemist_state, player);

            // Right side: Ingredient details for selected recipe
            spawn_ingredient_details(main_content, alchemist_state, player);
        });

    // Navigation hint
    content.spawn((
        Text::new("[↑↓] Navigate  [Enter] Brew  [Backspace] Back"),
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

/// Spawn the recipe list panel.
fn spawn_recipe_list(
    parent: &mut ChildBuilder,
    alchemist_state: &AlchemistTabState,
    player: &PlayerResource,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(50.0),
                row_gap: Val::Px(5.0),
                ..default()
            },
        ))
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
            for (i, &recipe_id) in alchemist_state.available_recipes.iter().enumerate() {
                if let Ok(recipe) = Recipe::new(recipe_id) {
                    let is_selected = i == alchemist_state.recipe_selection.selected;
                    spawn_recipe_item(list_container, &recipe, is_selected, player);
                }
            }
        });
}

/// Spawn a single recipe list item.
fn spawn_recipe_item(
    parent: &mut ChildBuilder,
    recipe: &Recipe,
    is_selected: bool,
    player: &PlayerResource,
) {
    // Determine if this recipe can be crafted
    let can_craft = recipe.can_craft(&player);
    let status_text = if can_craft { "[READY]" } else { "[MISSING]" };
    let status_color = if can_craft {
        Color::srgb(0.3, 0.8, 0.3)
    } else {
        Color::srgb(0.8, 0.3, 0.3)
    };

    let (bg_color, text_color) = if is_selected {
        (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
    } else {
        (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
    };

    let prefix = if is_selected { "> " } else { "  " };

    parent
        .spawn((
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
    alchemist_state: &AlchemistTabState,
    player: &PlayerResource,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(50.0),
                row_gap: Val::Px(5.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ))
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
            if let Some(&recipe_id) = alchemist_state
                .available_recipes
                .get(alchemist_state.recipe_selection.selected)
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
fn spawn_ingredient_row(
    parent: &mut ChildBuilder,
    item_id: ItemId,
    required: u32,
    player: &PlayerResource,
) {
    use crate::inventory::FindsItems;

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
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
        ))
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

/// Spawn player stats display.
fn spawn_player_stats(parent: &mut ChildBuilder, player: &PlayerResource) {
    use crate::entities::Progression;

    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            row_gap: Val::Px(5.0),
            ..default()
        },))
        .with_children(|stats| {
            // HP
            stats.spawn((
                Text::new(format!("HP: {}/{}", player.hp(), player.max_hp())),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.3, 0.3)),
            ));

            // Level & XP
            stats.spawn((
                Text::new(format!(
                    "Level: {}  XP: {}/{}",
                    player.level(),
                    player.prog.xp,
                    Progression::xp_to_next_level(player.level())
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
