#![allow(static_mut_refs)]
use std::io::{self, Stdout};
use std::panic;
use std::time::Duration;

use crossterm::terminal;

/// RAII guard that ensures terminal is restored to normal mode on drop.
/// This handles both normal exit and panic scenarios.
pub struct TerminalGuard;

impl TerminalGuard {
    /// Creates a new TerminalGuard, enabling raw mode and setting up a panic hook.
    pub fn new() -> io::Result<Self> {
        // Set up panic hook to restore terminal before printing panic info
        let original_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            // Restore terminal first
            let _ = terminal::disable_raw_mode();
            // Then call original panic handler to print the error
            original_hook(panic_info);
        }));

        terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}
use ratatui::{prelude::CrosstermBackend, Terminal};
use tuirealm::{Application, Event, EventListenerCfg, NoUserEvent};

use crate::dungeon::Dungeon;
use crate::magic::effect::PassiveEffect;
use crate::location::{Alchemist, Blacksmith, Field, LocationData, LocationId, Mine, Store};
use crate::ui::screen::ScreenLifecycle;
use crate::ui::state::UIState;
use crate::{
    combat::{ActiveCombat, CombatRounds},
    mob::Mob,
    item::consumable::ConsumableRegistry,
    player::Player,
    town::definition::Town,
    ui::{
        Id,
        components::mine::screen::MineScreen,
        components::screens::dungeon::DungeonScreen,
        main_menu::MainMenuScreen,
        fight::FightScreen,
        modal_wrapper::ModalWrapper,
        profile::PlayerProfile,
        town::TownScreen,
    },
};

/// Tracks where combat was initiated from
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CombatSource {
    #[default]
    Field,
    Dungeon,
}

// Re-export ModalType from ui::state for backward compatibility
pub use crate::ui::state::ModalType;

static mut GAME_STATE: Option<GameState> = None;

pub fn init_game_state(gs: GameState) {
    unsafe { GAME_STATE = Some(gs); }
}

pub fn game_state() -> &'static mut GameState {
    unsafe {
        GAME_STATE
            .as_mut()
            .expect("game_state() called before init_game_state() - ensure initialization in main()")
    }
}

pub struct GameState {
    // Registries (only consumable_registry remains - others replaced by direct spawning)
    consumable_registry: ConsumableRegistry,

    // UI state (grouped for better organization)
    /// UI-specific state containing screen, modals, and toasts.
    pub ui: UIState,

    // Framework state
    app: Application<Id, Event<NoUserEvent>, NoUserEvent>,
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,

    // Game state
    pub player: Player,
    pub town: Town,
    pub dungeon: Option<Dungeon>,
    pub combat_source: CombatSource,
    current_combat: Option<CombatRounds>,
    pub active_combat: Option<ActiveCombat>,
}

