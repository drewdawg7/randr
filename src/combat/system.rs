use crate::{
    combat::{AttackResult, Combatant, DropsGold, HasGold},
    entities::{mob::MobKind, progression::{GivesXP, HasProgression}},
    inventory::HasInventory,
    item::{Item, ItemKind},
    loot::HasLoot,
    system::game_state,
    ui::Id,
};


pub fn award_kill_gold<K: HasGold, T:DropsGold>(killer: &mut K, target: &mut T) -> i32 
{
    let dropped = target.drop_gold();
    killer.add_gold(dropped);
    dropped
}

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
    /// Item kinds rolled from loot table, used internally before spawning
    loot_kinds: Vec<ItemKind>,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub player_won: bool,
}

impl CombatRounds {
    pub fn new() -> Self {
        Self {
            attack_results: Vec::new(),
            dropped_loot: Vec::new(),
            loot_kinds: Vec::new(),
            gold_gained: 0,
            xp_gained: 0,
            player_won: false,
        }
    }
    pub fn add_round(&mut self, round: AttackResult) {
        self.attack_results.push(round);
    }

    pub fn loot_kinds(&self) -> &[ItemKind] {
        &self.loot_kinds
    }
}


pub fn enter_combat<P, M>(player: &mut P, mob: &mut M) -> CombatRounds
where
    P: Combatant + HasGold + HasProgression,
    M: Combatant + DropsGold + GivesXP + HasLoot,
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
        player.dec_gold(
            ((player.gold() as f64) * 0.05).round() as i32
        );
        player.increase_health(player.max_hp());

    } else if !mob.is_alive() {
        cr.player_won = true;
        cr.gold_gained = award_kill_gold(player, mob);
        cr.xp_gained = mob.give_xp();
        player.gain_xp(cr.xp_gained);

        // Roll for loot drops (kinds only - items spawned later with quality)
        cr.loot_kinds = mob.loot().roll_drops();
    }
    cr
}

pub fn start_fight(mob_kind: MobKind) {
    let gs = game_state();
    let mut mob = gs.spawn_mob(mob_kind);
    let mut combat_rounds = enter_combat(&mut gs.player, &mut mob);

    // Spawn items with quality and add to both dropped_loot and inventory
    for item_kind in &combat_rounds.loot_kinds.clone() {
        let item = gs.spawn_item(*item_kind);
        combat_rounds.dropped_loot.push(item.clone());
        let _ = gs.player.add_to_inv(item);
    }

    gs.set_current_combat(combat_rounds);
    gs.current_screen = Id::Fight;
}
