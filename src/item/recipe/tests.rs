#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use crate::{
    item::{Item, ItemId, ItemType},
    item::enums::{ItemQuality, MaterialType},
    item::recipe::{Recipe, RecipeId, RecipeError},
    player::Player,
    stats::StatSheet,
    inventory::{FindsItems, ManagesItems},
};

// ==================== Test Helpers ====================

#[cfg(test)]
fn create_test_material(id: ItemId, _quantity: u32) -> Item {
    Item {
        item_uuid: Uuid::new_v4(),
        item_id: id,
        item_type: ItemType::Material(MaterialType::Ore),
        name: format!("{:?}", id),
        is_equipped: false,
        is_locked: false,
        num_upgrades: 0,
        max_upgrades: 0,
        max_stack_quantity: 99,
        base_stats: StatSheet::new(),
        stats: StatSheet::new(),
        gold_value: 10,
        quality: ItemQuality::Normal,
    }
}

#[cfg(test)]
fn add_materials_to_player(player: &mut Player, id: ItemId, quantity: u32) {
    for _ in 0..quantity {
        let item = create_test_material(id, 1);
        player.add_to_inv(item).unwrap();
    }
}

// ==================== Recipe::new() tests ====================

#[test]
fn recipe_new_creates_from_valid_id() {
    let recipe = Recipe::new(RecipeId::BasicHPPotion);
    assert!(recipe.is_ok());
}

#[test]
fn recipe_new_creates_for_smelting_recipe() {
    let recipe = Recipe::new(RecipeId::CopperIngot);
    assert!(recipe.is_ok());
}

#[test]
fn recipe_new_creates_for_forging_recipe() {
    let recipe = Recipe::new(RecipeId::BronzeSword);
    assert!(recipe.is_ok());
}

// ==================== Recipe::name() tests ====================

#[test]
fn recipe_name_returns_recipe_name() {
    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert_eq!(recipe.name(), "Basic HP Potion");
}

#[test]
fn recipe_name_returns_correct_name_for_smelting() {
    let recipe = Recipe::new(RecipeId::BronzeIngot).unwrap();
    assert_eq!(recipe.name(), "Bronze Ingot");
}

// ==================== Recipe::ingredients() tests ====================

#[test]
fn recipe_ingredients_returns_ingredient_map() {
    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let ingredients = recipe.ingredients();

    assert!(ingredients.contains_key(&ItemId::SlimeGel));
    assert_eq!(ingredients.get(&ItemId::SlimeGel), Some(&10));
}

#[test]
fn recipe_ingredients_returns_multiple_ingredients() {
    let recipe = Recipe::new(RecipeId::BronzeIngot).unwrap();
    let ingredients = recipe.ingredients();

    // Bronze requires copper ore and tin ore
    assert!(ingredients.contains_key(&ItemId::CopperOre));
    assert!(ingredients.contains_key(&ItemId::TinOre));
    assert_eq!(ingredients.len(), 2);
}

// ==================== Recipe::output_item_id() tests ====================

#[test]
fn recipe_output_item_id_returns_output() {
    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert_eq!(recipe.output_item_id(), ItemId::BasicHPPotion);
}

#[test]
fn recipe_output_item_id_returns_correct_output_for_forging() {
    let recipe = Recipe::new(RecipeId::CopperSword).unwrap();
    assert_eq!(recipe.output_item_id(), ItemId::CopperSword);
}

// ==================== Recipe::can_craft() tests ====================

#[test]
fn recipe_can_craft_returns_true_with_all_ingredients() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 10);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(recipe.can_craft(&player));
}

#[test]
fn recipe_can_craft_returns_true_with_excess_ingredients() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 20);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(recipe.can_craft(&player));
}

#[test]
fn recipe_can_craft_returns_false_when_missing_ingredients() {
    let player = Player::default();

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(!recipe.can_craft(&player));
}

#[test]
fn recipe_can_craft_returns_false_with_insufficient_quantity() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 5); // Need 10

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(!recipe.can_craft(&player));
}

#[test]
fn recipe_can_craft_returns_false_when_missing_one_of_multiple_ingredients() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::CopperOre, 1);
    // Missing TinOre for bronze ingot

    let recipe = Recipe::new(RecipeId::BronzeIngot).unwrap();
    assert!(!recipe.can_craft(&player));
}

#[test]
fn recipe_can_craft_returns_true_with_multiple_ingredients() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::CopperOre, 1);
    add_materials_to_player(&mut player, ItemId::TinOre, 1);

    let recipe = Recipe::new(RecipeId::BronzeIngot).unwrap();
    assert!(recipe.can_craft(&player));
}

// ==================== Recipe::craft() tests ====================

#[test]
fn recipe_craft_returns_not_enough_ingredients_error_when_cannot_craft() {
    let mut player = Player::default();

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let result = recipe.craft(&mut player);

    assert!(matches!(result, Err(RecipeError::NotEnoughIngredients)));
}

