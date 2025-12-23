use std::collections::HashMap;

use game::combat::{enter_combat};
use game::entities::mob::{MobKind, MobRegistry};
use game::entities::{Mob, Player, Progression};
mod menu;

use game::inventory::{EquipmentSlot, HasInventory, Inventory};
use game::item::definition::{ItemKind, ItemRegistry};
use game::stats::{self, StatInstance, StatSheet, StatType};
use menu::{run_menu, MenuChoice};

fn main() -> std::io::Result<()> {
    
    let mut player = Player {
        gold: 0,
        name: "Drew",
        prog: Progression::new(),
        inventory: Inventory::new(),

            stats: {
                let mut stats: HashMap<StatType, StatInstance> = HashMap::new();
                stats.insert(
                    StatType::Attack, 
                    StatInstance {
                        stat_type: StatType::Attack,
                        current_value: 12,
                        max_value: 12
                    }

                );
                stats.insert(
                    StatType::Health, 
                    StatInstance {
                        stat_type: StatType::Health,
                        current_value: 100,
                        max_value: 100
                    }

                );

                StatSheet { stats }
            }

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






