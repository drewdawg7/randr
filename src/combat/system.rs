use crate::{
    combat::{AttackResult, Combatant, DropsGold, HasGold},
    entities::{mob::MobKind, progression::{GivesXP, HasProgression}},
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
    let target_health_before = defender.health();
    let damage_to_target = attacker.attack_power();
    defender.take_damage(damage_to_target);
    let target_health_after = defender.health();
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
}

impl CombatRounds {
    pub fn new() -> Self {
        Self {
            attack_results: Vec::new()
        }
    }
    pub fn add_round(&mut self, round: AttackResult) {
        self.attack_results.push(round);
    } 
}


pub fn enter_combat<P, M>(player: &mut P, mob: &mut M) -> CombatRounds 
where 
    P: Combatant + HasGold + HasProgression,
    M: Combatant + DropsGold + GivesXP,
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
        println!("You died!");
    } else if !mob.is_alive() {
        let _gold = award_kill_gold(player, mob);
        let xp = mob.give_xp();
        player.gain_xp(xp);

    }
    cr
}

pub fn start_fight(mob_kind: MobKind) {
    let gs = game_state();
    let mut mob = gs.spawn_mob(mob_kind);
    let combat_rounds = enter_combat(&mut gs.player, &mut mob);
    gs.set_current_combat(combat_rounds);
    gs.current_screen = Id::Fight;
}