impl GameState {
    pub fn consumable_registry(&self) -> &ConsumableRegistry {
        &self.consumable_registry
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

    pub fn start_combat(&mut self, mob: Mob) {
        self.active_combat = Some(ActiveCombat::new(mob));
    }

    pub fn active_combat(&self) -> Option<&ActiveCombat> {
        self.active_combat.as_ref()
    }

    pub fn active_combat_mut(&mut self) -> Option<&mut ActiveCombat> {
        self.active_combat.as_mut()
    }

    pub fn end_combat(&mut self) {
        self.active_combat = None;
    }

    /// Enter the dungeon - generates if not already generated
    pub fn enter_dungeon(&mut self) {
        if self.dungeon.is_none() {
            let mut dungeon = Dungeon::default();
            dungeon.generate();

            // Check for DungeonReveal passive effect
            let has_reveal = self
                .player
                .tome_passive_effects()
                .iter()
                .any(|e| matches!(e, PassiveEffect::DungeonReveal));

            if has_reveal {
                dungeon.reveal_all_rooms();
            }

            self.dungeon = Some(dungeon);
        }
        self.ui.current_screen = Id::Dungeon;
    }

    /// Leave the dungeon and return to town
    pub fn leave_dungeon(&mut self) {
        self.ui.current_screen = Id::Town;
    }

    /// Reset the dungeon (for when completed or explicit reset)
    pub fn reset_dungeon(&mut self) {
        self.dungeon = None;
    }

    /// Get reference to the active dungeon
    pub fn dungeon(&self) -> Option<&Dungeon> {
        self.dungeon.as_ref()
    }

    /// Get mutable reference to the active dungeon
    pub fn dungeon_mut(&mut self) -> Option<&mut Dungeon> {
        self.dungeon.as_mut()
    }

    pub fn blacksmith(&self) -> &crate::location::Blacksmith {
        &self.town.blacksmith
    }

    pub fn store(&self) -> &crate::location::Store {
        &self.town.store
    }

    pub fn store_mut(&mut self) -> &mut crate::location::Store {
        &mut self.town.store
    }

    pub fn alchemist(&self) -> &crate::location::Alchemist {
        &self.town.alchemist
    }

    pub fn storage(&self) -> &crate::storage::Storage {
        &self.town.storage
    }

    pub fn storage_mut(&mut self) -> &mut crate::storage::Storage {
        &mut self.town.storage
    }

    /// Get the screen lifecycle tracker.
    pub fn screen_lifecycle(&self) -> &ScreenLifecycle {
        &self.ui.lifecycle
    }

    /// Get mutable access to the screen lifecycle tracker.
    pub fn screen_lifecycle_mut(&mut self) -> &mut ScreenLifecycle {
        &mut self.ui.lifecycle
    }

    pub fn initialize(&mut self) {

        // Populate store with initial stock (must happen after game_state is set)
        self.town.store.restock();

        let menu = ModalWrapper::new(MainMenuScreen::default());
        let _ = self.app.mount(Id::Menu, Box::new(menu), vec![]);
        let town = ModalWrapper::new(TownScreen::new());
        let _ = self.app.mount(Id::Town, Box::new(town), vec![]);
        let fight = ModalWrapper::new(FightScreen::new());
        let _ = self.app.mount(Id::Fight, Box::new(fight), vec![]);
        let mine = ModalWrapper::new(MineScreen::new());
        let _ = self.app.mount(Id::Mine, Box::new(mine), vec![]);
        let dungeon = ModalWrapper::new(DungeonScreen::new());
        let _ = self.app.mount(Id::Dungeon, Box::new(dungeon), vec![]);
        let profile = ModalWrapper::new(PlayerProfile::new());
        let _ = self.app.mount(Id::Profile, Box::new(profile), vec![]);
    }

    pub fn run_current_screen(&mut self) -> std::io::Result<()> {
        self.town.store.check_and_restock();
        self.town.mine.check_and_regenerate();
        self.town.mine.check_and_respawn_rock();
        self.ui.toasts.cleanup();

        let current = self.ui.current_screen;
        if current == Id::Quit {
            return Ok(());
        }

        // Update screen lifecycle to detect transitions
        self.ui.lifecycle.update(current);

        if current != Id::Fight {
            self.current_combat = None;
            self.active_combat = None;
        }

        let mut terminal = self.terminal.take().expect("terminal missing");
        let toasts = self.ui.toasts.toasts();
        terminal.draw(|frame| {
            use ratatui::widgets::Block;
            use ratatui::style::Style;
            use crate::ui::theme as colors;
            use crate::toast::render::render_toasts;

            frame.render_widget(
                Block::default().style(Style::default().bg(colors::BACKGROUND)),
                frame.area()
            );
            self.app.view(&current, frame, frame.area());
            render_toasts(frame, toasts);
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

        let store = {
            let id = LocationId::VillageStore;
            let spec = id.spec();
            match &spec.data {
                LocationData::Store(data) => Store::from_spec(id, spec, data),
                _ => unreachable!(),
            }
        };
        let blacksmith = {
            let id = LocationId::VillageBlacksmith;
            let spec = id.spec();
            match &spec.data {
                LocationData::Blacksmith(data) => Blacksmith::from_spec(id, spec, data),
                _ => unreachable!(),
            }
        };
        let alchemist = {
            let id = LocationId::VillageAlchemist;
            let spec = id.spec();
            match &spec.data {
                LocationData::Alchemist(data) => Alchemist::from_spec(id, spec, data),
                _ => unreachable!(),
            }
        };
        let field = {
            let id = LocationId::VillageField;
            let spec = id.spec();
            match &spec.data {
                LocationData::Field(data) => Field::from_spec(id, spec, data),
                _ => unreachable!(),
            }
        };
        let mine = {
            let id = LocationId::VillageMine;
            let spec = id.spec();
            match &spec.data {
                LocationData::Mine(data) => Mine::from_spec(id, spec, data),
                _ => unreachable!(),
            }
        };
        let town = Town::new("Village".to_string(), store, blacksmith, alchemist, field, mine);

        Self {
            // Registries (only consumable_registry remains)
            consumable_registry: ConsumableRegistry::new(),

            // UI state
            ui: UIState::new(),

            // Framework state
            app: Application::init(
                EventListenerCfg::default()
                    .crossterm_input_listener(Duration::from_millis(20), 3)
                    .poll_timeout(Duration::from_millis(10)),
            ),
            terminal: Some(terminal),

            // Game state
            player: Player::default(),
            town,
            dungeon: None,
            combat_source: CombatSource::default(),
            current_combat: None,
            active_combat: None,
        }
    }
}
