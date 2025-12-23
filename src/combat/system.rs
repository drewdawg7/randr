use crate::{combat::{AttackResult, Combatant, DropsGold, HasGold}, entities::progression::{GivesXP, HasProgression}};


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
    let target_died = defender.is_alive();
    AttackResult {
        damage_to_target,
        target_health_before,
        target_health_after,
        target_died
    }
}



pub fn enter_combat<P, M>(player: &mut P, mob: &mut M) 
where 
    P: Combatant + HasGold + HasProgression,
    M: Combatant + DropsGold + GivesXP,
{
    while player.is_alive() && mob.is_alive() {
        let a1 = attack(player, mob);
        println!(
            "{} did {} damage to {}. {} HP remaining.",
            player.name(),
            a1.damage_to_target,
            mob.name(),
            mob.health()
        );
        let a2 = attack(mob, player);
        println!(
                    "{} did {} damage to {}. {} HP remaining.",
                    mob.name(),
                    a2.damage_to_target,
                    player.name(),
                    player.health()
        );
    }
    if !player.is_alive() {
        println!("You died!");
    } else if !mob.is_alive() {
        let gold = award_kill_gold(player, mob);
        let xp = mob.give_xp();
        player.gain_xp(xp);
        println!("{} was slain. You have recieved {} gold.", mob.name(), gold);
        
    }
}
