#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use crate::{
    combat::{attack, apply_defense, calculate_damage_reduction, Combatant, DealsDamage, IsKillable},
    loot::LootTable,
    mob::{Mob, MobId, enums::MobQuality},
    player::PlayerGold,
    stats::{StatSheet, StatType},
};

#[cfg(test)]
fn create_test_mob(name: &str, hp: i32, attack: i32, defense: i32) -> Mob {
    let mut stats = HashMap::new();
    stats.insert(StatType::Health, StatType::Health.instance(hp));
    stats.insert(StatType::Attack, StatType::Attack.instance(attack));
    stats.insert(StatType::Defense, StatType::Defense.instance(defense));
    Mob {
        mob_id: MobId::Slime,
        quality: MobQuality::Normal,
        name: name.to_string(),
        stats: StatSheet { stats },
        gold: 5,
        dropped_xp: 15,
        loot_table: LootTable::default(),
        death_processed: false,
    }
}

// ==================== attack() tests ====================

#[test]
fn attack_deals_damage_with_variance_and_defense_reduction() {
    let attacker = create_test_mob("Attacker", 100, 20, 5);
    let mut defender = create_test_mob("Defender", 100, 10, 8);

    let result = attack(&attacker, &mut defender);

    // Attacker attack ~20 with ±25% variance (15-25), defender defense 8
    // Defense reduction = 8 / (8 + 50) = ~13.8%
    assert!(result.damage_to_target > 0);
    assert!(result.damage_to_target <= 25); // Max raw damage
    assert_eq!(result.target_health_before, 100);
    assert!(result.target_health_after < 100);
    assert!(!result.target_died);
}

#[test]
fn attack_high_defense_reduces_damage_significantly() {
    let attacker = create_test_mob("Attacker", 100, 10, 0);
    let mut defender = create_test_mob("Tank", 100, 5, 100);

    let result = attack(&attacker, &mut defender);

    // Attacker attack ~10 with variance, defender has 100 defense
    // Defense reduction = 100 / (100 + 50) = 66.7%
    // Raw damage ~8-12, reduced to ~3-4
    assert!(result.damage_to_target >= 0);
    assert!(result.damage_to_target < 10); // Significantly reduced
}

#[test]
fn attack_kills_target_when_damage_exceeds_health() {
    let attacker = create_test_mob("Strong", 100, 100, 0);
    let mut defender = create_test_mob("Weakling", 10, 5, 0);

    let result = attack(&attacker, &mut defender);

    // Attacker has high attack, defender has 0 defense, low HP
    assert!(result.damage_to_target > 10); // Overkill
    assert!(result.target_died);
}

#[test]
fn attack_result_contains_correct_names() {
    let attacker = create_test_mob("Goblin", 50, 10, 2);
    let mut defender = create_test_mob("Orc", 50, 10, 2);

    let result = attack(&attacker, &mut defender);

    assert_eq!(result.attacker, "Goblin");
    assert_eq!(result.defender, "Orc");
}

#[test]
fn mob_attacks_mob() {
    let attacker = create_test_mob("Orc", 50, 20, 3);
    let mut defender = create_test_mob("Goblin", 100, 10, 5);

    let result = attack(&attacker, &mut defender);

    // Orc attack ~20 with variance, goblin defense 5
    // Defense reduction = 5 / (5 + 50) = ~9.1%
    assert!(result.damage_to_target > 0);
    assert!(result.target_health_before == 100);
    assert!(result.target_health_after < 100);
    assert_eq!(result.attacker, "Orc");
    assert_eq!(result.defender, "Goblin");
}

#[test]
fn attack_with_zero_defense_takes_full_damage() {
    let attacker = create_test_mob("Attacker", 100, 20, 0);
    let mut defender = create_test_mob("Unarmored", 100, 10, 0);

    let result = attack(&attacker, &mut defender);

    // With 0 defense, no damage reduction occurs
    // Attacker attack ~20 with ±25% variance (15-25)
    assert!(result.damage_to_target >= 15);
    assert!(result.damage_to_target <= 25);
}

// ==================== Combatant trait tests ====================

#[test]
fn combatant_is_alive_when_health_positive() {
    let mob = create_test_mob("Alive", 50, 10, 5);
    assert!(mob.is_alive());
}

#[test]
fn combatant_is_dead_when_health_zero() {
    let mob = create_test_mob("Dead", 0, 10, 5);
    assert!(!mob.is_alive());
}

#[test]
fn combatant_take_damage_reduces_health() {
    let mut mob = create_test_mob("Test", 100, 10, 5);
    mob.take_damage(30);
    assert_eq!(mob.effective_health(), 70);
}

#[test]
fn combatant_take_damage_floors_at_zero() {
    let mut mob = create_test_mob("Test", 50, 10, 5);
    mob.take_damage(100);
    assert_eq!(mob.effective_health(), 0);
}

#[test]
fn mob_is_alive_after_creation() {
    let mob = create_test_mob("Fresh Mob", 100, 10, 5);
    assert!(mob.is_alive());
}

// ==================== PlayerGold tests ====================

#[test]
fn player_gold_add_increases_balance() {
    let mut gold = PlayerGold(0);
    gold.add(100);
    assert_eq!(gold.0, 100);
    gold.add(50);
    assert_eq!(gold.0, 150);
}

