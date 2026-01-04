use crate::{
    combat::{ActiveCombat, AttackResult, Combatant, CombatPhase, HasGold, IsKillable, MobDeathResult, SpellCastResult},
    entities::progression::HasProgression,
    loot::LootDrop,
    magic::{effect::{ActiveEffect, PassiveEffect}, spell::ComputedSpell},
    player::Player,
    stats::HasStats,
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
        let gold_with_bonus = apply_goldfind(death_result.gold_dropped, player.effective_goldfind());
        player.add_gold(gold_with_bonus);
        cr.gold_gained = gold_with_bonus;

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

/// Execute a spell cast step. Returns the SpellCastResult.
/// Updates the combat phase based on outcome.
pub fn player_cast_spell_step(player: &mut Player, combat: &mut ActiveCombat) -> SpellCastResult {
    // Get the equipped tome and active spell
    let Some(tome) = player.equipped_tome() else {
        return SpellCastResult::NoSpell;
    };

    let Some(spell) = tome.active_spell() else {
        return SpellCastResult::NoSpell;
    };

    // Get the active effect from the spell
    let (spell_name, effect) = match spell {
        ComputedSpell::Active { name, effect, .. } => (name.clone(), effect.clone()),
        ComputedSpell::Hybrid { name, active, .. } => (name.clone(), active.clone()),
        ComputedSpell::Passive { .. } => {
            return SpellCastResult::Fizzle {
                reason: "Passive spells cannot be cast in combat".to_string(),
            };
        }
        ComputedSpell::Backfire { reason, .. } => {
            return SpellCastResult::Fizzle {
                reason: format!("Spell backfired: {}", reason),
            };
        }
        ComputedSpell::Fizzle { reason } => {
            return SpellCastResult::Fizzle {
                reason: reason.clone(),
            };
        }
    };

    // Apply the effect
    let result = apply_active_effect(player, combat, &spell_name, &effect);

    // Update combat phase based on outcome
    match &result {
        SpellCastResult::Damage { target_died, .. }
        | SpellCastResult::LifeDrain { target_died, .. } => {
            if *target_died {
                combat.phase = CombatPhase::Victory;
            } else {
                combat.phase = CombatPhase::PlayerAttacking;
            }
        }
        SpellCastResult::Heal { .. } => {
            // After healing, it's still the player's turn ending
            combat.phase = CombatPhase::PlayerAttacking;
        }
        _ => {}
    }

    result
}

/// Apply an active spell effect and return the result
fn apply_active_effect(
    player: &mut Player,
    combat: &mut ActiveCombat,
    spell_name: &str,
    effect: &ActiveEffect,
) -> SpellCastResult {
    match effect {
        ActiveEffect::Damage { amount, element } => {
            let target_health_before = combat.mob.hp();
            let target_defense = combat.mob.effective_defense();

            // Apply defense reduction to spell damage
            let damage_dealt = apply_defense(*amount, target_defense);
            combat.mob.take_damage(damage_dealt);

            let target_health_after = combat.mob.hp();
            let target_died = !combat.mob.is_alive();

            SpellCastResult::Damage {
                spell_name: spell_name.to_string(),
                damage_dealt,
                element: element.clone(),
                target_health_before,
                target_health_after,
                target_died,
            }
        }

        ActiveEffect::Heal { amount } => {
            let max_hp = player.max_hp();
            let hp_before = player.hp();
            let amount_healed = (*amount).min(max_hp - hp_before);
            player.increase_health(amount_healed);

            SpellCastResult::Heal {
                spell_name: spell_name.to_string(),
                amount_healed,
                caster_health_after: player.hp(),
            }
        }

        ActiveEffect::LifeDrain { damage, heal_percent } => {
            let target_health_before = combat.mob.hp();
            let target_defense = combat.mob.effective_defense();

            // Apply damage
            let damage_dealt = apply_defense(*damage, target_defense);
            combat.mob.take_damage(damage_dealt);

            let target_health_after = combat.mob.hp();
            let target_died = !combat.mob.is_alive();

            // Heal based on damage dealt
            let heal_amount = (damage_dealt * heal_percent / 100).max(1);
            let max_hp = player.max_hp();
            let hp_before = player.hp();
            let amount_healed = heal_amount.min(max_hp - hp_before);
            player.increase_health(amount_healed);

            SpellCastResult::LifeDrain {
                spell_name: spell_name.to_string(),
                damage_dealt,
                amount_healed,
                target_health_after,
                caster_health_after: player.hp(),
                target_died,
            }
        }

        ActiveEffect::AreaDamage { amount, element } => {
            // Treat as single-target damage in current 1v1 combat
            let target_health_before = combat.mob.hp();
            let target_defense = combat.mob.effective_defense();

            let damage_dealt = apply_defense(*amount, target_defense);
            combat.mob.take_damage(damage_dealt);

            let target_health_after = combat.mob.hp();
            let target_died = !combat.mob.is_alive();

            SpellCastResult::Damage {
                spell_name: spell_name.to_string(),
                damage_dealt,
                element: element.clone(),
                target_health_before,
                target_health_after,
                target_died,
            }
        }

        ActiveEffect::DefenseBuff { .. } => {
            // TODO: Implement defense buff tracking
            SpellCastResult::Fizzle {
                reason: "Defense buffs not yet implemented".to_string(),
            }
        }

        ActiveEffect::Slow { .. } => {
            // TODO: Implement slow effect tracking
            SpellCastResult::Fizzle {
                reason: "Slow effects not yet implemented".to_string(),
            }
        }

        ActiveEffect::DamageWithEffect { damage, element, secondary } => {
            // Apply primary damage first
            let target_health_before = combat.mob.hp();
            let target_defense = combat.mob.effective_defense();

            let damage_dealt = apply_defense(*damage, target_defense);
            combat.mob.take_damage(damage_dealt);

            let target_health_after = combat.mob.hp();
            let target_died = !combat.mob.is_alive();

            // Apply secondary effect (if target is still alive)
            if !target_died {
                // Recursively apply secondary effect (result ignored for now)
                let _ = apply_active_effect(player, combat, spell_name, secondary);
            }

            SpellCastResult::Damage {
                spell_name: spell_name.to_string(),
                damage_dealt,
                element: element.clone(),
                target_health_before,
                target_health_after,
                target_died,
            }
        }
    }
}

/// Check if the player has an active spell available to cast
pub fn player_has_castable_spell(player: &Player) -> bool {
    player
        .equipped_tome()
        .and_then(|tome| tome.active_spell())
        .map(|spell| matches!(spell, ComputedSpell::Active { .. } | ComputedSpell::Hybrid { .. }))
        .unwrap_or(false)
}

/// Get the name of the player's active spell, if any
pub fn get_active_spell_name(player: &Player) -> Option<String> {
    player
        .equipped_tome()
        .and_then(|tome| tome.active_spell())
        .and_then(|spell| match spell {
            ComputedSpell::Active { name, .. } => Some(name.clone()),
            ComputedSpell::Hybrid { name, .. } => Some(name.clone()),
            _ => None,
        })
}

