#[cfg(test)]
use crate::{
    item::{Item, ItemId, ItemType},
    item::enums::{EquipmentType, ItemQuality, MaterialType},
    stats::{StatType, StatSheet, HasStats},
    location::blacksmith::{Blacksmith, BlacksmithError},
    player::Player,
    combat::HasGold,
};
#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
fn create_test_item(
    name: &str,
    item_type: ItemType,
    attack: i32,
    defense: i32,
    num_upgrades: i32,
    max_upgrades: i32,
    quality: ItemQuality,
) -> Item {
    let base_stats = StatSheet::new()
        .with(StatType::Attack, attack)
        .with(StatType::Defense, defense);
    let stats = quality.multiply_stats(&base_stats);

    Item {
        item_uuid: Uuid::new_v4(),
        item_id: ItemId::Sword,
        item_type,
        name: name.to_string(),
        is_equipped: false,
        is_locked: false,
        num_upgrades,
        max_upgrades,
        max_stack_quantity: 1,
        base_stats,
        stats,
        gold_value: 10,
        quality,
        tome_data: None,
    }
}

#[cfg(test)]
fn create_test_weapon(attack: i32, num_upgrades: i32, max_upgrades: i32) -> Item {
    create_test_item(
        "Test Sword",
        ItemType::Equipment(EquipmentType::Weapon),
        attack,
        0,
        num_upgrades,
        max_upgrades,
        ItemQuality::Normal,
    )
}

#[cfg(test)]
fn create_test_shield(defense: i32, num_upgrades: i32, max_upgrades: i32) -> Item {
    create_test_item(
        "Test Shield",
        ItemType::Equipment(EquipmentType::Shield),
        0,
        defense,
        num_upgrades,
        max_upgrades,
        ItemQuality::Normal,
    )
}

#[cfg(test)]
fn create_material_item() -> Item {
    Item {
        item_uuid: Uuid::new_v4(),
        item_id: ItemId::CopperOre,
        item_type: ItemType::Material(MaterialType::Ore),
        name: "Copper Ore".to_string(),
        is_equipped: false,
        is_locked: false,
        num_upgrades: 0,
        max_upgrades: 0,
        max_stack_quantity: 99,
        base_stats: StatSheet::new(),
        stats: StatSheet::new(),
        gold_value: 5,
        quality: ItemQuality::Normal,
        tome_data: None,
    }
}

// ==================== Blacksmith creation tests ====================

#[test]
fn blacksmith_new_sets_properties() {
    let blacksmith = Blacksmith::new("Test Smith".to_string(), 5, 10);

    assert_eq!(blacksmith.name, "Test Smith");
    assert_eq!(blacksmith.max_upgrades, 5);
    assert_eq!(blacksmith.base_upgrade_cost, 10);
}

// ==================== calc_upgrade_cost tests ====================

#[test]
fn calc_upgrade_cost_first_upgrade() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let item = create_test_weapon(10, 0, 5);

    // First upgrade: (0 + 1) * 10 * 1.0 (normal quality) = 10
    assert_eq!(blacksmith.calc_upgrade_cost(&item), 10);
}

#[test]
fn calc_upgrade_cost_second_upgrade() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let item = create_test_weapon(10, 1, 5);

    // Second upgrade: (1 + 1) * 10 * 1.0 = 20
    assert_eq!(blacksmith.calc_upgrade_cost(&item), 20);
}

#[test]
fn calc_upgrade_cost_increases_linearly() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 10, 5);

    for upgrades in 0..5 {
        let item = create_test_weapon(10, upgrades, 10);
        let expected_cost = (upgrades + 1) * 5;
        assert_eq!(blacksmith.calc_upgrade_cost(&item), expected_cost);
    }
}

#[test]
fn calc_upgrade_cost_with_different_base_costs() {
    let cheap_smith = Blacksmith::new("Cheap".to_string(), 5, 5);
    let expensive_smith = Blacksmith::new("Expensive".to_string(), 5, 50);
    let item = create_test_weapon(10, 2, 5);

    // Third upgrade with base cost 5: (2 + 1) * 5 = 15
    assert_eq!(cheap_smith.calc_upgrade_cost(&item), 15);

    // Third upgrade with base cost 50: (2 + 1) * 50 = 150
    assert_eq!(expensive_smith.calc_upgrade_cost(&item), 150);
}

