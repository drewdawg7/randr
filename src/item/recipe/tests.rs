#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use crate::{
    assets::SpriteSheetKey,
    inventory::{FindsItems, Inventory, ManagesItems},
    item::enums::{ItemQuality, MaterialType},
    item::recipe::{Recipe, RecipeError, RecipeId},
    item::{Item, ItemId, ItemType, SpriteInfo},
    stats::StatSheet,
};

#[cfg(test)]
fn create_test_material(id: ItemId) -> Item {
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
        sprite: SpriteInfo { name: String::new(), sheet_key: SpriteSheetKey::IconItems },
    }
}

#[cfg(test)]
fn add_materials(inventory: &mut Inventory, id: ItemId, quantity: u32) {
    for _ in 0..quantity {
        let item = create_test_material(id);
        inventory.add_to_inv(item).unwrap();
    }
}

#[test]
fn recipe_new_creates_from_valid_id() {
    let recipe = Recipe::new(RecipeId::BasicHPPotion);
    assert!(recipe.is_ok());
}

#[test]
fn recipe_new_creates_for_smelting_recipe() {
    let recipe = Recipe::new(RecipeId::IronIngot);
    assert!(recipe.is_ok());
}

#[test]
fn recipe_new_creates_for_forging_recipe() {
    let recipe = Recipe::new(RecipeId::CopperSword);
    assert!(recipe.is_ok());
}

#[test]
fn recipe_name_returns_recipe_name() {
    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert_eq!(recipe.name(), "Basic HP Potion");
}

#[test]
fn recipe_name_returns_correct_name_for_smelting() {
    let recipe = Recipe::new(RecipeId::CopperIngot).unwrap();
    assert_eq!(recipe.name(), "Copper Ingot");
}

#[test]
fn recipe_ingredients_returns_ingredient_map() {
    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let ingredients = recipe.ingredients();

    assert!(ingredients.contains_key(&ItemId::SlimeGel));
    assert_eq!(ingredients.get(&ItemId::SlimeGel), Some(&10));
}

#[test]
fn recipe_ingredients_returns_multiple_ingredients() {
    let recipe = Recipe::new(RecipeId::CopperIngot).unwrap();
    let ingredients = recipe.ingredients();

    assert!(ingredients.contains_key(&ItemId::IronOre));
    assert!(ingredients.contains_key(&ItemId::GoldOre));
    assert_eq!(ingredients.len(), 2);
}

#[test]
fn recipe_output_item_id_returns_output() {
    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert_eq!(recipe.output_item_id(), ItemId::BasicHPPotion);
}

#[test]
fn recipe_output_item_id_returns_correct_output_for_forging() {
    let recipe = Recipe::new(RecipeId::IronSword).unwrap();
    assert_eq!(recipe.output_item_id(), ItemId::IronSword);
}

#[test]
fn recipe_can_craft_returns_true_with_all_ingredients() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 10);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(recipe.can_craft(&inventory));
}

#[test]
fn recipe_can_craft_returns_true_with_excess_ingredients() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 20);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(recipe.can_craft(&inventory));
}

#[test]
fn recipe_can_craft_returns_false_when_missing_ingredients() {
    let inventory = Inventory::new();

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(!recipe.can_craft(&inventory));
}

#[test]
fn recipe_can_craft_returns_false_with_insufficient_quantity() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 5);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(!recipe.can_craft(&inventory));
}

#[test]
fn recipe_can_craft_returns_false_when_missing_one_of_multiple_ingredients() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::IronOre, 1);

    let recipe = Recipe::new(RecipeId::CopperIngot).unwrap();
    assert!(!recipe.can_craft(&inventory));
}

#[test]
fn recipe_can_craft_returns_true_with_multiple_ingredients() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::IronOre, 1);
    add_materials(&mut inventory, ItemId::GoldOre, 1);

    let recipe = Recipe::new(RecipeId::CopperIngot).unwrap();
    assert!(recipe.can_craft(&inventory));
}

#[test]
fn recipe_craft_returns_not_enough_ingredients_error_when_cannot_craft() {
    let mut inventory = Inventory::new();

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let result = recipe.craft(&mut inventory);

    assert!(matches!(result, Err(RecipeError::NotEnoughIngredients)));
}

