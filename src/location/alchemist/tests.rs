#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
use crate::{
    inventory::{FindsItems, Inventory, ManagesItems},
    item::enums::{ItemQuality, MaterialType},
    item::recipe::RecipeId,
    item::{Item, ItemId, ItemType},
    location::{
        alchemist::{enums::AlchemistError, Alchemist},
        AlchemistData, LocationData, LocationId, LocationSpec,
    },
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
    }
}

#[cfg(test)]
fn add_materials(inventory: &mut Inventory, id: ItemId, quantity: u32) {
    for _ in 0..quantity {
        let item = create_test_material(id);
        inventory.add_to_inv(item).unwrap();
    }
}

#[cfg(test)]
fn fill_remaining_inventory(inventory: &mut Inventory) {
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
        let _ = inventory.add_to_inv(item);
    }
}

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
    assert_eq!(
        alchemist.description(),
        "A mysterious shop filled with potions."
    );
    assert_eq!(alchemist.location_id(), LocationId::VillageAlchemist);
}

#[test]
fn alchemist_brew_potion_crafts_recipe_successfully() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 10);

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    assert!(result.is_ok());
}

#[test]
fn alchemist_brew_potion_returns_crafted_item() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 10);

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    assert!(result.is_ok());
    let item = result.unwrap();
    assert_eq!(item.item_id, ItemId::BasicHPPotion);
}

#[test]
fn alchemist_brew_potion_adds_to_inventory() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 10);

    let _ = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    let potion = inventory.find_item_by_id(ItemId::BasicHPPotion);
    assert!(potion.is_some());
}

#[test]
fn alchemist_brew_potion_consumes_ingredients() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 15);

    let _ = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    let remaining = inventory.find_item_by_id(ItemId::SlimeGel);
    assert!(remaining.is_some());
    assert_eq!(remaining.unwrap().quantity, 5);
}

#[test]
fn alchemist_brew_potion_returns_recipe_error_for_missing_ingredients() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut inventory = Inventory::new();

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    assert!(matches!(result, Err(AlchemistError::RecipeError(_))));
}

#[test]
fn alchemist_brew_potion_returns_recipe_error_for_insufficient_ingredients() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 5);

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    assert!(matches!(result, Err(AlchemistError::RecipeError(_))));
}

#[test]
fn alchemist_brew_potion_returns_inventory_full_when_inventory_full() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut inventory = Inventory::new();
    add_materials(&mut inventory, ItemId::SlimeGel, 20);
    fill_remaining_inventory(&mut inventory);

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    assert!(matches!(result, Err(AlchemistError::InventoryFull)));
}

#[test]
fn alchemist_error_recipe_error_wraps_recipe_error() {
    let alchemist = Alchemist::new("Test".to_string());
    let mut inventory = Inventory::new();

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    if let Err(AlchemistError::RecipeError(recipe_err)) = result {
        assert!(matches!(
            recipe_err,
            crate::item::recipe::RecipeError::NotEnoughIngredients
        ));
    } else {
        panic!("Expected RecipeError");
    }
}

#[test]
fn user_flow_with_ingredients_brews_potion_successfully() {
    let alchemist = Alchemist::new("Village Alchemist".to_string());
    let mut inventory = Inventory::new();

    add_materials(&mut inventory, ItemId::SlimeGel, 10);

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);
    assert!(result.is_ok());

    let potion = inventory.find_item_by_id(ItemId::BasicHPPotion);
    assert!(potion.is_some());

    assert!(inventory.find_item_by_id(ItemId::SlimeGel).is_none());
}

#[test]
fn user_flow_without_ingredients_cannot_brew() {
    let alchemist = Alchemist::new("Village Alchemist".to_string());
    let mut inventory = Inventory::new();

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);

    assert!(result.is_err());
    assert!(inventory.find_item_by_id(ItemId::BasicHPPotion).is_none());
}

#[test]
fn user_flow_brewing_consumes_exact_ingredient_quantities() {
    let alchemist = Alchemist::new("Village Alchemist".to_string());
    let mut inventory = Inventory::new();

    add_materials(&mut inventory, ItemId::SlimeGel, 20);

    assert!(alchemist
        .brew_potion(&mut inventory, &RecipeId::BasicHPPotion)
        .is_ok());
    assert_eq!(
        inventory.find_item_by_id(ItemId::SlimeGel).unwrap().quantity,
        10
    );

    assert!(alchemist
        .brew_potion(&mut inventory, &RecipeId::BasicHPPotion)
        .is_ok());
    assert!(inventory.find_item_by_id(ItemId::SlimeGel).is_none());

    let result = alchemist.brew_potion(&mut inventory, &RecipeId::BasicHPPotion);
    assert!(matches!(result, Err(AlchemistError::RecipeError(_))));
}

#[test]
fn user_flow_multiple_potions_stack_in_inventory() {
    let alchemist = Alchemist::new("Village Alchemist".to_string());
    let mut inventory = Inventory::new();

    add_materials(&mut inventory, ItemId::SlimeGel, 30);

    for _ in 0..3 {
        alchemist
            .brew_potion(&mut inventory, &RecipeId::BasicHPPotion)
            .unwrap();
    }

    let potions = inventory.find_item_by_id(ItemId::BasicHPPotion);
    assert!(potions.is_some());
    assert_eq!(potions.unwrap().quantity, 3);
}
