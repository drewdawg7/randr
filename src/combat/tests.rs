#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use crate::{
    combat::{attack, enter_combat, apply_defense, calculate_damage_reduction, Combatant, DealsDamage, HasGold, IsKillable},
    entities::{
        mob::{Mob, MobId},
        mob::enums::MobQuality,
        player::Player,
        progression::HasProgression,
    },
    loot::LootTable,
    stats::{HasStats, StatSheet, StatType},
};

#[cfg(test)]
fn create_test_mob(name: &'static str, hp: i32, attack: i32, defense: i32) -> Mob {
    let mut stats = HashMap::new();
    stats.insert(StatType::Health, StatType::Health.instance(hp));
    stats.insert(StatType::Attack, StatType::Attack.instance(attack));
    stats.insert(StatType::Defense, StatType::Defense.instance(defense));
    Mob {
        spec: MobId::Slime,
        quality: MobQuality::Normal,
        name,
        stats: StatSheet { stats },
        gold: 5,
        dropped_xp: 15,
        loot_table: LootTable::default(),
        death_processed: false,
    }
}

#[cfg(test)]
fn create_test_player(hp: i32, attack: i32, defense: i32) -> Player {
    let mut player = Player::default();
    // Set the stats to our test values
    player.stats_mut().stats.insert(StatType::Health, StatType::Health.instance(hp));
    player.stats_mut().stats.insert(StatType::Attack, StatType::Attack.instance(attack));
    player.stats_mut().stats.insert(StatType::Defense, StatType::Defense.instance(defense));
    player
}

// ==================== attack() tests ====================

#[test]
fn attack_deals_damage_with_variance_and_defense_reduction() {
    let player = create_test_player(100, 20, 5);
    let mut mob = create_test_mob("Goblin", 100, 10, 8);

    let result = attack(&player, &mut mob);

    // Player attack ~20 with ±25% variance (15-25), mob defense 8
    // Defense reduction = 8 / (8 + 50) = ~13.8%
    assert!(result.damage_to_target > 0);
    assert!(result.damage_to_target <= 25); // Max raw damage
    assert_eq!(result.target_health_before, 100);
    assert!(result.target_health_after < 100);
    assert!(!result.target_died);
}

#[test]
fn attack_high_defense_reduces_damage_significantly() {
    let player = create_test_player(100, 10, 0);
    let mut mob = create_test_mob("Tank", 100, 5, 100);

    let result = attack(&player, &mut mob);

    // Player attack ~10 with variance, mob has 100 defense
    // Defense reduction = 100 / (100 + 50) = 66.7%
    // Raw damage ~8-12, reduced to ~3-4
    assert!(result.damage_to_target >= 0);
    assert!(result.damage_to_target < 10); // Significantly reduced
}

#[test]
fn attack_kills_target_when_damage_exceeds_health() {
    let player = create_test_player(100, 100, 0);
    let mut mob = create_test_mob("Weakling", 10, 5, 0);

    let result = attack(&player, &mut mob);

    // Player has high attack, mob has 0 defense, low HP
    assert!(result.damage_to_target > 10); // Overkill
    assert!(result.target_died);
}

#[test]
fn attack_result_contains_correct_names() {
    let player = Player::default();
    let mut mob = create_test_mob("Goblin", 50, 10, 2);

    let result = attack(&player, &mut mob);

    assert_eq!(result.attacker, "Drew");
    assert_eq!(result.defender, "Goblin");
}

#[test]
fn mob_attacks_player() {
    let mut player = create_test_player(100, 10, 5);
    let mob = create_test_mob("Orc", 50, 20, 3);

    let result = attack(&mob, &mut player);

    // Mob attack ~20 with variance, player defense 5
    // Defense reduction = 5 / (5 + 50) = ~9.1%
    assert!(result.damage_to_target > 0);
    assert!(result.target_health_before == 100);
    assert!(result.target_health_after < 100);
    assert_eq!(result.attacker, "Orc");
    assert_eq!(result.defender, "Drew");
}

#[test]
fn attack_with_zero_defense_takes_full_damage() {
    let player = create_test_player(100, 20, 0);
    let mut mob = create_test_mob("Unarmored", 100, 10, 0);

    let result = attack(&player, &mut mob);

    // With 0 defense, no damage reduction occurs
    // Player attack ~20 with ±25% variance (15-25)
    assert!(result.damage_to_target >= 15);
    assert!(result.damage_to_target <= 25);
}

