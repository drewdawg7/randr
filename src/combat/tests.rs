#[cfg(test)]
use crate::{
    combat::{apply_defense, calculate_damage_reduction},
    player::PlayerGold,
};

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

// ==================== Defense calculation tests ====================

#[test]
fn defense_reduction_follows_diminishing_returns() {
    let reduction_0 = calculate_damage_reduction(0);
    let reduction_25 = calculate_damage_reduction(25);
    let reduction_50 = calculate_damage_reduction(50);
    let reduction_100 = calculate_damage_reduction(100);

    assert!((reduction_0 - 0.0).abs() < 0.01);
    assert!((reduction_25 - 0.333).abs() < 0.01);
    assert!((reduction_50 - 0.5).abs() < 0.01);
    assert!((reduction_100 - 0.667).abs() < 0.01);
}

#[test]
fn apply_defense_reduces_damage_correctly() {
    assert_eq!(apply_defense(100, 50), 50);
    assert_eq!(apply_defense(10, 50), 5);

    assert_eq!(apply_defense(100, 0), 100);

    let reduced = apply_defense(100, 1000);
    assert!(reduced > 0);
    assert!(reduced < 10);
}
