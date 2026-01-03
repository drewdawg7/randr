use crate::{
    combat::{ActiveCombat, AttackResult, Combatant, CombatPhase, HasGold, IsKillable, MobDeathResult},
    entities::{progression::HasProgression, Player},
    loot::LootDrop,
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


pub fn enter_combat<M>(player: &mut Player, mob: &mut M) -> CombatRounds
where
    M: Combatant + IsKillable<DeathResult = MobDeathResult>,
{
    let mut cr = CombatRounds::default();
    while player.is_alive() && mob.is_alive() {
        let a1 = attack(player, mob);
        cr.add_round(a1);
        if mob.is_alive() {
            let a2 = attack(mob, player);
            cr.add_round(a2);
        }
    }
    if !player.is_alive() {
        cr.player_won = false;
        let _death_result = player.on_death(0);
    } else if !mob.is_alive() {
        cr.player_won = true;
        let death_result = mob.on_death(player.effective_magicfind());

        // Apply gold with goldfind bonus
        let gf = player.effective_goldfind();
        let multiplier = 1.0 + (gf as f64 / 100.0);
        let gold_with_bonus = ((death_result.gold_dropped as f64) * multiplier).round() as i32;
        player.add_gold(gold_with_bonus);
        cr.gold_gained = death_result.gold_dropped;

        // Award XP
        cr.xp_gained = death_result.xp_dropped;
        player.gain_xp(cr.xp_gained);

        // Set loot drops
        cr.loot_drops = death_result.loot_drops;
    }
    cr
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
    let gf = player.effective_goldfind();
    let multiplier = 1.0 + (gf as f64 / 100.0);
    let gold_with_bonus = ((death_result.gold_dropped as f64) * multiplier).round() as i32;
    player.add_gold(gold_with_bonus);
    combat.gold_gained = death_result.gold_dropped;

    // Award XP
    combat.xp_gained = death_result.xp_dropped;
    player.gain_xp(combat.xp_gained);

    // Store loot drops for spawning
    combat.loot_drops = death_result.loot_drops;
}

/// Process player defeat: gold penalty and health restore.
/// Call this when combat.phase == CombatPhase::Defeat.
pub fn process_defeat(player: &mut Player) {
    let _death_result = player.on_death(0);
}