#[test]
fn recipe_craft_returns_not_enough_ingredients_when_insufficient() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 5); // Need 10

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let result = recipe.craft(&mut player);

    assert!(matches!(result, Err(RecipeError::NotEnoughIngredients)));
}

#[test]
fn recipe_craft_consumes_ingredients() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 15);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let _ = recipe.craft(&mut player);

    // Should have 5 remaining (15 - 10)
    let remaining = player.find_item_by_id(ItemId::SlimeGel);
    assert!(remaining.is_some());
    assert_eq!(remaining.unwrap().quantity, 5);
}

#[test]
fn recipe_craft_consumes_exact_quantity() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 10);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let _ = recipe.craft(&mut player);

    // Should have no remaining items
    let remaining = player.find_item_by_id(ItemId::SlimeGel);
    assert!(remaining.is_none());
}

#[test]
fn recipe_craft_returns_output_item_id() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 10);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let result = recipe.craft(&mut player);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ItemId::BasicHPPotion);
}

#[test]
fn recipe_craft_consumes_multiple_ingredients() {
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::CopperOre, 2);
    add_materials_to_player(&mut player, ItemId::TinOre, 2);

    let recipe = Recipe::new(RecipeId::BronzeIngot).unwrap();
    let result = recipe.craft(&mut player);

    assert!(result.is_ok());

    // Should have 1 of each remaining
    let copper = player.find_item_by_id(ItemId::CopperOre);
    let tin = player.find_item_by_id(ItemId::TinOre);
    assert!(copper.is_some());
    assert!(tin.is_some());
    assert_eq!(copper.unwrap().quantity, 1);
    assert_eq!(tin.unwrap().quantity, 1);
}

// ==================== RecipeId helper method tests ====================

#[test]
fn recipe_id_all_alchemy_recipes_returns_alchemy_recipes() {
    let alchemy_recipes = RecipeId::all_alchemy_recipes();

    assert!(!alchemy_recipes.is_empty());
    assert!(alchemy_recipes.contains(&RecipeId::BasicHPPotion));
}

#[test]
fn recipe_id_all_forging_recipes_returns_forging_recipes() {
    let forging_recipes = RecipeId::all_forging_recipes();

    assert!(!forging_recipes.is_empty());
    assert!(forging_recipes.contains(&RecipeId::BronzeSword));
    assert!(forging_recipes.contains(&RecipeId::CopperSword));
}

#[test]
fn recipe_id_material_returns_correct_material() {
    assert_eq!(RecipeId::CopperSword.material(), crate::item::recipe::ForgeMaterial::Copper);
    assert_eq!(RecipeId::TinSword.material(), crate::item::recipe::ForgeMaterial::Tin);
    assert_eq!(RecipeId::BronzeSword.material(), crate::item::recipe::ForgeMaterial::Bronze);
}

// ==================== RecipeSpec tests ====================

#[test]
fn recipe_specs_have_valid_outputs() {
    for recipe_id in RecipeId::ALL {
        let recipe = Recipe::new(*recipe_id).unwrap();
        // Output should be a valid ItemId (just accessing it should work)
        let _output = recipe.output_item_id();
    }
}

#[test]
fn recipe_specs_have_non_empty_ingredients() {
    for recipe_id in RecipeId::ALL {
        let recipe = Recipe::new(*recipe_id).unwrap();
        assert!(!recipe.ingredients().is_empty(), "Recipe {:?} has no ingredients", recipe_id);
    }
}

#[test]
fn recipe_specs_have_non_empty_names() {
    for recipe_id in RecipeId::ALL {
        let recipe = Recipe::new(*recipe_id).unwrap();
        assert!(!recipe.name().is_empty(), "Recipe {:?} has empty name", recipe_id);
    }
}

// ==================== User Flow tests ====================

#[test]
fn user_flow_player_crafts_potion_successfully() {
    let mut player = Player::default();

    // Player gathers slime gel from combat
    add_materials_to_player(&mut player, ItemId::SlimeGel, 10);

    // Player can craft the potion
    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(recipe.can_craft(&player));

    // Player crafts the potion
    let result = recipe.craft(&mut player);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ItemId::BasicHPPotion);

    // Ingredients are consumed
    assert!(player.find_item_by_id(ItemId::SlimeGel).is_none());
}

#[test]
fn user_flow_player_cannot_craft_without_ingredients() {
    let player = Player::default();

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();

    // Player cannot craft without ingredients
    assert!(!recipe.can_craft(&player));
}

#[test]
fn user_flow_crafting_consumes_exact_quantities() {
    let mut player = Player::default();

    // Player has exactly enough for 2 crafts
    add_materials_to_player(&mut player, ItemId::SlimeGel, 20);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();

    // First craft
    assert!(recipe.craft(&mut player).is_ok());
    assert_eq!(player.find_item_by_id(ItemId::SlimeGel).unwrap().quantity, 10);

    // Second craft
    assert!(recipe.craft(&mut player).is_ok());
    assert!(player.find_item_by_id(ItemId::SlimeGel).is_none());

    // Third craft fails
    assert!(!recipe.can_craft(&player));
}