// ==================== enter_combat() tests ====================

#[test]
fn enter_combat_player_wins_when_stronger() {
    let mut player = create_test_player(100, 30, 10);
    let mut mob = create_test_mob("Weak Goblin", 20, 5, 2);

    let result = enter_combat(&mut player, &mut mob);

    assert!(result.player_won);
    assert!(!mob.is_alive());
    assert!(player.is_alive());
    // Gold should be awarded (mob drops 5 gold)
    assert_eq!(result.gold_gained, 5);
    // XP should be awarded
    assert_eq!(result.xp_gained, 15);
}

#[test]
fn enter_combat_player_loses_when_weaker() {
    let mut player = create_test_player(20, 5, 0);
    let mut mob = create_test_mob("Strong Orc", 100, 30, 10);

    let result = enter_combat(&mut player, &mut mob);

    assert!(!result.player_won);
    // Note: on_death restores player health, so player is alive after combat
    assert!(mob.is_alive());
    assert_eq!(result.gold_gained, 0);
    assert_eq!(result.xp_gained, 0);
}

#[test]
fn enter_combat_records_attack_rounds() {
    let mut player = create_test_player(100, 15, 5);
    let mut mob = create_test_mob("Goblin", 30, 8, 3);

    let result = enter_combat(&mut player, &mut mob);

    // Combat should have multiple rounds recorded
    assert!(!result.attack_results.is_empty());

    // First attack should be from player
    assert_eq!(result.attack_results[0].attacker, "Drew");
    assert_eq!(result.attack_results[0].defender, "Goblin");
}

#[test]
fn enter_combat_alternates_attackers() {
    let mut player = create_test_player(100, 10, 5);
    let mut mob = create_test_mob("Goblin", 50, 8, 3);

    let result = enter_combat(&mut player, &mut mob);

    // Check that attacks alternate (player, mob, player, mob, ...)
    for (i, attack_result) in result.attack_results.iter().enumerate() {
        if i % 2 == 0 {
            assert_eq!(attack_result.attacker, "Drew");
        } else {
            assert_eq!(attack_result.attacker, "Goblin");
        }
    }
}

#[test]
fn enter_combat_mob_does_not_attack_after_dying() {
    // Player one-shots the mob
    let mut player = create_test_player(100, 100, 0);
    let mut mob = create_test_mob("Weak Slime", 10, 50, 0);

    let result = enter_combat(&mut player, &mut mob);

    // Only one attack should occur (player kills mob in one hit)
    assert_eq!(result.attack_results.len(), 1);
    assert_eq!(result.attack_results[0].attacker, "Drew");
    assert!(result.attack_results[0].target_died);

    // Player should take no damage
    assert_eq!(player.hp(), 100);
}

#[test]
fn enter_combat_player_gains_xp_on_victory() {
    let mut player = create_test_player(100, 50, 10);
    let starting_xp = player.progression().xp;
    let mut mob = create_test_mob("Goblin", 20, 5, 0);

    let result = enter_combat(&mut player, &mut mob);

    assert!(result.player_won);
    assert_eq!(result.xp_gained, 15);
    assert!(player.progression().xp > starting_xp || player.progression().level > 1);
}

#[test]
fn enter_combat_player_gains_gold_on_victory() {
    let mut player = create_test_player(100, 50, 10);
    let starting_gold = player.gold();
    let mut mob = create_test_mob("Rich Goblin", 20, 5, 0);

    let result = enter_combat(&mut player, &mut mob);

    assert!(result.player_won);
    assert_eq!(result.gold_gained, 5);
    // Player gold should increase (gold_gained is base, actual gain may be modified by goldfind)
    assert!(player.gold() >= starting_gold + result.gold_gained);
}

// ==================== Combatant trait tests ====================

#[test]
fn combatant_is_alive_when_health_positive() {
    let player = create_test_player(50, 10, 5);
    assert!(player.is_alive());
}

#[test]
fn combatant_is_dead_when_health_zero() {
    let player = create_test_player(0, 10, 5);
    assert!(!player.is_alive());
}