#[test]
fn calc_upgrade_cost_affected_by_quality() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);

    // Normal quality: multiplier 1.0
    let normal_item = create_test_item(
        "Sword", ItemType::Equipment(EquipmentType::Weapon),
        10, 0, 0, 5, ItemQuality::Normal
    );

    // Improved quality: multiplier 1.1
    let improved_item = create_test_item(
        "Sword", ItemType::Equipment(EquipmentType::Weapon),
        10, 0, 0, 5, ItemQuality::Improved
    );

    // Base cost: (0 + 1) * 10 = 10
    assert_eq!(blacksmith.calc_upgrade_cost(&normal_item), 10);
    // With 1.1 multiplier: 10 * 1.1 = 11
    assert_eq!(blacksmith.calc_upgrade_cost(&improved_item), 11);
}

// ==================== upgrade_item tests ====================

#[test]
fn upgrade_item_success_deducts_gold() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(100);
    let mut item = create_test_weapon(10, 0, 5);

    let result = blacksmith.upgrade_item(&mut player, &mut item);

    assert!(result.is_ok());
    // First upgrade costs 10, so 100 - 10 = 90
    assert_eq!(player.gold(), 90);
}

#[test]
fn upgrade_item_success_increments_num_upgrades() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(100);
    let mut item = create_test_weapon(10, 0, 5);

    assert_eq!(item.num_upgrades, 0);

    let _ = blacksmith.upgrade_item(&mut player, &mut item);

    assert_eq!(item.num_upgrades, 1);
}

#[test]
fn upgrade_item_weapon_increases_attack() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(100);
    let mut item = create_test_weapon(10, 0, 5);

    let initial_attack = item.attack();
    let _ = blacksmith.upgrade_item(&mut player, &mut item);

    // Upgrade multiplies base stat by 1.1, so 10 * 0.1 = 1 increase (min 1)
    assert!(item.attack() > initial_attack);
}

#[test]
fn upgrade_item_shield_increases_defense() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(100);
    let mut item = create_test_shield(10, 0, 5);

    let initial_defense = item.defense();
    let _ = blacksmith.upgrade_item(&mut player, &mut item);

    // Upgrade increases defense
    assert!(item.defense() > initial_defense);
}

#[test]
fn upgrade_item_fails_when_max_upgrades_reached() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 3, 10);
    let mut player = Player::default();
    player.add_gold(100);
    // Item already at max upgrades for this blacksmith
    let mut item = create_test_weapon(10, 3, 5);

    let result = blacksmith.upgrade_item(&mut player, &mut item);

    assert!(matches!(result, Err(BlacksmithError::MaxUpgradesReached)));
    // Gold should not be deducted
    assert_eq!(player.gold(), 100);
}

#[test]
fn upgrade_item_fails_when_not_enough_gold() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(5); // Only 5 gold, but first upgrade costs 10
    let mut item = create_test_weapon(10, 0, 5);

    let result = blacksmith.upgrade_item(&mut player, &mut item);

    assert!(matches!(result, Err(BlacksmithError::NotEnoughGold)));
    // Gold should not be deducted
    assert_eq!(player.gold(), 5);
    // Item should not be upgraded
    assert_eq!(item.num_upgrades, 0);
}

#[test]
fn upgrade_item_fails_with_zero_gold() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    let mut item = create_test_weapon(10, 0, 5);

    let result = blacksmith.upgrade_item(&mut player, &mut item);

    assert!(matches!(result, Err(BlacksmithError::NotEnoughGold)));
}

#[test]
fn upgrade_item_succeeds_with_exact_gold() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(10); // Exactly enough for first upgrade
    let mut item = create_test_weapon(10, 0, 5);

    let result = blacksmith.upgrade_item(&mut player, &mut item);

    assert!(result.is_ok());
    assert_eq!(player.gold(), 0);
    assert_eq!(item.num_upgrades, 1);
}

#[test]
fn upgrade_item_multiple_times() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(1000);
    let mut item = create_test_weapon(10, 0, 5);

    // Upgrade 3 times
    for i in 0..3 {
        let result = blacksmith.upgrade_item(&mut player, &mut item);
        assert!(result.is_ok());
        assert_eq!(item.num_upgrades, i + 1);
    }

    // Total cost: 10 + 20 + 30 = 60
    assert_eq!(player.gold(), 940);
}

#[test]
fn upgrade_item_respects_blacksmith_max_not_item_max() {
    // Blacksmith allows only 2 upgrades, but item allows 5
    let blacksmith = Blacksmith::new("Smith".to_string(), 2, 10);
    let mut player = Player::default();
    player.add_gold(1000);
    let mut item = create_test_weapon(10, 0, 5);

    // First two upgrades should succeed
    assert!(blacksmith.upgrade_item(&mut player, &mut item).is_ok());
    assert!(blacksmith.upgrade_item(&mut player, &mut item).is_ok());

    // Third upgrade should fail due to blacksmith max
    let result = blacksmith.upgrade_item(&mut player, &mut item);
    assert!(matches!(result, Err(BlacksmithError::MaxUpgradesReached)));
    assert_eq!(item.num_upgrades, 2);
}

