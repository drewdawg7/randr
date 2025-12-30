#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use crate::{
    combat::{attack, award_kill_gold, enter_combat, Combatant, HasGold},
    entities::{
        mob::{Mob, MobKind},
        player::{Player},
        progression::{HasProgression}
    },
    loot::{LootTable},
    stats::{HasStats, StatSheet, StatType}


};

#[cfg(test)]
fn create_test_mob(name: &'static str, hp: i32, attack: i32, defense: i32) -> Mob {
    let mut stats = HashMap::new();
    stats.insert(StatType::Health, StatType::Health.instance(hp));
    stats.insert(StatType::Attack, StatType::Attack.instance(attack));
    stats.insert(StatType::Defense, StatType::Defense.instance(defense));
    Mob {
        spec: MobKind::Slime,
        name,
        stats: StatSheet { stats },
        gold: 5,
        loot_table: LootTable::default(),
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
fn attack_deals_damage_based_on_attack_minus_defense() {
    let player = create_test_player(100, 20, 5);
    let mut mob = create_test_mob("Goblin", 100, 10, 8);

    let result = attack(&player, &mut mob);

    // Damage = player attack (20) - mob defense (8) = 12
    assert_eq!(result.damage_to_target, 12);
    assert_eq!(result.target_health_before, 100);
    assert_eq!(result.target_health_after, 88);
    assert!(!result.target_died);
}

#[test]
fn attack_minimum_damage_is_zero() {
    let player = create_test_player(100, 5, 0);
    let mut mob = create_test_mob("Tank", 100, 10, 20);

    let result = attack(&player, &mut mob);

    // Defense (20) > Attack (5), so damage should be 0
    assert_eq!(result.damage_to_target, 0);
    assert_eq!(result.target_health_after, 100);
}

#[test]
fn attack_kills_target_when_damage_exceeds_health() {
    let player = create_test_player(100, 50, 0);
    let mut mob = create_test_mob("Weakling", 10, 5, 0);

    let result = attack(&player, &mut mob);

    assert_eq!(result.damage_to_target, 50);
    assert_eq!(result.target_health_after, 0);
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

    // Damage = mob attack (20) - player defense (5) = 15
    assert_eq!(result.damage_to_target, 15);
    assert_eq!(result.target_health_before, 100);
    assert_eq!(result.target_health_after, 85);
    assert_eq!(result.attacker, "Orc");
    assert_eq!(result.defender, "Drew");
}

#[test]
fn attack_with_zero_defense_takes_full_damage() {
    let player = create_test_player(100, 25, 0);
    let mut mob = create_test_mob("Unarmored", 100, 10, 0);

    let result = attack(&player, &mut mob);

    assert_eq!(result.damage_to_target, 25);
    assert_eq!(result.target_health_after, 75);
}

// ==================== award_kill_gold() tests ====================

#[test]
fn award_kill_gold_transfers_gold_to_player() {
    let mut player = create_test_player(100, 20, 5);
    let mut mob = create_test_mob("Goblin", 50, 10, 2);

    assert_eq!(player.gold(), 0);

    let dropped = award_kill_gold(&mut player, &mut mob);
    assert!((1..=5).contains(&dropped));
    assert_eq!(player.gold(), dropped);
}

#[test]
fn award_kill_gold_accumulates() {
    let mut player = create_test_player(100, 20, 5);
    player.add_gold(50);
    let mut mob = create_test_mob("Goblin", 50, 10, 2);

    let starting_gold = player.gold();
    let dropped = award_kill_gold(&mut player, &mut mob);

    assert_eq!(player.gold(), starting_gold + dropped);
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
    // Gold and XP should be awarded
    assert!(result.gold_gained >= 1);
    assert!(result.xp_gained >= 15); // Mob gives 15-20 XP
}

#[test]
fn enter_combat_player_loses_when_weaker() {
    let mut player = create_test_player(20, 5, 0);
    let mut mob = create_test_mob("Strong Orc", 100, 30, 10);

    let result = enter_combat(&mut player, &mut mob);

    assert!(!result.player_won);
    assert!(!player.is_alive());
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
    assert!(result.xp_gained >= 15 && result.xp_gained <= 20);
    assert!(player.progression().xp > starting_xp || player.progression().level > 1);
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
    let player = create_test_player(100, 15, 5);
    // Without weapon equipped, effective attack equals base attack
    assert_eq!(player.effective_attack(), 15);
}

#[test]
fn player_effective_defense_without_shield() {
    let player = create_test_player(100, 10, 8);
    // Without shield equipped, effective defense equals base defense
    assert_eq!(player.effective_defense(), 8);
}

// ==================== Combat math edge cases ====================

#[test]
fn combat_high_defense_negates_damage() {
    let mut player = create_test_player(100, 10, 50);
    let mob = create_test_mob("Weak Attacker", 50, 5, 0);

    // Mob attacks player, but player defense (50) > mob attack (5)
    let result = attack(&mob, &mut player);
    assert_eq!(result.damage_to_target, 0);
    assert_eq!(player.hp(), 100);
}

#[test]
fn combat_equal_attack_and_defense() {
    let player = create_test_player(100, 20, 5);
    let mut mob = create_test_mob("Balanced", 50, 10, 20);

    // Player attack (20) == mob defense (20) => 0 damage
    let result = attack(&player, &mut mob);
    assert_eq!(result.damage_to_target, 0);
}
