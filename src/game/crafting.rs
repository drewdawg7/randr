use bevy::prelude::*;

use crate::entities::Progression;
use crate::inventory::{Inventory, ManagesItems};
use crate::item::recipe::{Recipe, RecipeId};
use crate::player::{Player, PlayerGold, PlayerName};
use crate::stats::StatSheet;

/// Event sent when player attempts to brew a potion.
#[derive(Event, Debug, Clone)]
pub struct BrewPotionEvent {
    pub recipe_id: RecipeId,
}

/// Result event for brewing operations.
#[derive(Event, Debug, Clone)]
pub enum BrewingResult {
    Success { item_name: String },
    InsufficientIngredients { recipe_name: String },
    InventoryFull { item_name: String },
    CraftingFailed { recipe_name: String },
}

/// Plugin for crafting-related events and systems.
pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BrewPotionEvent>()
            .add_event::<BrewingResult>()
            .add_systems(Update, handle_brew_potion);
    }
}

/// Handle brew potion events by executing the crafting logic.
fn handle_brew_potion(
    mut brew_events: EventReader<BrewPotionEvent>,
    mut result_events: EventWriter<BrewingResult>,
    name: Res<PlayerName>,
    mut gold: ResMut<PlayerGold>,
    mut progression: ResMut<Progression>,
    mut inventory: ResMut<Inventory>,
    mut stats: ResMut<StatSheet>,
) {
    for event in brew_events.read() {
        let Ok(recipe) = Recipe::new(event.recipe_id) else {
            continue;
        };

        let recipe_name = recipe.name().to_string();

        // Build Player view for Recipe API
        let mut player = Player::from_resources(&name, &gold, &progression, &inventory, &stats);

        // Check ingredients
        if !recipe.can_craft(&player) {
            result_events.send(BrewingResult::InsufficientIngredients { recipe_name });
            continue;
        }

        // Craft the potion (consumes ingredients)
        match recipe.craft(&mut player) {
            Ok(item_id) => {
                // Spawn the item and add to inventory
                let item = item_id.spawn();
                let item_name = recipe.name().to_string();

                match player.add_to_inv(item) {
                    Ok(_) => {
                        // Write changes back to resources
                        player.write_back(&mut gold, &mut progression, &mut inventory, &mut stats);
                        result_events.send(BrewingResult::Success { item_name });
                    }
                    Err(_) => {
                        result_events.send(BrewingResult::InventoryFull { item_name });
                    }
                }
            }
            Err(_) => {
                result_events.send(BrewingResult::CraftingFailed { recipe_name });
            }
        }
    }
}
