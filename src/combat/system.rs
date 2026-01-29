use crate::{
    combat::{Attack, AttackResult, Combatant},
    inventory::Inventory,
    stats::{HasStats, StatSheet, StatType},
};

/// Constant for diminishing returns defense formula.
/// Higher values = more defense needed for same reduction.
/// With K=50: 50 defense = 50% reduction, 100 defense = 67% reduction
const DEFENSE_CONSTANT: f64 = 50.0;

/// Calculate damage reduction percentage with diminishing returns.
/// Uses formula: reduction = defense / (defense + K)
/// Returns a value between 0.0 and 1.0 (never reaches 1.0)
pub fn calculate_damage_reduction(defense: i32) -> f64 {
    let def = defense.max(0) as f64;
    def / (def + DEFENSE_CONSTANT)
}

/// Apply percentage-based defense to raw damage.
/// Returns final damage after reduction.
pub fn apply_defense(raw_damage: i32, defense: i32) -> i32 {
    let reduction = calculate_damage_reduction(defense);
    let damage_multiplier = 1.0 - reduction;
    (raw_damage as f64 * damage_multiplier).round() as i32
}

/// Apply goldfind bonus to base gold amount.
/// Formula: final = base * (1 + goldfind/100)
/// Example: 100 goldfind = 2x gold multiplier
pub fn apply_goldfind(base_gold: i32, goldfind: i32) -> i32 {
    let multiplier = 1.0 + (goldfind as f64 / 100.0);
    ((base_gold as f64) * multiplier).round() as i32
}

/// Result of applying victory rewards to a player.
#[derive(Debug, Clone)]
pub struct VictoryRewards {
    pub gold_gained: i32,
    pub xp_gained: i32,
}

/// Generic attack function for trait-based combat (used in tests).
pub fn attack<A: Combatant, D: Combatant>(attacker: &A, defender: &mut D) -> AttackResult {
    let target_health_before = defender.effective_health();
    let target_defense = defender.effective_defense();

    // Roll damage from attack range
    let raw_damage = attacker.get_attack().roll_damage();
    // Apply percentage-based defense with diminishing returns
    let damage_to_target = apply_defense(raw_damage, target_defense);

    defender.take_damage(damage_to_target);
    let target_health_after = defender.effective_health();
    let target_died = !defender.is_alive();
    AttackResult {
        attacker: attacker.name().to_string(),
        defender: defender.name().to_string(),
        damage_to_target,
        target_health_before,
        target_health_after,
        target_died,
    }
}

// =============================================================================
// Direct Resource Combat Helpers
// =============================================================================
// These functions operate directly on Bevy resources (StatSheet, Inventory)
// instead of requiring the Player struct with trait bounds.

/// Attack variance for player combat (same as DealsDamage trait constant)
const ATTACK_VARIANCE: f64 = 0.25;

/// Apply damage directly to a StatSheet resource.
pub fn player_take_damage(stats: &mut StatSheet, amount: i32) {
    stats.decrease_stat(StatType::Health, amount);
}

/// Get the player's Attack struct (damage range) from resources.
/// Combines base attack stat with equipment bonus.
pub fn player_attack_value(stats: &StatSheet, inventory: &Inventory) -> Attack {
    let base = stats.attack();
    let equipment_bonus = inventory.sum_equipment_stats(StatType::Attack);
    let total = base + equipment_bonus;
    let variance = (total as f64 * ATTACK_VARIANCE).round() as i32;
    Attack::new((total - variance).max(1), total + variance)
}

/// Get the player's effective defense from resources.
/// Combines base defense stat with equipment bonus.
pub fn player_effective_defense(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.defense();
    let equipment_bonus = inventory.sum_equipment_stats(StatType::Defense);
    base + equipment_bonus
}

/// Get the player's effective magic find from resources.
pub fn player_effective_magicfind(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::MagicFind);
    let equipment_bonus = inventory.sum_equipment_stats(StatType::MagicFind);
    base + equipment_bonus
}

