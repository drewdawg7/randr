#![allow(static_mut_refs)]
use std::{collections::HashMap, io::{self, Stdout},  time::Duration};

use crossterm::terminal;
use ratatui::{prelude::{CrosstermBackend}, Terminal};
use tuirealm::{Application, Event, EventListenerCfg, NoUserEvent};

use crate::{combat::CombatRounds, entities::{mob::{MobKind, MobRegistry}, Mob}, item::{definition::{ItemKind, ItemRegistry}, Item}, store::Store, ui::{common::{Id, ScreenId, ScreenKind}, fightscreen::FightScreen, menuscreen::MenuScreen, storescreen::StoreScreen}};


static mut GAME_STATE: Option<GameState> = None;
pub fn init_game_state(gs: GameState) {
    unsafe { GAME_STATE = Some(gs); }
}

pub fn game_state() -> &'static mut GameState {
    unsafe { GAME_STATE.as_mut().unwrap() }
}
pub struct GameState {
   item_registry: ItemRegistry,
   mob_registry: MobRegistry,
   pub current_screen: ScreenId,
   screens: HashMap<ScreenId, ScreenKind>,
   app: Application<Id, Event<NoUserEvent>, NoUserEvent>,
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
}

impl GameState {
    pub fn spawn_mob(&self, mob: MobKind) -> Mob {
        self.mob_registry.spawn(mob)
    }

    pub fn spawn_item(&self, item: ItemKind) -> Item {
        self.item_registry.spawn(item)
    }

    fn add_screen(&mut self, screen: ScreenKind) {
        let id = screen.id();
        self.screens.insert(id, screen);

    }
    pub fn app_mut(&mut self) -> &mut Application<Id, Event<NoUserEvent>, NoUserEvent> {
        &mut self.app
    }
 
    pub fn init_fight(&mut self, combat_rounds: CombatRounds) {
        let fight_screen = self.screens.get_mut(&ScreenId::Fight);
        if let Some(sk) = fight_screen && let ScreenKind::Fight(fs) = sk {fs.add_fight(combat_rounds);}
    }
    pub fn initialize(&mut self) {
        let _ = terminal::enable_raw_mode();
        self.init_screens();
    } 

    pub fn run_current_screen(&mut self, current: &mut ScreenId) -> std::io::Result<()> {
        let mut screen = self.screens.remove(current).expect("missing screen");

        let mut terminal = self.terminal.take().expect("terminal missing");

        terminal.draw(|frame| {
            screen.view(self, frame, frame.area());
        })?;

        self.terminal = Some(terminal);

        if let Some(next) = screen.tick(self) {
            *current = next;
        }

        self.screens.insert(screen.id(), screen);
        Ok(())
    }
    fn init_screens(&mut self) {
        let menu_screen = MenuScreen::new(&mut self.app);
        self.add_screen(ScreenKind::MainMenu(menu_screen));
        let store = Store::default();
        let store_screen = StoreScreen::new(&mut self.app, &store);
        self.add_screen(ScreenKind::Store(store_screen));
        let fight_screen = FightScreen::new(&mut self.app, CombatRounds::new());
        self.add_screen(ScreenKind::Fight(fight_screen));
    
    }
}

impl Default for GameState {
    fn default() -> Self {
          let stdout: io::Stdout = io::stdout();
        let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
        let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();
        Self {
            item_registry: ItemRegistry::new(),
            mob_registry: MobRegistry::new(),
            screens: HashMap::new(),
            app: Application::init(
                EventListenerCfg::default()
                    .crossterm_input_listener(Duration::from_millis(20), 3)
                    .poll_timeout(Duration::from_millis(10)),
            ),
            terminal: Some(terminal),
            current_screen: ScreenId::Menu,
        }
    }
}
