#![allow(static_mut_refs)]
use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::terminal;
use ratatui::{prelude::CrosstermBackend, Terminal};
use tuirealm::{Application, Event, EventListenerCfg, NoUserEvent};

use crate::{
    combat::CombatRounds,
    entities::{mob::{MobKind, MobRegistry}, Mob, Player},
    item::{ItemKind, definition::ItemRegistry},
    town::definition::Town,
    ui::{
        Id,
        equipment::Equipment,
        main_menu::MainMenuScreen,
        fight::FightScreen,
        profile::PlayerProfile,
        with_back_menu::WithBackMenu,
        items::BlacksmithItems,
        tabbed_container::{TabbedContainer, TabEntry},
        town::TownScreen,
    },
};

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
    pub current_screen: Id,
    app: Application<Id, Event<NoUserEvent>, NoUserEvent>,
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    pub player: Player,
    pub town: Town,
    current_combat: Option<CombatRounds>,
}

impl GameState {
    pub fn spawn_mob(&self, mob: MobKind) -> Mob {
        self.mob_registry.spawn(mob)
    }

    pub fn spawn_item(&self, item: ItemKind) -> crate::item::Item {
        self.item_registry.spawn(item)
    }

    pub fn get_item_name(&self, kind: ItemKind) -> &'static str {
        self.item_registry
            .get(&kind)
            .map(|spec| spec.name)
            .unwrap_or("Unknown")
    }

    pub fn current_combat(&self) -> Option<&CombatRounds> {
        self.current_combat.as_ref()
    }

    pub fn set_current_combat(&mut self, combat_rounds: CombatRounds) {
        self.current_combat = Some(combat_rounds);
    }

    pub fn clear_current_combat(&mut self) {
        self.current_combat = None;
    }

    pub fn blacksmith(&self) -> &crate::blacksmith::Blacksmith {
        &self.town.blacksmith
    }

    pub fn store(&self) -> &crate::store::Store {
        &self.town.store
    }

    pub fn initialize(&mut self) {
        let _ = terminal::enable_raw_mode();

        // Mount all components
        let menu = MainMenuScreen::default();
        let _ = self.app.mount(Id::Menu, Box::new(menu), vec![]);

        // Mount Town screen with Store and Blacksmith tabs
        let _ = self.app.mount(Id::Town, Box::new(TownScreen::new()), vec![]);

        let fight = WithBackMenu::new(FightScreen::new(), Id::Menu);
        let _ = self.app.mount(Id::Fight, Box::new(fight), vec![]);

        // Create tabbed profile with Player and Equipment tabs
        let profile_tabs = TabbedContainer::new(
            vec![
                TabEntry::new("Player", PlayerProfile::new()),
                TabEntry::new("Equipment", Equipment::default()),
            ],
        );
        let _ = self.app.mount(Id::Profile, Box::new(profile_tabs), vec![]);

        let _ = self.app.mount(Id::BlacksmithItems, Box::new(BlacksmithItems::default()), vec![]);
    }

    pub fn run_current_screen(&mut self) -> std::io::Result<()> {
        let current = self.current_screen;
        if current == Id::Quit {
            return Ok(());
        }

        // Clear combat data when leaving fight screen
        if current != Id::Fight {
            self.current_combat = None;
        }

        let mut terminal = self.terminal.take().expect("terminal missing");
        terminal.draw(|frame| {
            self.app.view(&current, frame, frame.area());
        })?;
        self.terminal = Some(terminal);

        let _ = self.app.active(&current);
        let _ = self.app.tick(tuirealm::PollStrategy::Once);

        Ok(())
    }
}

impl Default for GameState {
    fn default() -> Self {
        let stdout: io::Stdout = io::stdout();
        let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
        let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend).unwrap();
        terminal.clear().unwrap();

        let store = crate::store::Store::default();
        let blacksmith = crate::blacksmith::Blacksmith::new("Village Blacksmith".to_string(), 10, 50);
        let town = Town::new("Village".to_string(), store, blacksmith);

        Self {
            player: Player::default(),
            item_registry: ItemRegistry::new(),
            mob_registry: MobRegistry::new(),
            town,
            app: Application::init(
                EventListenerCfg::default()
                    .crossterm_input_listener(Duration::from_millis(20), 3)
                    .poll_timeout(Duration::from_millis(10)),
            ),
            terminal: Some(terminal),
            current_screen: Id::Menu,
            current_combat: None,
        }
    }
}
