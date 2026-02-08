use bevy::prelude::*;

use crate::inventory::{Inventory, ManagesItems};
use crate::item::recipe::{Recipe, RecipeId};
use crate::player::PlayerMarker;

#[derive(Message, Debug, Clone)]
pub struct BrewPotionEvent {
    pub recipe_id: RecipeId,
}

#[derive(Message, Debug, Clone)]
pub enum BrewingResult {
    Success { item_name: String },
    InsufficientIngredients { recipe_name: String },
    InventoryFull { item_name: String },
    CraftingFailed { recipe_name: String },
}

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BrewPotionEvent>()
            .add_message::<BrewingResult>()
            .add_systems(Update, handle_brew_potion.run_if(on_message::<BrewPotionEvent>));
    }
}

fn handle_brew_potion(
    mut brew_events: MessageReader<BrewPotionEvent>,
    mut result_events: MessageWriter<BrewingResult>,
    mut player: Query<&mut Inventory, With<PlayerMarker>>,
) {
    let Ok(mut inventory) = player.single_mut() else {
        return;
    };

    for event in brew_events.read() {
        let Ok(recipe) = Recipe::new(event.recipe_id) else {
            continue;
        };

        let recipe_name = recipe.name().to_string();

        if !recipe.can_craft(&*inventory) {
            result_events.write(BrewingResult::InsufficientIngredients { recipe_name });
            continue;
        }

        match recipe.craft(&mut *inventory) {
            Ok(item_id) => {
                let item = item_id.spawn();
                let item_name = recipe.name().to_string();

                match inventory.add_to_inv(item) {
                    Ok(_) => {
                        result_events.write(BrewingResult::Success { item_name });
                    }
                    Err(_) => {
                        result_events.write(BrewingResult::InventoryFull { item_name });
                    }
                }
            }
            Err(_) => {
                result_events.write(BrewingResult::CraftingFailed { recipe_name });
            }
        }
    }
}
