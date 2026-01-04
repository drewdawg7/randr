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
use crate::item::recipe::RecipeRegistry;
use crate::magic::effect::PassiveEffect;
use crate::location::spec::specs::{
    VILLAGE_ALCHEMIST, VILLAGE_BLACKSMITH, VILLAGE_FIELD, VILLAGE_MINE, VILLAGE_STORE,
};
use crate::location::{Alchemist, Blacksmith, Field, LocationData, Mine, Store};
use crate::magic::word::WordRegistry;
use crate::toast::ToastQueue;
use crate::ui::components::magic::SpellTestModal;
use crate::ui::components::player::inventory_modal::InventoryModal;
use crate::ui::components::player::profile_modal::ProfileModal;
use crate::ui::screen::ScreenLifecycle;
use crate::ui::state::UIState;
use crate::{
    combat::{ActiveCombat, CombatRounds},
    entities::mob::{MobId, MobRegistry, Mob},
    item::{consumable::ConsumableRegistry, ItemId, ItemRegistry},
    location::mine::{Rock, RockId, RockRegistry},
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
    // Registries
    item_registry: ItemRegistry,
    mob_registry: MobRegistry,
    rock_registry: RockRegistry,
    consumable_registry: ConsumableRegistry,
    recipe_registry: RecipeRegistry,
    word_registry: WordRegistry,

    // UI state (grouped for better organization)
    /// UI-specific state. Prefer using `self.ui` for new code.
    pub ui: UIState,

    // Legacy UI fields (deprecated - use `ui` field instead)
    // These are kept for backward compatibility during migration
    pub current_screen: Id,
    screen_lifecycle: ScreenLifecycle,
    pub active_modal: ModalType,
    pub inventory_modal: InventoryModal,
    pub spell_test_modal: SpellTestModal,
    pub profile_modal: ProfileModal,
    pub show_item_details: bool,
    pub toasts: ToastQueue,

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
    pub fn spawn_mob(&self, mob: MobId) -> Option<Mob> {
        self.mob_registry.spawn(mob)
    }

    pub fn spawn_item(&self, item: ItemId) -> Option<crate::item::Item> {
        self.item_registry.spawn(item)
    }

    pub fn spawn_rock(&self, rock: RockId) -> Option<Rock> {
        self.rock_registry.spawn(rock)
    }

    pub fn get_item_name(&self, kind: ItemId) -> &'static str {
        self.item_registry
            .get(&kind)
            .map(|spec| spec.name)
            .unwrap_or("Unknown")
    }

    pub fn is_item_equipment(&self, kind: ItemId) -> bool {
        self.item_registry
            .get(&kind)
            .map(|spec| spec.item_type.is_equipment())
            .unwrap_or(false)
    }

    pub fn get_rock_name(&self, kind: RockId) -> &'static str {
        self.rock_registry
            .get(&kind)
            .map(|spec| spec.name)
            .unwrap_or("Unknown")
    }

    pub fn consumable_registry(&self) -> &ConsumableRegistry {
        &self.consumable_registry
    }

    pub fn recipe_registry(&self) -> &RecipeRegistry {
        &self.recipe_registry
    }

    pub fn item_registry(&self) -> &ItemRegistry {
        &self.item_registry
    }

    pub fn word_registry(&self) -> &WordRegistry {
        &self.word_registry
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
        self.current_screen = Id::Dungeon;
    }

    /// Leave the dungeon and return to town
    pub fn leave_dungeon(&mut self) {
        self.current_screen = Id::Town;
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
        &self.screen_lifecycle
    }

    /// Get mutable access to the screen lifecycle tracker.
    pub fn screen_lifecycle_mut(&mut self) -> &mut ScreenLifecycle {
        &mut self.screen_lifecycle
    }

    pub fn initialize(&mut self) {
        let _ = terminal::enable_raw_mode();

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
        self.toasts.cleanup();

        let current = self.current_screen;
        if current == Id::Quit {
            return Ok(());
        }

        // Update screen lifecycle to detect transitions
        self.screen_lifecycle.update(current);

        if current != Id::Fight {
            self.current_combat = None;
            self.active_combat = None;
        }

        let mut terminal = self.terminal.take().expect("terminal missing");
        let toasts = self.toasts.toasts();
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

        let store = match &VILLAGE_STORE.data {
            LocationData::Store(data) => Store::from_spec(&VILLAGE_STORE, data),
            _ => unreachable!(),
        };
        let blacksmith = match &VILLAGE_BLACKSMITH.data {
            LocationData::Blacksmith(data) => Blacksmith::from_spec(&VILLAGE_BLACKSMITH, data),
            _ => unreachable!(),
        };
        let alchemist = match &VILLAGE_ALCHEMIST.data {
            LocationData::Alchemist(data) => Alchemist::from_spec(&VILLAGE_ALCHEMIST, data),
            _ => unreachable!(),
        };
        let field = match &VILLAGE_FIELD.data {
            LocationData::Field(data) => Field::from_spec(&VILLAGE_FIELD, data),
            _ => unreachable!(),
        };
        let mine = match &VILLAGE_MINE.data {
            LocationData::Mine(data) => Mine::from_spec(&VILLAGE_MINE, data),
            _ => unreachable!(),
        };
        let town = Town::new("Village".to_string(), store, blacksmith, alchemist, field, mine);

        Self {
            // Registries
            item_registry: ItemRegistry::new(),
            mob_registry: MobRegistry::new(),
            rock_registry: RockRegistry::new(),
            consumable_registry: ConsumableRegistry::new(),
            recipe_registry: RecipeRegistry::new(),
            word_registry: WordRegistry::new(),

            // UI state (new grouped struct)
            ui: UIState::new(),

            // Legacy UI fields (kept for backward compatibility)
            current_screen: Id::Menu,
            screen_lifecycle: ScreenLifecycle::new(Id::Menu),
            active_modal: ModalType::None,
            inventory_modal: InventoryModal::new(),
            spell_test_modal: SpellTestModal::new(),
            profile_modal: ProfileModal::new(),
            show_item_details: false,
            toasts: ToastQueue::default(),

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
