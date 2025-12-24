use std::collections::HashMap;

use game::combat::{enter_combat};
use game::entities::mob::{MobKind, MobRegistry};
use game::entities::{Mob, Player, Progression};
mod menu;

use game::inventory::{EquipmentSlot, HasInventory, Inventory};
use game::item::definition::{ItemKind, ItemRegistry};
use game::stats::{StatInstance, StatSheet, StatType};
use game::store::{Store};
use game::ui::{key_to_action, MenuScreen, Screen, ScreenId, StoreScreen};


fn main() -> std::io::Result<()> {
    use crossterm::{event, terminal};
    use std::io::{self, Write};


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
                    max_value: 12,
                },
            );
            stats.insert(
                StatType::Health,
                StatInstance {
                    stat_type: StatType::Health,
                    current_value: 100,
                    max_value: 100,
                },
            );
            StatSheet { stats }
        },
    };

    let m_registry: MobRegistry = MobRegistry::new();
    let i_registry: ItemRegistry = ItemRegistry::new();

    let sword = i_registry.spawn(ItemKind::Sword);
    let dagger = i_registry.spawn(ItemKind::Dagger);

    let mut store = Store::new("The Shop");
    store.add_item(&sword);
    store.add_item(&sword);
    store.add_item(&dagger);

    player.equip_item(sword, EquipmentSlot::Weapon);


    let mut menu_screen = MenuScreen::new(
        vec!["Fight".to_string(), "Store".to_string(), "Quit".to_string()]
    );
    let mut store_screen = StoreScreen::new(store);

    let mut current = ScreenId::Menu;

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    game::ui::enter_alternate_screen(&mut stdout);

    // ---------- MAIN LOOP ----------

    loop {
        match current {
            ScreenId::Menu => {
                menu_screen.draw(&mut stdout);
                stdout.flush()?;

                if let Some(action) = key_to_action(event::read()?) {
                    current = menu_screen.handle(action);
                }
            }

            ScreenId::Store => {
                store_screen.draw(&mut stdout);
                stdout.flush()?;

                if let Some(action) = key_to_action(event::read()?) {
                    current = store_screen.handle(action);
                }
            }

            ScreenId::Fight => {
                // ---------- GAMEPLAY PHASE ----------
                let _ = terminal::disable_raw_mode();
                let mut mobs: Vec<Mob> = Vec::new();
                mobs.push(m_registry.spawn(MobKind::Slime));
                mobs.push(m_registry.spawn(MobKind::Goblin));

                println!("Welcome, {}!", player.name);

                while let Some(mut mob) = mobs.pop() {
                    enter_combat(&mut player, &mut mob);
                }

                println!("{}", player.pretty_print());
                println!("\nPress Enter to return to menu...");
                let _ = std::io::stdin().read_line(&mut String::new());
                let _ = terminal::enable_raw_mode();
                current = ScreenId::Menu;
            }

            ScreenId::Quit => break,
        }
    }

    // ---------- CLEANUP ----------

    game::ui::leave_alternate_screen(&mut stdout);
    terminal::disable_raw_mode()
}