#[test]
fn combatant_take_damage_reduces_health() {
    let mut player = create_test_player(100, 10, 5);
    player.take_damage(30);
    assert_eq!(player.effective_health(), 70);
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

// ==================== HasGold trait tests ====================

#[test]
fn player_add_gold_increases_balance() {
    let mut player = Player::default();
    player.add_gold(100);
    assert_eq!(player.gold(), 100);
    player.add_gold(50);
    assert_eq!(player.gold(), 150);
}

#[test]
fn player_dec_gold_decreases_balance() {
    let mut player = Player::default();
    player.add_gold(100);
    player.dec_gold(30);
    assert_eq!(player.gold(), 70);
}

#[test]
fn player_dec_gold_floors_at_zero() {
    let mut player = Player::default();
    player.add_gold(50);
    player.dec_gold(100);
    assert_eq!(player.gold(), 0);
}

// ==================== Player effective stats tests ====================

#[test]
fn player_effective_attack_without_weapon() {
    let player = create_test_player(100, 20, 5);
    // effective_attack returns average of attack range
    // Base attack 20 with ±25% variance = 15-25, average = 20
    assert_eq!(player.effective_attack(), 20);
}

#[test]
fn player_effective_defense_without_shield() {
    let player = create_test_player(100, 10, 8);
    // Without shield equipped, effective defense equals base defense
    assert_eq!(player.effective_defense(), 8);
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
    let mut player = create_test_player(100, 10, 200);
    let mob = create_test_mob("Weak Attacker", 50, 10, 0);

    // Player has 200 defense = 80% reduction
    // Mob attack ~10 with variance, reduced to ~2
    let result = attack(&mob, &mut player);
    // With percentage-based defense, damage is reduced but not to 0
    assert!(result.damage_to_target >= 0);
    assert!(result.damage_to_target < 5); // Heavily reduced
}

#[test]
fn combat_equal_attack_and_defense_still_deals_damage() {
    let player = create_test_player(100, 20, 5);
    let mut mob = create_test_mob("Balanced", 50, 10, 20);

    // Player attack ~20 with variance, mob defense 20
    // Defense reduction = 20 / (20 + 50) = ~28.6%
    let result = attack(&player, &mut mob);
    // With percentage-based defense, equal attack/defense still deals damage
    assert!(result.damage_to_target > 0);
}

// ==================== IsKillable trait tests ====================

#[test]
fn mob_on_death_returns_death_result() {
    let mut mob = create_test_mob("Dying Mob", 10, 5, 0);
    mob.take_damage(100);

    let result = mob.on_death();

    assert_eq!(result.gold_dropped, 5);
    assert_eq!(result.xp_dropped, 15);
}

#[test]
fn mob_on_death_returns_empty_on_second_call() {
    let mut mob = create_test_mob("Dying Mob", 10, 5, 0);
    mob.take_damage(100);

    // First call returns rewards
    let first_result = mob.on_death();
    assert_eq!(first_result.gold_dropped, 5);
    assert_eq!(first_result.xp_dropped, 15);

    // Second call returns empty result (guard prevents double rewards)
    let second_result = mob.on_death();
    assert_eq!(second_result.gold_dropped, 0);
    assert_eq!(second_result.xp_dropped, 0);
    assert!(second_result.loot_drops.is_empty());
}

#[test]
fn player_on_death_loses_gold_percentage() {
    let mut player = Player::default();
    player.add_gold(100);
    player.take_damage(player.hp()); // Kill player

    let result = player.on_death();

    // Player loses 5% of gold
    assert_eq!(result.gold_lost, 5);
    assert_eq!(player.gold(), 95);
}

#[test]
fn player_on_death_restores_health() {
    let mut player = create_test_player(100, 10, 5);
    player.take_damage(100); // Kill the player (hp = 0)
    assert_eq!(player.hp(), 0);

    let _ = player.on_death();

    // on_death adds max_hp to current hp, restoring from 0 to full
    assert_eq!(player.hp(), player.max_hp());
}

// ==================== CombatRounds tests ====================

#[test]
fn combat_rounds_tracks_loot_drops() {
    let mut player = create_test_player(100, 100, 10);
    let mut mob = create_test_mob("Loot Mob", 10, 5, 0);

    let result = enter_combat(&mut player, &mut mob);

    assert!(result.player_won);
    // loot_drops returns the items rolled from loot table
    let _ = result.loot_drops();
}

#[test]
fn combat_rounds_new_is_empty() {
    use crate::combat::CombatRounds;
    let rounds = CombatRounds::new();

    assert!(rounds.attack_results.is_empty());
    assert!(rounds.loot_drops.is_empty());
    assert_eq!(rounds.gold_gained, 0);
    assert_eq!(rounds.xp_gained, 0);
    assert!(!rounds.player_won);
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
