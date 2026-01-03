#[cfg(test)]
use crate::stats::{HasStats, StatInstance, StatSheet, StatType};

// ==================== StatType enum tests ====================

#[test]
fn stat_type_all_returns_all_stat_types() {
    let all_stats = StatType::all();

    assert_eq!(all_stats.len(), 6);
    assert!(all_stats.contains(&StatType::Health));
    assert!(all_stats.contains(&StatType::Attack));
    assert!(all_stats.contains(&StatType::Defense));
    assert!(all_stats.contains(&StatType::Mining));
    assert!(all_stats.contains(&StatType::GoldFind));
    assert!(all_stats.contains(&StatType::MagicFind));
}

#[test]
fn stat_type_instance_creates_stat_with_correct_type_and_value() {
    let health = StatType::Health.instance(100);
    assert_eq!(health.stat_type, StatType::Health);
    assert_eq!(health.current_value, 100);
    assert_eq!(health.max_value, 100);

    let attack = StatType::Attack.instance(25);
    assert_eq!(attack.stat_type, StatType::Attack);
    assert_eq!(attack.current_value, 25);
    assert_eq!(attack.max_value, 25);
}

#[test]
fn stat_type_instance_works_with_zero_value() {
    let zero_stat = StatType::Defense.instance(0);
    assert_eq!(zero_stat.current_value, 0);
    assert_eq!(zero_stat.max_value, 0);
}

#[test]
fn stat_type_instance_works_with_negative_value() {
    // While unusual, the system allows negative values
    let negative_stat = StatType::Mining.instance(-10);
    assert_eq!(negative_stat.current_value, -10);
    assert_eq!(negative_stat.max_value, -10);
}

// ==================== StatInstance tests ====================

#[test]
fn stat_instance_increase_adds_to_current_value() {
    let mut stat = StatType::Health.instance(50);
    stat.increase(20);
    assert_eq!(stat.current_value, 70);
    assert_eq!(stat.max_value, 50); // max_value unchanged
}

#[test]
fn stat_instance_increase_multiple_times() {
    let mut stat = StatType::Attack.instance(10);
    stat.increase(5);
    stat.increase(3);
    stat.increase(2);
    assert_eq!(stat.current_value, 20);
}

#[test]
fn stat_instance_decrease_subtracts_from_current_value() {
    let mut stat = StatType::Health.instance(100);
    stat.decrease(30);
    assert_eq!(stat.current_value, 70);
    assert_eq!(stat.max_value, 100); // max_value unchanged
}

#[test]
fn stat_instance_decrease_floors_at_zero() {
    let mut stat = StatType::Health.instance(50);
    stat.decrease(100); // Decrease by more than current value
    assert_eq!(stat.current_value, 0);
    assert_eq!(stat.max_value, 50); // max_value unchanged
}

#[test]
fn stat_instance_decrease_to_exactly_zero() {
    let mut stat = StatType::Defense.instance(25);
    stat.decrease(25);
    assert_eq!(stat.current_value, 0);
}

#[test]
fn stat_instance_increase_max_adds_to_max_value() {
    let mut stat = StatType::Health.instance(100);
    stat.increase_max(20);
    assert_eq!(stat.max_value, 120);
    assert_eq!(stat.current_value, 100); // current_value unchanged
}

#[test]
fn stat_instance_decrease_max_subtracts_from_max_value() {
    let mut stat = StatType::Health.instance(100);
    stat.decrease_max(30);
    assert_eq!(stat.max_value, 70);
    assert_eq!(stat.current_value, 100); // current_value unchanged
}

#[test]
fn stat_instance_decrease_max_floors_at_zero() {
    let mut stat = StatType::Health.instance(50);
    stat.decrease_max(100);
    assert_eq!(stat.max_value, 0);
    assert_eq!(stat.current_value, 50); // current_value unchanged
}

// ==================== StatSheet tests ====================

#[test]
fn stat_sheet_new_creates_empty_stat_sheet() {
    let sheet = StatSheet::new();
    assert!(sheet.stats.is_empty());
}

