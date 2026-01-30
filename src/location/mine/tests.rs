#[cfg(test)]
use crate::{
    location::mine::{
        definition::Mine,
        rock::{Rock, RockId},
    },
    loot::LootTable,
    stats::{StatSheet, StatType, HasStats},
    combat::IsKillable,
};

// ==================== Helper functions ====================

#[cfg(test)]
fn create_test_rock(rock_id: RockId, health: i32) -> Rock {
    Rock {
        rock_id,
        stats: StatSheet::new().with(StatType::Health, health),
        loot: LootTable::new().build(),
    }
}

// ==================== Mine creation tests ====================

#[test]
fn mine_new_sets_name() {
    let mine = Mine::new("Test Mine".to_string());

    assert_eq!(mine.name, "Test Mine");
}

#[test]
fn mine_new_has_no_current_rock() {
    let mine = Mine::new("Test Mine".to_string());

    assert!(mine.current_rock.is_none());
}

#[test]
fn mine_new_sets_rock_weights() {
    let mine = Mine::new("Test Mine".to_string());

    assert!(mine.rock_weights.contains_key(&RockId::Iron));
    assert!(mine.rock_weights.contains_key(&RockId::Coal));
    assert!(mine.rock_weights.contains_key(&RockId::Gold));
}

#[test]
fn mine_new_iron_has_highest_weight() {
    let mine = Mine::new("Test Mine".to_string());

    let iron_weight = mine.rock_weights.get(&RockId::Iron).unwrap();
    let coal_weight = mine.rock_weights.get(&RockId::Coal).unwrap();
    let gold_weight = mine.rock_weights.get(&RockId::Gold).unwrap();

    assert!(iron_weight > coal_weight);
    assert!(iron_weight > gold_weight);
}

// ==================== Mine default tests ====================

#[test]
fn mine_default_has_expected_name() {
    let mine = Mine::default();

    assert_eq!(mine.name, "Village Mine");
}

#[test]
fn mine_default_has_no_current_rock() {
    let mine = Mine::default();

    assert!(mine.current_rock.is_none());
}

#[test]
fn mine_default_has_all_rock_types() {
    let mine = Mine::default();

    assert!(mine.rock_weights.contains_key(&RockId::Gold));
    assert!(mine.rock_weights.contains_key(&RockId::Coal));
    assert!(mine.rock_weights.contains_key(&RockId::Iron));
    assert!(mine.rock_weights.contains_key(&RockId::Mixed));
}

// ==================== Mine methods tests ====================

#[test]
fn mine_current_rock_mut_returns_none_when_no_rock() {
    let mut mine = Mine::default();

    assert!(mine.current_rock_mut().is_none());
}

#[test]
fn mine_current_rock_mut_returns_some_when_rock_exists() {
    let mut mine = Mine::default();
    mine.current_rock = Some(create_test_rock(RockId::Iron, 50));

    assert!(mine.current_rock_mut().is_some());
}

// ==================== Rock creation tests ====================

#[test]
fn rock_has_correct_rock_id() {
    let rock = create_test_rock(RockId::Coal, 50);

    assert_eq!(rock.rock_id, RockId::Coal);
}

#[test]
fn rock_has_correct_health() {
    let rock = create_test_rock(RockId::Iron, 100);

    assert_eq!(rock.hp(), 100);
}

// ==================== Rock IsKillable trait tests ====================

#[test]
fn rock_take_damage_reduces_health() {
    let mut rock = create_test_rock(RockId::Iron, 50);

    rock.take_damage(10);

    assert_eq!(rock.hp(), 40);
}

#[test]
fn rock_take_damage_multiple_times() {
    let mut rock = create_test_rock(RockId::Iron, 50);

    rock.take_damage(10);
    rock.take_damage(15);
    rock.take_damage(5);

    assert_eq!(rock.hp(), 20);
}

#[test]
fn rock_take_damage_does_not_go_negative() {
    let mut rock = create_test_rock(RockId::Iron, 50);

    rock.take_damage(100);

    assert_eq!(rock.hp(), 0);
}

#[test]
fn rock_is_alive_when_health_positive() {
    let rock = create_test_rock(RockId::Iron, 50);

    assert!(rock.is_alive());
}

#[test]
fn rock_is_not_alive_when_health_zero() {
    let mut rock = create_test_rock(RockId::Iron, 50);

    rock.take_damage(50);

    assert!(!rock.is_alive());
}

#[test]
fn rock_health_returns_current_hp() {
    let rock = create_test_rock(RockId::Gold, 75);

    assert_eq!(rock.health(), 75);
}

// ==================== Rock HasStats trait tests ====================

#[test]
fn rock_stats_returns_stat_sheet() {
    let rock = create_test_rock(RockId::Iron, 50);

    assert_eq!(rock.stats().value(StatType::Health), 50);
}

#[test]
fn rock_stats_mut_allows_modification() {
    let mut rock = create_test_rock(RockId::Iron, 50);

    rock.stats_mut().increase_stat(StatType::Health, 10);

    assert_eq!(rock.hp(), 60);
}

// ==================== RockId tests ====================

#[test]
fn rock_id_equality() {
    assert_eq!(RockId::Iron, RockId::Iron);
    assert_eq!(RockId::Coal, RockId::Coal);
    assert_eq!(RockId::Gold, RockId::Gold);
    assert_eq!(RockId::Mixed, RockId::Mixed);
}

#[test]
fn rock_id_inequality() {
    assert_ne!(RockId::Iron, RockId::Coal);
    assert_ne!(RockId::Gold, RockId::Mixed);
}

#[test]
fn rock_id_is_copy() {
    let id = RockId::Iron;
    let id_copy = id;

    assert_eq!(id, id_copy);
}
