use bevy::prelude::*;

use crate::game::{BrewPotionEvent, Player};
use crate::input::{GameAction, NavigationDirection};
use crate::item::recipe::{Recipe, RecipeId};
use crate::item::ItemId;
use crate::ui::widgets::spawn_player_stats;
use crate::ui::{selection_colors, selection_prefix};

use super::super::shared::{spawn_menu, MenuOption, SelectionState};
use super::super::{ContentArea, TabContent, TownTab};

/// Plugin for the Alchemist tab.
pub struct AlchemistTabPlugin;

impl Plugin for AlchemistTabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AlchemistMode>()
            .init_resource::<AlchemistSelections>()
            .add_systems(OnEnter(TownTab::Alchemist), spawn_alchemist_content)
            .add_systems(
                Update,
                (handle_alchemist_input, refresh_alchemist_on_mode_change)
                    .run_if(in_state(TownTab::Alchemist)),
            );
    }
}

/// Spawns alchemist UI content when entering the Alchemist tab.
fn spawn_alchemist_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    alchemist_mode: Res<AlchemistMode>,
    alchemist_selections: Res<AlchemistSelections>,
    player: Res<Player>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_alchemist_ui(
        &mut commands,
        content_entity,
        &alchemist_mode,
        &alchemist_selections,
        &player,
    );
}

/// Refreshes alchemist UI when mode or selections change.
fn refresh_alchemist_on_mode_change(
    mut commands: Commands,
    alchemist_mode: Res<AlchemistMode>,
    alchemist_selections: Res<AlchemistSelections>,
    content_query: Query<Entity, With<ContentArea>>,
    tab_content_query: Query<Entity, With<TabContent>>,
    player: Res<Player>,
) {
    if !alchemist_mode.is_changed() && !alchemist_selections.is_changed() {
        return;
    }

    // Despawn existing content
    for entity in &tab_content_query {
        commands.entity(entity).despawn_recursive();
    }

    // Respawn with new state
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };
    spawn_alchemist_ui(
        &mut commands,
        content_entity,
        &alchemist_mode,
        &alchemist_selections,
        &player,
    );
}

/// The kind of mode for the Alchemist tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlchemistModeKind {
    #[default]
    Menu,
    Brew,
}

/// Alchemist mode resource.
#[derive(Resource)]
pub struct AlchemistMode {
    pub mode: AlchemistModeKind,
    pub available_recipes: Vec<RecipeId>,
}

impl Default for AlchemistMode {
    fn default() -> Self {
        Self {
            mode: AlchemistModeKind::Menu,
            available_recipes: RecipeId::all_alchemy_recipes(),
        }
    }
}

/// Alchemist selections resource.
#[derive(Resource)]
pub struct AlchemistSelections {
    pub menu: usize,
    pub recipe: SelectionState,
}

impl Default for AlchemistSelections {
    fn default() -> Self {
        let recipe_count = RecipeId::all_alchemy_recipes().len();
        Self {
            menu: 0,
            recipe: SelectionState {
                selected: 0,
                count: recipe_count,
                scroll_offset: 0,
                visible_count: 10,
            },
        }
    }
}

const ALCHEMIST_MENU_OPTIONS: &[MenuOption] = &[MenuOption {
    label: "Brew",
    description: Some("Brew potions from recipes"),
}];

/// Handle input for the Alchemist tab.
fn handle_alchemist_input(
    mut alchemist_mode: ResMut<AlchemistMode>,
    mut alchemist_selections: ResMut<AlchemistSelections>,
    mut action_events: EventReader<GameAction>,
    mut brew_events: EventWriter<BrewPotionEvent>,
) {
    for action in action_events.read() {
        match alchemist_mode.mode {
            AlchemistModeKind::Menu => match action {
                GameAction::Navigate(NavigationDirection::Up) => {
                    if alchemist_selections.menu > 0 {
                        alchemist_selections.menu -= 1;
                    } else {
                        alchemist_selections.menu = ALCHEMIST_MENU_OPTIONS.len() - 1;
                    }
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    alchemist_selections.menu =
                        (alchemist_selections.menu + 1) % ALCHEMIST_MENU_OPTIONS.len();
                }
                GameAction::Select => {
                    // Only one option currently: Brew
                    if alchemist_selections.menu == 0 {
                        alchemist_mode.mode = AlchemistModeKind::Brew;
                        alchemist_selections.recipe.reset();
                    }
                }
                _ => {}
            },
            AlchemistModeKind::Brew => match action {
                GameAction::Navigate(NavigationDirection::Up) => {
                    alchemist_selections.recipe.move_up();
                }
                GameAction::Navigate(NavigationDirection::Down) => {
                    alchemist_selections.recipe.move_down();
                }
                GameAction::Select => {
                    // Emit brewing event - game logic handled by CraftingPlugin
                    if let Some(&recipe_id) = alchemist_mode
                        .available_recipes
                        .get(alchemist_selections.recipe.selected)
                    {
                        brew_events.send(BrewPotionEvent { recipe_id });
                    }
                }
                GameAction::Back => {
                    alchemist_mode.mode = AlchemistModeKind::Menu;
                    alchemist_selections.menu = 0;
                }
                _ => {}
            },
        }
    }
}

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
            .with_children(|content| {
                match alchemist_mode.mode {
                    AlchemistModeKind::Menu => spawn_menu_mode(content, alchemist_selections, player),
                    AlchemistModeKind::Brew => spawn_brew_mode(content, alchemist_mode, alchemist_selections, player),
                }
            });
    });
}

/// Spawn the menu mode UI.
fn spawn_menu_mode(
    content: &mut ChildBuilder,
    alchemist_selections: &AlchemistSelections,
    player: &Player,
) {
    // Player stats summary
    spawn_player_stats(content, player);

    // Menu options
    spawn_menu(
        content,
        ALCHEMIST_MENU_OPTIONS,
        alchemist_selections.menu,
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
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
    player: &Player,
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
            spawn_recipe_list(main_content, alchemist_mode, alchemist_selections, player);

            // Right side: Ingredient details for selected recipe
            spawn_ingredient_details(main_content, alchemist_mode, alchemist_selections, player);
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
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
    player: &Player,
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
            for (i, &recipe_id) in alchemist_mode.available_recipes.iter().enumerate() {
                if let Ok(recipe) = Recipe::new(recipe_id) {
                    let is_selected = i == alchemist_selections.recipe.selected;
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
    player: &Player,
) {
    // Determine if this recipe can be crafted
    let can_craft = recipe.can_craft(&player);
    let status_text = if can_craft { "[READY]" } else { "[MISSING]" };
    let status_color = if can_craft {
        Color::srgb(0.3, 0.8, 0.3)
    } else {
        Color::srgb(0.8, 0.3, 0.3)
    };

    let (bg_color, text_color) = selection_colors(is_selected);

    let prefix = selection_prefix(is_selected);

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
    alchemist_mode: &AlchemistMode,
    alchemist_selections: &AlchemistSelections,
    player: &Player,
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
fn spawn_ingredient_row(
    parent: &mut ChildBuilder,
    item_id: ItemId,
    required: u32,
    player: &Player,
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
