#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use crate::{
    item::{Item, ItemId, ItemType},
    item::enums::{ItemQuality, MaterialType},
    item::recipe::RecipeId,
    location::{
        alchemist::{Alchemist, enums::AlchemistError},
        AlchemistData, LocationData, LocationId, LocationSpec,
    },
    player::Player,
    stats::StatSheet,
    inventory::{FindsItems, ManagesItems},
};

// ==================== Test Helpers ====================

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
    }
}

#[cfg(test)]
fn add_materials_to_player(player: &mut Player, id: ItemId, quantity: u32) {
    for _ in 0..quantity {
        let item = create_test_material(id);
        player.add_to_inv(item).unwrap();
    }
}

#[cfg(test)]
fn fill_remaining_inventory(player: &mut Player) {
    // Fill remaining inventory slots with non-stackable items
    // Keep adding until inventory is full
    for _ in 0..15 {
        let item = Item {
            item_uuid: Uuid::new_v4(),
            item_id: ItemId::Sword,
            item_type: ItemType::Equipment(crate::item::enums::EquipmentType::Weapon),
            name: "Test Sword".to_string(),
            is_equipped: false,
            is_locked: false,
            num_upgrades: 0,
            max_upgrades: 5,
            max_stack_quantity: 1,
            base_stats: StatSheet::new(),
            stats: StatSheet::new(),
            gold_value: 100,
            quality: ItemQuality::Normal,
        };
        // Ignore errors - we just want to fill what we can
        let _ = player.add_to_inv(item);
    }
}

// ==================== Alchemist::new() tests ====================

#[test]
fn alchemist_new_creates_with_name() {
    let alchemist = Alchemist::new("Test Alchemist".to_string());

    assert_eq!(alchemist.name, "Test Alchemist");
}

#[test]
fn alchemist_new_sets_default_location_id() {
    let alchemist = Alchemist::new("Test".to_string());

    assert_eq!(alchemist.location_id(), LocationId::VillageAlchemist);
}

#[test]
fn alchemist_new_sets_empty_description() {
    let alchemist = Alchemist::new("Test".to_string());

    assert!(alchemist.description().is_empty());
}

// ==================== Alchemist::from_spec() tests ====================

#[test]
fn alchemist_from_spec_creates_from_location_spec() {
    let spec = LocationSpec {
        name: "Village Alchemist",
        description: "A mysterious shop filled with potions.",
        refresh_interval: None,
        min_level: None,
        data: LocationData::Alchemist(AlchemistData {}),
    };
    let data = AlchemistData {};

    let alchemist = Alchemist::from_spec(LocationId::VillageAlchemist, &spec, &data);

    assert_eq!(alchemist.name, "Village Alchemist");
    assert_eq!(alchemist.description(), "A mysterious shop filled with potions.");
    assert_eq!(alchemist.location_id(), LocationId::VillageAlchemist);
}

// ==================== Alchemist::brew_potion() tests ====================

#[test]
fn alchemist_brew_potion_crafts_recipe_successfully() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 10);

    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    assert!(result.is_ok());
}

#[test]
fn alchemist_brew_potion_returns_crafted_item() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 10);

    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    assert!(result.is_ok());
    let item = result.unwrap();
    assert_eq!(item.item_id, ItemId::BasicHPPotion);
}

#[test]
fn alchemist_brew_potion_adds_to_player_inventory() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 10);

    let _ = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    // Player should now have the potion
    let potion = player.find_item_by_id(ItemId::BasicHPPotion);
    assert!(potion.is_some());
}

#[test]
fn alchemist_brew_potion_consumes_ingredients() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 15);

    let _ = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    // Should have 5 slime gel remaining
    let remaining = player.find_item_by_id(ItemId::SlimeGel);
    assert!(remaining.is_some());
    assert_eq!(remaining.unwrap().quantity, 5);
}

