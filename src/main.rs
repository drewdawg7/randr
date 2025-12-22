use game::combat::{enter_combat};
use game::entities::mob::{MobKind, MobRegistry};
use game::entities::progression::HasProgression;
use game::entities::{Mob, Player, Progression};
mod menu;

use game::inventory::{EquipmentSlot, HasInventory, Inventory};
use game::item::definition::{ItemKind, ItemRegistry};
use menu::{run_menu, MenuChoice};

fn main() -> std::io::Result<()> {
    
    let mut player = Player {
        health: 100,
        max_health: 100,
        attack: 12,
        gold: 0,
        name: "Drew",
        prog: Progression::new(),
        inventory: Inventory::new(),

    };
    let m_registry = MobRegistry::new();
    let i_registry = ItemRegistry::new();
    let sword = i_registry.spawn(ItemKind::Sword);
    player.equip_item(sword, EquipmentSlot::Weapon);
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

        println!("{}", player.pretty_print());
        println!("\nPress Enter to return to menu...");
        let _ = std::io::stdin().read_line(&mut String::new());
    }
        Ok(())
}






