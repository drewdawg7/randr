use game::combat::{Named, enter_combat};
use game::entities::mob::{MobKind, MobRegistry};
use game::entities::{Player, Mob};
mod menu;

use game::item::{ItemKind, ItemRegistry};
use menu::{run_menu, MenuChoice};
fn main() -> std::io::Result<()> {
    
    let mut player = Player {
        health: 100,
        attack: 12,
        gold: 0,
        name: "Drew",
    };
    let m_registry = MobRegistry::new();
    let i_registry = ItemRegistry::new();
    let sword = i_registry.spawn(ItemKind::Sword);
    println!("item: {:?}", sword);
    while let MenuChoice::Fight = run_menu()? {

        let mut mobs: Vec<Mob> = Vec::new();
        let m1 = m_registry.spawn(MobKind::Slime);
        let m2 = m_registry.spawn(MobKind::Goblin);
        mobs.push(m1);
        mobs.push(m2);

        println!("Welcome, {}!", player.name);

        while let Some(mut mob) = mobs.pop() {
            enter_combat(&mut player, &mut mob);
        }

        println!("{:?}", player);
        println!("\nPress Enter to return to menu...");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
        Ok(())
}