#[test]
fn recipe_craft_returns_not_enough_ingredients_when_insufficient() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 5);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let result = recipe.craft(&mut inventory);

    assert!(matches!(result, Err(RecipeError::NotEnoughIngredients)));
}

#[test]
fn recipe_craft_consumes_ingredients() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 15);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let _ = recipe.craft(&mut inventory);

    let remaining = inventory.find_item_by_id(ItemId::SlimeGel);
    assert!(remaining.is_some());
    assert_eq!(remaining.unwrap().quantity, 5);
}

#[test]
fn recipe_craft_consumes_exact_quantity() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 10);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let _ = recipe.craft(&mut inventory);

    let remaining = inventory.find_item_by_id(ItemId::SlimeGel);
    assert!(remaining.is_none());
}

#[test]
fn recipe_craft_returns_output_item_id() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 10);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    let result = recipe.craft(&mut inventory);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ItemId::BasicHPPotion);
}

#[test]
fn recipe_craft_consumes_multiple_ingredients() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::IronOre, 2);
    add_materials(&mut inventory, ItemId::GoldOre, 2);

    let recipe = Recipe::new(RecipeId::CopperIngot).unwrap();
    let result = recipe.craft(&mut inventory);

    assert!(result.is_ok());

    let copper = inventory.find_item_by_id(ItemId::IronOre);
    let tin = inventory.find_item_by_id(ItemId::GoldOre);
    assert!(copper.is_some());
    assert!(tin.is_some());
    assert_eq!(copper.unwrap().quantity, 1);
    assert_eq!(tin.unwrap().quantity, 1);
}

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
    assert!(forging_recipes.contains(&RecipeId::CopperSword));
    assert!(forging_recipes.contains(&RecipeId::IronSword));
}

#[test]
fn recipe_id_material_returns_correct_material() {
    assert_eq!(
        RecipeId::IronSword.material(),
        crate::item::recipe::ForgeMaterial::Iron
    );
    assert_eq!(
        RecipeId::GoldSword.material(),
        crate::item::recipe::ForgeMaterial::Gold
    );
    assert_eq!(
        RecipeId::CopperSword.material(),
        crate::item::recipe::ForgeMaterial::Bronze
    );
}

#[test]
fn recipe_specs_have_valid_outputs() {
    for recipe_id in RecipeId::ALL {
        let recipe = Recipe::new(*recipe_id).unwrap();
        let _output = recipe.output_item_id();
    }
}

#[test]
fn recipe_specs_have_non_empty_ingredients() {
    for recipe_id in RecipeId::ALL {
        let recipe = Recipe::new(*recipe_id).unwrap();
        assert!(
            !recipe.ingredients().is_empty(),
            "Recipe {:?} has no ingredients",
            recipe_id
        );
    }
}

#[test]
fn recipe_specs_have_non_empty_names() {
    for recipe_id in RecipeId::ALL {
        let recipe = Recipe::new(*recipe_id).unwrap();
        assert!(
            !recipe.name().is_empty(),
            "Recipe {:?} has empty name",
            recipe_id
        );
    }
}

#[test]
fn user_flow_crafts_potion_successfully() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 10);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(recipe.can_craft(&inventory));

    let result = recipe.craft(&mut inventory);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ItemId::BasicHPPotion);

    assert!(inventory.find_item_by_id(ItemId::SlimeGel).is_none());
}

#[test]
fn user_flow_cannot_craft_without_ingredients() {
    let inventory = Inventory::new();

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();
    assert!(!recipe.can_craft(&inventory));
}

#[test]
fn user_flow_crafting_consumes_exact_quantities() {
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 20);

    let recipe = Recipe::new(RecipeId::BasicHPPotion).unwrap();

    assert!(recipe.craft(&mut inventory).is_ok());
    assert_eq!(
        inventory.find_item_by_id(ItemId::SlimeGel).unwrap().quantity,
        10
    );

    assert!(recipe.craft(&mut inventory).is_ok());
    assert!(inventory.find_item_by_id(ItemId::SlimeGel).is_none());

    assert!(!recipe.can_craft(&inventory));
}
