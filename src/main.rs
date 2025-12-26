use std::io::{self};
use std::time::Duration;

use crossterm::terminal;
use game::entities::mob::{MobKind, MobRegistry};
use game::entities::{Player};
use game::item::definition::{ItemKind, ItemRegistry};
use game::ui::fightscreen::FightScreen;
use game::ui::menuscreen::MenuScreen;
use game::ui::storescreen::StoreScreen;
use game::ui::common::{Id, Screen, ScreenId};
use ratatui::{Terminal, backend::CrosstermBackend};

use game::inventory::{EquipmentSlot, HasInventory};
use game::store::Store;
use game::combat::enter_combat;
use tuirealm::{Application, Event, EventListenerCfg, NoUserEvent};


fn main() -> std::io::Result<()> {
    let mut player = Player::default();
    let mut app: Application<Id, Event<NoUserEvent>, NoUserEvent> = Application::init(
        EventListenerCfg::default()
            .crossterm_input_listener(Duration::from_millis(20), 3)
            .poll_timeout(Duration::from_millis(10)),
    );
    let m_registry: MobRegistry = MobRegistry::new();
    let i_registry: ItemRegistry = ItemRegistry::new();

    let sword = i_registry.spawn(ItemKind::Sword);

    let store = Store::default();

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
                let mut mob = m_registry.spawn(MobKind::Goblin);
                let combat_rounds = enter_combat(&mut player, &mut mob);
                let mut fight_screen = FightScreen::new(&mut app, combat_rounds);

                terminal.draw(|frame| {
                    fight_screen.view(&mut app, frame, frame.area());
                })?;

                if let Some(screen) = fight_screen.tick(&mut app) {
                    current = screen;
                }
            }
            ScreenId::Quit => break,
        }
    }

    // ---------- CLEANUP ----------
    terminal::disable_raw_mode()
}