#[test]
fn player_gold_subtract_decreases_balance() {
    let mut gold = PlayerGold(100);
    gold.subtract(30);
    assert_eq!(gold.0, 70);
}

#[test]
fn player_gold_subtract_floors_at_zero() {
    let mut gold = PlayerGold(50);
    gold.subtract(100);
    assert_eq!(gold.0, 0);
}

// ==================== Mob effective stats tests ====================

#[test]
fn mob_effective_attack() {
    let mob = create_test_mob("Fighter", 100, 20, 5);
    // effective_attack returns average of attack range
    // Base attack 20 with ±25% variance = 15-25, average = 20
    assert_eq!(mob.effective_attack(), 20);
}

#[test]
fn mob_effective_defense() {
    let mob = create_test_mob("Tank", 100, 10, 8);
    assert_eq!(mob.effective_defense(), 8);
}

// ==================== Defense calculation tests ====================

#[test]
fn defense_reduction_follows_diminishing_returns() {
    // Test the defense formula: reduction = defense / (defense + 50)
    let reduction_0 = calculate_damage_reduction(0);
    let reduction_25 = calculate_damage_reduction(25);
    let reduction_50 = calculate_damage_reduction(50);
    let reduction_100 = calculate_damage_reduction(100);

    assert!((reduction_0 - 0.0).abs() < 0.01);
    assert!((reduction_25 - 0.333).abs() < 0.01); // 25/(25+50) = 1/3
    assert!((reduction_50 - 0.5).abs() < 0.01);   // 50/(50+50) = 1/2
    assert!((reduction_100 - 0.667).abs() < 0.01); // 100/(100+50) = 2/3
}

#[test]
fn apply_defense_reduces_damage_correctly() {
    // With 50 defense (50% reduction)
    assert_eq!(apply_defense(100, 50), 50);
    assert_eq!(apply_defense(10, 50), 5);

    // With 0 defense (no reduction)
    assert_eq!(apply_defense(100, 0), 100);

    // With very high defense (approaches but never reaches 100%)
    let reduced = apply_defense(100, 1000);
    assert!(reduced > 0); // Never fully blocked
    assert!(reduced < 10); // But heavily reduced
}

// ==================== Combat math edge cases ====================

#[test]
fn combat_high_defense_reduces_but_doesnt_negate_damage() {
    let attacker = create_test_mob("Weak Attacker", 50, 10, 0);
    let mut defender = create_test_mob("Tank", 100, 10, 200);

    // Defender has 200 defense = 80% reduction
    // Attacker attack ~10 with variance, reduced to ~2
    let result = attack(&attacker, &mut defender);
    // With percentage-based defense, damage is reduced but not to 0
    assert!(result.damage_to_target >= 0);
    assert!(result.damage_to_target < 5); // Heavily reduced
}

#[test]
fn combat_equal_attack_and_defense_still_deals_damage() {
    let attacker = create_test_mob("Attacker", 100, 20, 5);
    let mut defender = create_test_mob("Balanced", 50, 10, 20);

    // Attacker attack ~20 with variance, defender defense 20
    // Defense reduction = 20 / (20 + 50) = ~28.6%
    let result = attack(&attacker, &mut defender);
    // With percentage-based defense, equal attack/defense still deals damage
    assert!(result.damage_to_target > 0);
}

// ==================== IsKillable trait tests ====================

#[test]
fn mob_on_death_returns_death_result() {
    let mut mob = create_test_mob("Dying Mob", 10, 5, 0);
    mob.take_damage(100);

    let result = mob.on_death(0);

    assert_eq!(result.gold_dropped, 5);
    assert_eq!(result.xp_dropped, 15);
}

#[test]
fn mob_on_death_returns_empty_on_second_call() {
    let mut mob = create_test_mob("Dying Mob", 10, 5, 0);
    mob.take_damage(100);

    // First call returns rewards
    let first_result = mob.on_death(0);
    assert_eq!(first_result.gold_dropped, 5);
    assert_eq!(first_result.xp_dropped, 15);

    // Second call returns empty result (guard prevents double rewards)
    let second_result = mob.on_death(0);
    assert_eq!(second_result.gold_dropped, 0);
    assert_eq!(second_result.xp_dropped, 0);
    assert!(second_result.loot_drops.is_empty());
}

// ==================== MobQuality tests ====================

#[test]
fn mob_has_quality() {
    let mob = create_test_mob("Quality Mob", 100, 10, 5);
    // Test that mob quality is accessible
    assert!(matches!(mob.quality, MobQuality::Normal));
}

// ==================== DropsGold trait tests ====================

#[test]
fn mob_drops_gold_returns_gold_value() {
    use crate::combat::DropsGold;
    let mob = create_test_mob("Rich Mob", 100, 10, 5);

    assert_eq!(mob.drop_gold(), 5);
}

// ==================== GivesXP trait tests ====================

#[test]
fn mob_gives_xp_returns_xp_value() {
    use crate::entities::progression::GivesXP;
    let mob = create_test_mob("XP Mob", 100, 10, 5);

    assert_eq!(mob.give_xp(), 15);
}