#[test]
fn stat_sheet_with_builder_adds_stats_fluently() {
    let sheet = StatSheet::new()
        .with(StatType::Health, 100)
        .with(StatType::Attack, 20)
        .with(StatType::Defense, 15);

    assert_eq!(sheet.stats.len(), 3);
    assert_eq!(sheet.value(StatType::Health), 100);
    assert_eq!(sheet.value(StatType::Attack), 20);
    assert_eq!(sheet.value(StatType::Defense), 15);
}

#[test]
fn stat_sheet_with_skips_zero_value_stats() {
    let sheet = StatSheet::new()
        .with(StatType::Health, 100)
        .with(StatType::Attack, 0)  // Should be skipped
        .with(StatType::Defense, 10);

    assert_eq!(sheet.stats.len(), 2);
    assert!(sheet.stat(StatType::Health).is_some());
    assert!(sheet.stat(StatType::Attack).is_none());
    assert!(sheet.stat(StatType::Defense).is_some());
}

#[test]
fn stat_sheet_value_returns_stat_value() {
    let sheet = StatSheet::new()
        .with(StatType::Health, 75);

    assert_eq!(sheet.value(StatType::Health), 75);
}

#[test]
fn stat_sheet_value_returns_zero_for_missing_stat() {
    let sheet = StatSheet::new();
    assert_eq!(sheet.value(StatType::Health), 0);
    assert_eq!(sheet.value(StatType::Attack), 0);
}

#[test]
fn stat_sheet_max_value_returns_max_value() {
    let mut sheet = StatSheet::new();
    sheet.insert(StatType::Health.instance(100));

    assert_eq!(sheet.max_value(StatType::Health), 100);
}

#[test]
fn stat_sheet_max_value_returns_zero_for_missing_stat() {
    let sheet = StatSheet::new();
    assert_eq!(sheet.max_value(StatType::Health), 0);
}

#[test]
fn stat_sheet_increase_stat_increases_existing_stat() {
    let mut sheet = StatSheet::new()
        .with(StatType::Attack, 10);

    sheet.increase_stat(StatType::Attack, 5);
    assert_eq!(sheet.value(StatType::Attack), 15);
}

#[test]
fn stat_sheet_increase_stat_does_nothing_for_missing_stat() {
    let mut sheet = StatSheet::new();

    // Attempting to increase a stat that doesn't exist should do nothing
    sheet.increase_stat(StatType::Attack, 10);
    assert_eq!(sheet.value(StatType::Attack), 0);
    assert!(sheet.stat(StatType::Attack).is_none());
}

#[test]
fn stat_sheet_decrease_stat_decreases_stat_value() {
    let mut sheet = StatSheet::new()
        .with(StatType::Health, 100);

    sheet.decrease_stat(StatType::Health, 30);
    assert_eq!(sheet.value(StatType::Health), 70);
}

#[test]
fn stat_sheet_decrease_stat_floors_at_zero() {
    let mut sheet = StatSheet::new()
        .with(StatType::Health, 50);

    sheet.decrease_stat(StatType::Health, 100);
    assert_eq!(sheet.value(StatType::Health), 0);
}

#[test]
fn stat_sheet_decrease_stat_does_nothing_for_missing_stat() {
    let mut sheet = StatSheet::new();

    // Attempting to decrease a stat that doesn't exist should do nothing
    sheet.decrease_stat(StatType::Health, 10);
    assert_eq!(sheet.value(StatType::Health), 0);
}

#[test]
fn stat_sheet_insert_adds_stat_instance() {
    let mut sheet = StatSheet::new();
    sheet.insert(StatType::Mining.instance(25));

    assert_eq!(sheet.value(StatType::Mining), 25);
    assert_eq!(sheet.max_value(StatType::Mining), 25);
}

#[test]
fn stat_sheet_insert_overwrites_existing_stat() {
    let mut sheet = StatSheet::new()
        .with(StatType::Attack, 10);

    sheet.insert(StatType::Attack.instance(50));
    assert_eq!(sheet.value(StatType::Attack), 50);
}

