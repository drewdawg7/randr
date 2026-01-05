use crate::{
    combat::{ActiveCombat, AttackResult, CombatEntity, Combatant, CombatPhase, HasGold, IsKillable},
    entities::progression::HasProgression,
    loot::LootDrop,
    magic::effect::PassiveEffect,
    player::Player,
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

pub fn attack<A: Combatant, D: Combatant>(attacker: &A, defender: &mut D)
-> AttackResult {
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
        target_died
    }
}
#[derive(Default, Clone)]
pub struct CombatRounds {
    pub attack_results: Vec<AttackResult>,
    /// Spawned loot drops from the loot table, includes item instances and quantities
    pub loot_drops: Vec<LootDrop>,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub player_won: bool,
}

impl CombatRounds {
    pub fn new() -> Self {
        Self {
            attack_results: Vec::new(),
            loot_drops: Vec::new(),
            gold_gained: 0,
            xp_gained: 0,
            player_won: false,
        }
    }
    pub fn add_round(&mut self, round: AttackResult) {
        self.attack_results.push(round);
    }

    pub fn loot_drops(&self) -> &[LootDrop] {
        &self.loot_drops
    }
}

/// Execute a single player attack step. Returns the AttackResult.
/// Updates the combat phase based on outcome.
pub fn player_attack_step(player: &Player, combat: &mut ActiveCombat) -> AttackResult {
    let result = attack(player, &mut combat.mob);
    combat.rounds.add_round(result.clone());
    combat.last_player_attack = Some(result.clone());

    if !combat.mob.is_alive() {
        combat.phase = CombatPhase::Victory;
    } else {
        combat.phase = CombatPhase::PlayerAttacking;
    }

    result
}

/// Execute a single enemy attack step. Returns the AttackResult.
/// Updates the combat phase based on outcome.
pub fn enemy_attack_step(combat: &mut ActiveCombat, player: &mut Player) -> AttackResult {
    let result = attack(&combat.mob, player);
    combat.rounds.add_round(result.clone());
    combat.last_enemy_attack = Some(result.clone());

    if !player.is_alive() {
        combat.phase = CombatPhase::Defeat;
    } else {
        combat.phase = CombatPhase::PlayerTurn;
    }

    result
}

/// Process victory rewards: gold (with goldfind), XP, and loot drops.
/// Call this when combat.phase == CombatPhase::Victory.
pub fn process_victory(player: &mut Player, combat: &mut ActiveCombat) {
    let death_result = combat.mob.on_death(player.effective_magicfind());

    // Apply gold with goldfind bonus
    let gold_with_bonus = apply_goldfind(death_result.gold_dropped, player.effective_goldfind());
    player.add_gold(gold_with_bonus);
    combat.gold_gained = gold_with_bonus;

    // Award XP (with XP multiplier passive bonus)
    let base_xp = death_result.xp_dropped;
    let xp_bonus_pct: i32 = player
        .tome_passive_effects()
        .iter()
        .filter_map(|e| {
            if let PassiveEffect::XPMultiplier(pct) = e {
                Some(*pct)
            } else {
                None
            }
        })
        .sum();
    let xp_multiplier = 1.0 + (xp_bonus_pct as f64 / 100.0);
    combat.xp_gained = (base_xp as f64 * xp_multiplier).round() as i32;
    player.gain_xp(combat.xp_gained);

    // Store loot drops for spawning
    combat.loot_drops = death_result.loot_drops;
}

/// Process player defeat: gold penalty and health restore.
/// Call this when combat.phase == CombatPhase::Defeat.
pub fn process_defeat(player: &mut Player) {
    let _death_result = player.on_death(0);
}

/// Summary of a combat entity's stats and potential rewards.
#[derive(Debug, Clone)]
pub struct CombatEntityInfo {
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub gold_reward: i32,
    pub xp_reward: i32,
}

/// Extract combat info from any entity implementing CombatEntity.
///
/// Uses the composite CombatEntity trait to access all relevant data
/// with a single trait bound instead of `Combatant + DropsGold + GivesXP + HasLoot`.
pub fn get_combat_entity_info<E: CombatEntity>(entity: &E) -> CombatEntityInfo {
    CombatEntityInfo {
        name: entity.name().to_string(),
        health: entity.effective_health(),
        attack: entity.effective_attack(),
        defense: entity.effective_defense(),
        gold_reward: entity.drop_gold(),
        xp_reward: entity.give_xp(),
    }
}

