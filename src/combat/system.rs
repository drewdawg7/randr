use crate::{
    combat::{Attack, AttackResult},
    inventory::Inventory,
    stats::{HasStats, StatSheet, StatType},
};

const DEFENSE_CONSTANT: f64 = 50.0;

pub fn calculate_damage_reduction(defense: i32) -> f64 {
    let def = defense.max(0) as f64;
    def / (def + DEFENSE_CONSTANT)
}

pub fn apply_defense(raw_damage: i32, defense: i32) -> i32 {
    let reduction = calculate_damage_reduction(defense);
    let damage_multiplier = 1.0 - reduction;
    (raw_damage as f64 * damage_multiplier).round() as i32
}

pub fn apply_goldfind(base_gold: i32, goldfind: i32) -> i32 {
    let multiplier = 1.0 + (goldfind as f64 / 100.0);
    ((base_gold as f64) * multiplier).round() as i32
}

#[derive(Debug, Clone)]
pub struct VictoryRewards {
    pub gold_gained: i32,
    pub xp_gained: i32,
}

const ATTACK_VARIANCE: f64 = 0.25;

pub fn player_take_damage(stats: &mut StatSheet, amount: i32) {
    stats.decrease_stat(StatType::Health, amount);
}

pub fn player_attack_value(stats: &StatSheet, inventory: &Inventory) -> Attack {
    let base = stats.attack();
    let equipment_bonus = inventory.sum_equipment_stats(StatType::Attack);
    let total = base + equipment_bonus;
    let variance = (total as f64 * ATTACK_VARIANCE).round() as i32;
    Attack::new((total - variance).max(1), total + variance)
}

pub fn player_effective_defense(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.defense();
    let equipment_bonus = inventory.sum_equipment_stats(StatType::Defense);
    base + equipment_bonus
}

pub fn player_effective_magicfind(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::MagicFind);
    let equipment_bonus = inventory.sum_equipment_stats(StatType::MagicFind);
    base + equipment_bonus
}

pub fn player_effective_goldfind(stats: &StatSheet, inventory: &Inventory) -> i32 {
    let base = stats.value(StatType::GoldFind);
    let equipment_bonus = inventory.sum_equipment_stats(StatType::GoldFind);
    base + equipment_bonus
}

pub fn process_player_defeat(stats: &mut StatSheet, gold: &mut crate::player::PlayerGold) {
    let gold_lost = ((gold.0 as f64) * 0.05).round() as i32;
    gold.subtract(gold_lost);

    let max_hp = stats.max_hp();
    stats.increase_stat(StatType::Health, max_hp);
}

pub fn apply_victory_rewards_direct(
    stats: &mut StatSheet,
    inventory: &Inventory,
    gold: &mut crate::player::PlayerGold,
    progression: &mut crate::entities::Progression,
    base_gold: i32,
    base_xp: i32,
) -> VictoryRewards {
    let goldfind = player_effective_goldfind(stats, inventory);
    let gold_gained = apply_goldfind(base_gold, goldfind);
    gold.add(gold_gained);

    progression.add_xp(base_xp);

    VictoryRewards {
        gold_gained,
        xp_gained: base_xp,
    }
}

use crate::mob::{CombatStats, Health};

pub fn player_attacks_entity(
    player_stats: &StatSheet,
    player_inventory: &Inventory,
    mob_health: &mut Health,
    mob_combat_stats: &CombatStats,
) -> AttackResult {
    let player_attack = player_attack_value(player_stats, player_inventory);
    let raw_damage = player_attack.roll_damage();
    let damage = apply_defense(raw_damage, mob_combat_stats.defense);

    mob_health.take_damage(damage);

    AttackResult {
        target_died: !mob_health.is_alive(),
    }
}

pub fn entity_attacks_player(
    mob_combat_stats: &CombatStats,
    player_stats: &mut StatSheet,
    player_inventory: &Inventory,
) -> AttackResult {
    let defense = player_effective_defense(player_stats, player_inventory);

    let base_attack = mob_combat_stats.attack;
    let variance = (base_attack as f64 * ATTACK_VARIANCE).round() as i32;
    let mob_attack = Attack::new((base_attack - variance).max(1), base_attack + variance);
    let raw_damage = mob_attack.roll_damage();
    let damage = apply_defense(raw_damage, defense);

    player_take_damage(player_stats, damage);

    AttackResult {
        target_died: player_stats.hp() <= 0,
    }
}
