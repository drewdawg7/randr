use crate::{
    combat::{AttackResult, Combatant, HasGold, IsKillable, MobDeathResult},
    entities::{progression::HasProgression, Player},
    item::{Item, ItemId},
};

pub fn attack<A: Combatant, D: Combatant>(attacker: &A, defender: &mut D)
-> AttackResult {
    let target_health_before = defender.effective_health();
    let target_defense = defender.effective_defense();

    let damage_to_target = (attacker.effective_attack() - target_defense).max(0);
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
    /// Spawned items with full quality info, for display and inventory
    pub dropped_loot: Vec<Item>,
    /// Item drops rolled from loot table (item_id, quantity), used internally before spawning
    loot_drops: Vec<(ItemId, i32)>,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub player_won: bool,
}

impl CombatRounds {
    pub fn new() -> Self {
        Self {
            attack_results: Vec::new(),
            dropped_loot: Vec::new(),
            loot_drops: Vec::new(),
            gold_gained: 0,
            xp_gained: 0,
            player_won: false,
        }
    }
    pub fn add_round(&mut self, round: AttackResult) {
        self.attack_results.push(round);
    }

    pub fn loot_drops(&self) -> &[(ItemId, i32)] {
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
        let _death_result = player.on_death();
    } else if !mob.is_alive() {
        cr.player_won = true;
        let death_result = mob.on_death();

        // Apply gold with goldfind bonus
        let gf = player.get_effective_goldfind();
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

