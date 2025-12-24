use std::collections::HashMap;
use std::io::{self};
use std::time::Duration;

use crossterm::terminal;
use game::entities::mob::{MobKind, MobRegistry};
use game::entities::{Mob, Player, Progression};
use game::item::definition::{ItemKind, ItemRegistry};
use ratatui::{Terminal, backend::CrosstermBackend};

use game::ui::{ Id, MenuScreen, ScreenId, StoreScreen};
use game::inventory::{EquipmentSlot, HasInventory, Inventory};
use game::stats::{StatSheet, StatType, StatInstance};
use game::store::Store;
use game::combat::enter_combat;
use tuirealm::{Application, Event, EventListenerCfg, NoUserEvent};


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
    let mut app: Application<Id, Event<NoUserEvent>, NoUserEvent> = Application::init(
        EventListenerCfg::default()
            .crossterm_input_listener(Duration::from_millis(20), 3)
            .poll_timeout(Duration::from_millis(10)),
    );
    let m_registry: MobRegistry = MobRegistry::new();
    let i_registry: ItemRegistry = ItemRegistry::new();

    let sword = i_registry.spawn(ItemKind::Sword);
    let dagger = i_registry.spawn(ItemKind::Dagger);

    let mut store = Store::new("The Shop");
    store.add_item(&sword);
    store.add_item(&sword);
    store.add_item(&dagger);

    player.equip_item(sword, EquipmentSlot::Weapon);

    let mut menu_screen = MenuScreen::new(&mut app);
    let mut store_screen = StoreScreen::new(&mut app, &store);

    let mut current = ScreenId::Menu;

    terminal::enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // ---------- MAIN LOOP ----------
    loop {
        match current {
            ScreenId::Menu => {
                terminal.draw(|frame| {
                    menu_screen.view(&mut app, frame, frame.area());
                })?;

                if let Some(screen) = menu_screen.tick(&mut app) {
                    current = screen;
                }
            }

            ScreenId::Store => {
                terminal.draw(|frame| {
                    store_screen.view(&mut app, frame, frame.area());
                })?;

                if let Some(screen) = store_screen.tick(&mut app) {
                    current = screen;
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
    terminal::disable_raw_mode()
}