/// Get the player's effective gold find from resources.
pub fn player_effective_goldfind(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::GoldFind);
    let equipment_bonus = inventory.sum_equipment_stats(StatType::GoldFind);
    base + equipment_bonus
}

/// Process player defeat using direct resources.
/// Applies gold penalty (5%) and restores health to full.
pub fn process_player_defeat(stats: &mut StatSheet, gold: &mut crate::player::PlayerGold) {
    // 5% gold penalty
    let gold_lost = ((gold.0 as f64) * 0.05).round() as i32;
    gold.subtract(gold_lost);

    // Restore health to full
    let max_hp = stats.max_hp();
    stats.increase_stat(StatType::Health, max_hp);
}

/// Apply victory rewards using direct resources.
/// Returns the calculated gold (with goldfind bonus) and XP for display.
pub fn apply_victory_rewards_direct(
    stats: &StatSheet,
    inventory: &Inventory,
    gold: &mut crate::player::PlayerGold,
    progression: &mut crate::entities::Progression,
    base_gold: i32,
    base_xp: i32,
) -> VictoryRewards {
    // Apply gold with goldfind bonus
    let goldfind = player_effective_goldfind(stats, inventory);
    let gold_gained = apply_goldfind(base_gold, goldfind);
    gold.add(gold_gained);

    // Apply XP
    progression.add_xp(base_xp);

    VictoryRewards {
        gold_gained,
        xp_gained: base_xp,
    }
}

// =============================================================================
// Entity Component-Based Combat Helpers
// =============================================================================
// These functions operate on the new ECS mob components (Health, CombatStats, etc.)
// instead of the old Mob struct.

use crate::mob::{CombatStats, Health};

/// Execute a player attack against a mob entity using ECS components.
/// Returns the AttackResult with damage dealt and whether the target died.
pub fn player_attacks_entity(
    player_name: &str,
    player_stats: &StatSheet,
    player_inventory: &Inventory,
    mob_name: &str,
    mob_health: &mut Health,
    mob_combat_stats: &CombatStats,
) -> AttackResult {
    let target_health_before = mob_health.current;
    let target_defense = mob_combat_stats.defense;

    // Roll damage from player's attack range
    let player_attack = player_attack_value(player_stats, player_inventory);
    let raw_damage = player_attack.roll_damage();
    // Apply percentage-based defense with diminishing returns
    let damage_to_target = apply_defense(raw_damage, target_defense);

    mob_health.take_damage(damage_to_target);
    let target_health_after = mob_health.current;
    let target_died = !mob_health.is_alive();

    AttackResult {
        attacker: player_name.to_string(),
        defender: mob_name.to_string(),
        damage_to_target,
        target_health_before,
        target_health_after,
        target_died,
    }
}

/// Execute a mob entity attack against the player using ECS components.
/// Returns the AttackResult with damage dealt.
pub fn entity_attacks_player(
    mob_name: &str,
    mob_combat_stats: &CombatStats,
    player_name: &str,
    player_stats: &mut StatSheet,
    player_inventory: &Inventory,
) -> AttackResult {
    let target_health_before = player_stats.hp();
    let target_defense = player_effective_defense(player_stats, player_inventory);

    // Roll damage from mob's attack range (using attack stat with variance)
    let base_attack = mob_combat_stats.attack;
    let variance = (base_attack as f64 * ATTACK_VARIANCE).round() as i32;
    let mob_attack = Attack::new((base_attack - variance).max(1), base_attack + variance);
    let raw_damage = mob_attack.roll_damage();
    // Apply percentage-based defense with diminishing returns
    let damage_to_target = apply_defense(raw_damage, target_defense);

    player_take_damage(player_stats, damage_to_target);
    let target_health_after = player_stats.hp();
    let target_died = target_health_after <= 0;

    AttackResult {
        attacker: mob_name.to_string(),
        defender: player_name.to_string(),
        damage_to_target,
        target_health_before,
        target_health_after,
        target_died,
    }
}