#[test]
fn alchemist_brew_potion_returns_recipe_error_for_missing_ingredients() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut player = Player::default();
    // No ingredients

    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    assert!(matches!(result, Err(AlchemistError::RecipeError(_))));
}

#[test]
fn alchemist_brew_potion_returns_recipe_error_for_insufficient_ingredients() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut player = Player::default();
    add_materials_to_player(&mut player, ItemId::SlimeGel, 5); // Need 10

    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    assert!(matches!(result, Err(AlchemistError::RecipeError(_))));
}

#[test]
fn alchemist_brew_potion_returns_inventory_full_when_player_inventory_full() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut player = Player::default();
    // Add MORE than needed so the slot isn't freed after crafting
    add_materials_to_player(&mut player, ItemId::SlimeGel, 20);
    // Fill remaining 14 slots
    fill_remaining_inventory(&mut player);

    // After crafting:
    // - 10 slime gel consumed, but 10 remain (slot still occupied)
    // - 14 sword slots still occupied
    // - Total: 15 slots full
    // - Potion can't be added -> InventoryFull
    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    assert!(matches!(result, Err(AlchemistError::InventoryFull)));
}

// ==================== AlchemistError tests ====================

#[test]
fn alchemist_error_recipe_error_wraps_recipe_error() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut player = Player::default();

    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    if let Err(AlchemistError::RecipeError(recipe_err)) = result {
        // Check we got a recipe error wrapped
        assert!(matches!(recipe_err, crate::item::recipe::RecipeError::NotEnoughIngredients));
    } else {
        panic!("Expected RecipeError");
    }
}

// ==================== User Flow tests ====================

#[test]
fn user_flow_player_with_ingredients_brews_potion_successfully() {
    let alchemist = Alchemist::new("Village Alchemist".to_string());
    let mut player = Player::default();

    // Player gathers slime gel from combat
    add_materials_to_player(&mut player, ItemId::SlimeGel, 10);

    // Player visits alchemist and brews a potion
    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);
    assert!(result.is_ok());

    // Player now has the potion
    let potion = player.find_item_by_id(ItemId::BasicHPPotion);
    assert!(potion.is_some());

    // Ingredients are consumed
    assert!(player.find_item_by_id(ItemId::SlimeGel).is_none());
}

#[test]
fn user_flow_player_without_ingredients_cannot_brew() {
    let alchemist = Alchemist::new("Village Alchemist".to_string());
    let mut player = Player::default();

    // Player has no ingredients
    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);

    assert!(result.is_err());
    // Player still has no potion
    assert!(player.find_item_by_id(ItemId::BasicHPPotion).is_none());
}

#[test]
fn user_flow_brewing_consumes_exact_ingredient_quantities() {
    let alchemist = Alchemist::new("Village Alchemist".to_string());
    let mut player = Player::default();

    // Player has enough for exactly 2 potions
    add_materials_to_player(&mut player, ItemId::SlimeGel, 20);

    // First brew
    assert!(alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion).is_ok());
    assert_eq!(player.find_item_by_id(ItemId::SlimeGel).unwrap().quantity, 10);

    // Second brew
    assert!(alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion).is_ok());
    assert!(player.find_item_by_id(ItemId::SlimeGel).is_none());

    // Third brew fails
    let result = alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion);
    assert!(matches!(result, Err(AlchemistError::RecipeError(_))));
}

#[test]
fn user_flow_multiple_potions_stack_in_inventory() {
    let alchemist = Alchemist::new("Village Alchemist".to_string());
    let mut player = Player::default();

    // Player has enough for 3 potions
    add_materials_to_player(&mut player, ItemId::SlimeGel, 30);

    // Brew 3 potions
    for _ in 0..3 {
        alchemist.brew_potion(&mut player, &RecipeId::BasicHPPotion).unwrap();
    }

    // Potions should stack
    let potions = player.find_item_by_id(ItemId::BasicHPPotion);
    assert!(potions.is_some());
    assert_eq!(potions.unwrap().quantity, 3);
}