#[test]
fn stat_sheet_increase_stat_max_increases_max_value() {
    let mut sheet = StatSheet::new()
        .with(StatType::Health, 100);

    sheet.increase_stat_max(StatType::Health, 20);
    assert_eq!(sheet.max_value(StatType::Health), 120);
    assert_eq!(sheet.value(StatType::Health), 100); // current unchanged
}

#[test]
fn stat_sheet_decrease_stat_max_decreases_max_value() {
    let mut sheet = StatSheet::new()
        .with(StatType::Health, 100);

    sheet.decrease_stat_max(StatType::Health, 30);
    assert_eq!(sheet.max_value(StatType::Health), 70);
    assert_eq!(sheet.value(StatType::Health), 100); // current unchanged
}

#[test]
fn stat_sheet_stat_returns_some_for_existing_stat() {
    let sheet = StatSheet::new()
        .with(StatType::Defense, 15);

    let stat = sheet.stat(StatType::Defense);
    assert!(stat.is_some());
    assert_eq!(stat.unwrap().current_value, 15);
}

#[test]
fn stat_sheet_stat_returns_none_for_missing_stat() {
    let sheet = StatSheet::new();
    assert!(sheet.stat(StatType::Attack).is_none());
}

#[test]
fn stat_sheet_stat_mut_allows_mutation() {
    let mut sheet = StatSheet::new()
        .with(StatType::Attack, 20);

    if let Some(stat) = sheet.stat_mut(StatType::Attack) {
        stat.increase(10);
    }

    assert_eq!(sheet.value(StatType::Attack), 30);
}

// ==================== HasStats trait tests ====================

#[cfg(test)]
struct TestEntity {
    stats: StatSheet,
}

#[cfg(test)]
impl HasStats for TestEntity {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

#[test]
fn has_stats_hp_returns_health_value() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::Health, 100),
    };

    assert_eq!(entity.hp(), 100);
}

#[test]
fn has_stats_hp_returns_zero_for_missing_health() {
    let entity = TestEntity {
        stats: StatSheet::new(),
    };

    assert_eq!(entity.hp(), 0);
}

#[test]
fn has_stats_max_hp_returns_max_health_value() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::Health, 100),
    };

    assert_eq!(entity.max_hp(), 100);
}

#[test]
fn has_stats_attack_returns_attack_value() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::Attack, 25),
    };

    assert_eq!(entity.attack(), 25);
}

#[test]
fn has_stats_defense_returns_defense_value() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::Defense, 15),
    };

    assert_eq!(entity.defense(), 15);
}

#[test]
fn has_stats_mining_returns_mining_value() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::Mining, 30),
    };

    assert_eq!(entity.mining(), 30);
}

#[test]
fn has_stats_goldfind_returns_goldfind_value() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::GoldFind, 50),
    };

    assert_eq!(entity.goldfind(), 50);
}

#[test]
fn has_stats_magicfind_returns_magicfind_value() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::MagicFind, 40),
    };

    assert_eq!(entity.magicfind(), 40);
}

#[test]
fn has_stats_inc_increases_stat_value() {
    let mut entity = TestEntity {
        stats: StatSheet::new().with(StatType::Attack, 20),
    };

    entity.inc(StatType::Attack, 10);
    assert_eq!(entity.attack(), 30);
}

#[test]
fn has_stats_dec_decreases_stat_value() {
    let mut entity = TestEntity {
        stats: StatSheet::new().with(StatType::Health, 100),
    };

    entity.dec(StatType::Health, 30);
    assert_eq!(entity.hp(), 70);
}

#[test]
fn has_stats_dec_floors_at_zero() {
    let mut entity = TestEntity {
        stats: StatSheet::new().with(StatType::Health, 50),
    };

    entity.dec(StatType::Health, 100);
    assert_eq!(entity.hp(), 0);
}

#[test]
fn has_stats_inc_max_increases_max_value() {
    let mut entity = TestEntity {
        stats: StatSheet::new().with(StatType::Health, 100),
    };

    entity.inc_max(StatType::Health, 20);
    assert_eq!(entity.max_hp(), 120);
    assert_eq!(entity.hp(), 100); // current unchanged
}