#[test]
fn upgrade_item_fails_for_non_equipment() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(100);
    let mut item = create_material_item();

    let result = blacksmith.upgrade_item(&mut player, &mut item);

    assert!(matches!(result, Err(BlacksmithError::NotEquipment)));
    assert_eq!(player.gold(), 100);
}

// ==================== Item upgrade tests (direct) ====================

#[test]
fn item_upgrade_respects_item_max_upgrades() {
    let mut item = create_test_weapon(10, 2, 3);

    // Should succeed (upgrade 2 -> 3)
    assert!(item.upgrade().is_ok());
    assert_eq!(item.num_upgrades, 3);

    // Should fail (already at max)
    assert!(item.upgrade().is_err());
    assert_eq!(item.num_upgrades, 3);
}

#[test]
fn item_upgrade_weapon_stats() {
    let mut item = create_test_weapon(10, 0, 5);

    assert!(item.upgrade().is_ok());

    assert_eq!(item.num_upgrades, 1);
    // Attack should increase by at least 1 (10 * 0.1 = 1)
    assert!(item.attack() >= 11);
}

#[test]
fn item_upgrade_shield_stats() {
    let mut item = create_test_shield(10, 0, 5);

    assert!(item.upgrade().is_ok());

    assert_eq!(item.num_upgrades, 1);
    // Defense should increase
    assert!(item.defense() >= 11);
}

// ==================== Quality upgrade tests ====================

#[test]
fn upgrade_quality_requires_upgrade_stone() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    let mut item = create_test_weapon(10, 0, 5);

    let result = blacksmith.upgrade_item_quality(&mut player, &mut item);

    assert!(matches!(result, Err(BlacksmithError::NoUpgradeStones)));
}

#[test]
fn upgrade_quality_fails_for_non_equipment() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    let mut item = create_material_item();

    let result = blacksmith.upgrade_item_quality(&mut player, &mut item);

    assert!(matches!(result, Err(BlacksmithError::NotEquipment)));
}

// ==================== ItemQuality tests ====================

#[test]
fn item_quality_next_quality_progression() {
    assert_eq!(ItemQuality::Poor.next_quality(), Some(ItemQuality::Normal));
    assert_eq!(ItemQuality::Normal.next_quality(), Some(ItemQuality::Improved));
    assert_eq!(ItemQuality::Improved.next_quality(), Some(ItemQuality::WellForged));
    assert_eq!(ItemQuality::WellForged.next_quality(), Some(ItemQuality::Masterworked));
    assert_eq!(ItemQuality::Masterworked.next_quality(), Some(ItemQuality::Mythic));
    assert_eq!(ItemQuality::Mythic.next_quality(), None);
}

#[test]
fn item_quality_multiplier_increases_with_quality() {
    assert!(ItemQuality::Poor.multiplier() < ItemQuality::Normal.multiplier());
    assert!(ItemQuality::Normal.multiplier() < ItemQuality::Improved.multiplier());
    assert!(ItemQuality::Improved.multiplier() < ItemQuality::WellForged.multiplier());
    assert!(ItemQuality::WellForged.multiplier() < ItemQuality::Masterworked.multiplier());
    assert!(ItemQuality::Masterworked.multiplier() < ItemQuality::Mythic.multiplier());
}

#[test]
fn item_quality_multiply_stats_applies_multiplier() {
    let stats = StatSheet::new().with(StatType::Attack, 10);

    let poor_stats = ItemQuality::Poor.multiply_stats(&stats);
    let normal_stats = ItemQuality::Normal.multiply_stats(&stats);
    let mythic_stats = ItemQuality::Mythic.multiply_stats(&stats);

    // Poor (0.8x) < Normal (1.0x) < Mythic (1.8x)
    assert!(poor_stats.value(StatType::Attack) < normal_stats.value(StatType::Attack));
    assert!(normal_stats.value(StatType::Attack) < mythic_stats.value(StatType::Attack));
}

#[test]
fn item_quality_display_names() {
    assert_eq!(ItemQuality::Poor.display_name(), "Poor");
    assert_eq!(ItemQuality::Normal.display_name(), "Normal");
    assert_eq!(ItemQuality::Improved.display_name(), "Improved");
    assert_eq!(ItemQuality::WellForged.display_name(), "Well-Forged");
    assert_eq!(ItemQuality::Masterworked.display_name(), "Masterworked");
    assert_eq!(ItemQuality::Mythic.display_name(), "Mythic");
}
