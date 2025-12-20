use game::combat::{Combatant, Named, AttackResult, enter_combat};
use game::entities::{Player, Mob};

fn main() {
    let mut player = Player {
        health: 100,
        attack: 12,
        gold: 0,
        name: "Drew".into()
    };

    let mut mobs = Mob::spawn_mobs(2);
    println!("Welcome, {}!", player.name);
    while let Some(mut mob) = mobs.pop() {
        enter_combat(&mut player, &mut mob);
    }
    println!("{:?}", player);
}






