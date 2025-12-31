#[cfg(test)]
use crate::{
    item::{Item, ItemId, ItemType},
    stats::{StatType, StatInstance, StatSheet, HasStats},
    blacksmith::{Blacksmith, BlacksmithError},
    entities::{
        player::Player
    },
    combat::{
        traits::{HasGold}
    }
};
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use uuid::Uuid;

#[cfg(test)]
fn create_test_item(
    name: &'static str,
    item_type: ItemType,
    attack: i32,
    defense: i32,
    num_upgrades: i32,
    max_upgrades: i32,
) -> Item {



    let mut stats: HashMap<StatType, StatInstance> = HashMap::new();
    stats.insert(StatType::Attack, StatType::Attack.instance(attack));
    stats.insert(StatType::Defense, StatType::Defense.instance(defense));

    Item {
        item_uuid: Uuid::new_v4(),
        kind: ItemId::Sword,
        item_type,
        name,
        is_equipped: false,
        num_upgrades,
        max_upgrades,
        max_stack_quantity: 1,
        stats: StatSheet { stats },
    }
}
#[cfg(test)]
fn create_test_weapon(attack: i32, num_upgrades: i32, max_upgrades: i32) -> Item {
    create_test_item("Test Sword", ItemType::Weapon, attack, 0, num_upgrades, max_upgrades)
}
#[cfg(test)]
fn create_test_shield(defense: i32, num_upgrades: i32, max_upgrades: i32) -> Item {
    create_test_item("Test Shield", ItemType::Shield, 0, defense, num_upgrades, max_upgrades)
}

// ==================== Blacksmith creation tests ====================

#[test]
fn blacksmith_new_sets_properties() {
    let blacksmith = Blacksmith::new("Test Smith".to_string(), 5, 10);

    assert_eq!(blacksmith.name, "Test Smith");
    assert_eq!(blacksmith.max_upgrades, 5);
    assert_eq!(blacksmith.base_upgrade_cost, 10);
}

#[test]
fn blacksmith_default_has_expected_values() {
    let blacksmith = Blacksmith::default();

    assert_eq!(blacksmith.name, "Blacksmith");
    assert_eq!(blacksmith.max_upgrades, 4);
    assert_eq!(blacksmith.base_upgrade_cost, 5);
}

// ==================== calc_upgrade_cost tests ====================

#[test]
fn calc_upgrade_cost_first_upgrade() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let item = create_test_weapon(10, 0, 5);

    // First upgrade: (0 + 1) * 10 = 10
    assert_eq!(blacksmith.calc_upgrade_cost(&item), 10);
}

#[test]
fn calc_upgrade_cost_second_upgrade() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let item = create_test_weapon(10, 1, 5);

    // Second upgrade: (1 + 1) * 10 = 20
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

    // Weapons gain +3 attack per upgrade
    assert_eq!(item.attack(), initial_attack + 3);
}

#[test]
fn upgrade_item_shield_increases_defense() {
    let blacksmith = Blacksmith::new("Smith".to_string(), 5, 10);
    let mut player = Player::default();
    player.add_gold(100);
    let mut item = create_test_shield(5, 0, 5);

    let initial_defense = item.def();
    let _ = blacksmith.upgrade_item(&mut player, &mut item);

    // Shields gain +1 defense per upgrade
    assert_eq!(item.def(), initial_defense + 1);
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
    // Attack increased by 3 * 3 = 9
    assert_eq!(item.attack(), 19);
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
    assert_eq!(item.attack(), 13); // 10 + 3
}

#[test]
fn item_upgrade_shield_stats() {
    let mut item = create_test_shield(5, 0, 5);

    assert!(item.upgrade().is_ok());

    assert_eq!(item.num_upgrades, 1);
    assert_eq!(item.def(), 6); // 5 + 1
}