#[test]
fn has_stats_dec_max_decreases_max_value() {
    let mut entity = TestEntity {
        stats: StatSheet::new().with(StatType::Health, 100),
    };

    entity.dec_max(StatType::Health, 30);
    assert_eq!(entity.max_hp(), 70);
    assert_eq!(entity.hp(), 100); // current unchanged
}

#[test]
fn has_stats_value_returns_stat_value() {
    let entity = TestEntity {
        stats: StatSheet::new()
            .with(StatType::Health, 100)
            .with(StatType::Attack, 25),
    };

    assert_eq!(entity.value(StatType::Health), 100);
    assert_eq!(entity.value(StatType::Attack), 25);
    assert_eq!(entity.value(StatType::Defense), 0); // Missing stat
}

#[test]
fn has_stats_max_value_returns_max_stat_value() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::Health, 100),
    };

    assert_eq!(entity.max_value(StatType::Health), 100);
    assert_eq!(entity.max_value(StatType::Attack), 0); // Missing stat
}

#[test]
fn has_stats_stat_returns_stat_instance() {
    let entity = TestEntity {
        stats: StatSheet::new().with(StatType::Defense, 15),
    };

    let stat = entity.stat(StatType::Defense);
    assert!(stat.is_some());
    assert_eq!(stat.unwrap().current_value, 15);

    let missing = entity.stat(StatType::Attack);
    assert!(missing.is_none());
}

// ==================== Edge cases and integration tests ====================

#[test]
fn stat_sheet_handles_all_stat_types() {
    let sheet = StatSheet::new()
        .with(StatType::Health, 100)
        .with(StatType::Attack, 25)
        .with(StatType::Defense, 15)
        .with(StatType::Mining, 30)
        .with(StatType::GoldFind, 50)
        .with(StatType::MagicFind, 40);

    assert_eq!(sheet.stats.len(), 6);
    assert_eq!(sheet.value(StatType::Health), 100);
    assert_eq!(sheet.value(StatType::Attack), 25);
    assert_eq!(sheet.value(StatType::Defense), 15);
    assert_eq!(sheet.value(StatType::Mining), 30);
    assert_eq!(sheet.value(StatType::GoldFind), 50);
    assert_eq!(sheet.value(StatType::MagicFind), 40);
}

#[test]
fn stat_instance_current_can_exceed_max_for_non_health_stats() {
    // For stats like Attack, Defense, etc., current can exceed max
    let mut stat = StatType::Attack.instance(20);
    stat.increase(50);

    assert_eq!(stat.current_value, 70);
    assert_eq!(stat.max_value, 20);
    // This is allowed - max_value is only meaningful for Health
}

#[test]
fn complex_stat_manipulation_sequence() {
    let mut entity = TestEntity {
        stats: StatSheet::new().with(StatType::Health, 100),
    };

    // Take damage
    entity.dec(StatType::Health, 30);
    assert_eq!(entity.hp(), 70);

    // Increase max health (e.g., level up)
    entity.inc_max(StatType::Health, 20);
    assert_eq!(entity.max_hp(), 120);
    assert_eq!(entity.hp(), 70);

    // Heal
    entity.inc(StatType::Health, 30);
    assert_eq!(entity.hp(), 100);

    // Take fatal damage
    entity.dec(StatType::Health, 200);
    assert_eq!(entity.hp(), 0);
}

#[test]
fn stat_sheet_builder_pattern_with_mixed_values() {
    let sheet = StatSheet::new()
        .with(StatType::Health, 100)
        .with(StatType::Attack, 0)     // Skipped
        .with(StatType::Defense, 15)
        .with(StatType::Mining, 0)     // Skipped
        .with(StatType::GoldFind, 25);

    assert_eq!(sheet.stats.len(), 3); // Only non-zero stats
    assert_eq!(sheet.value(StatType::Health), 100);
    assert_eq!(sheet.value(StatType::Attack), 0);  // Returns 0 for missing
    assert_eq!(sheet.value(StatType::Defense), 15);
    assert_eq!(sheet.value(StatType::Mining), 0);  // Returns 0 for missing
    assert_eq!(sheet.value(StatType::GoldFind), 25);
}
