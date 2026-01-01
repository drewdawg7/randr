#![allow(static_mut_refs)]
use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::terminal;
use ratatui::{prelude::CrosstermBackend, Terminal};
use tuirealm::{Application, Event, EventListenerCfg, NoUserEvent};

use crate::field::{Field, FieldId, FieldRegistry};
use crate::item::recipe::RecipeRegistry;
use crate::ui::components::player::inventory_modal::InventoryModal;
use crate::{
    combat::{ActiveCombat, CombatRounds},
    entities::{mob::{MobId, MobRegistry}, Mob, Player},
    item::{consumable::ConsumableRegistry, ItemId, ItemRegistry},
    mine::rock::{Rock, RockId, RockRegistry},
    town::definition::Town,
    ui::{
        Id,
        components::mine::screen::MineScreen,
        main_menu::MainMenuScreen,
        fight::FightScreen,
        modal_wrapper::ModalWrapper,
        profile::PlayerProfile,
        town::TownScreen,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalType {
    #[default]
    None,
    Keybinds,
    Inventory,
}

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
    rock_registry: RockRegistry,
    field_registry: FieldRegistry,
    consumable_registry: ConsumableRegistry,
    recipe_registry: RecipeRegistry,
    pub current_screen: Id,
    app: Application<Id, Event<NoUserEvent>, NoUserEvent>,
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    pub player: Player,
    pub town: Town,
    current_combat: Option<CombatRounds>,
    pub active_combat: Option<ActiveCombat>,
    pub active_modal: ModalType,
    pub inventory_modal: InventoryModal,
    pub show_item_details: bool,
}

impl GameState {
    pub fn spawn_mob(&self, mob: MobId) -> Mob {
        self.mob_registry.spawn(mob)
    }

    pub fn spawn_item(&self, item: ItemId) -> crate::item::Item {
        self.item_registry.spawn(item)
    }

    pub fn spawn_rock(&self, rock: RockId) -> Rock {
        self.rock_registry.spawn(rock)
    }

    pub fn spawn_field(&self, field: FieldId) -> Field {
        self.field_registry.spawn(field)
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

    pub fn blacksmith(&self) -> &crate::location::Blacksmith {
        &self.town.blacksmith
    }

    pub fn store(&self) -> &crate::location::Store {
        &self.town.store
    }

    pub fn store_mut(&mut self) -> &mut crate::location::Store {
        &mut self.town.store
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
        let profile = ModalWrapper::new(PlayerProfile::new());
        let _ = self.app.mount(Id::Profile, Box::new(profile), vec![]);
    }

    pub fn run_current_screen(&mut self) -> std::io::Result<()> {
        self.town.store.check_and_restock();

        let current = self.current_screen;
        if current == Id::Quit {
            return Ok(());
        }
        if current != Id::Fight {
            self.current_combat = None;
            self.active_combat = None;
        }

        let mut terminal = self.terminal.take().expect("terminal missing");
        terminal.draw(|frame| {
            use ratatui::widgets::Block;
            use ratatui::style::Style;
            use crate::ui::theme as colors;

            frame.render_widget(
                Block::default().style(Style::default().bg(colors::BACKGROUND)),
                frame.area()
            );
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

        let store = crate::location::Store::default();
        let blacksmith = crate::location::Blacksmith::new("Village Blacksmith".to_string(), 10, 50);
        let mine = crate::mine::Mine::default();
        let field_registry = FieldRegistry::new();
        let field = field_registry.spawn(FieldId::VillageField);
        let town = Town::new("Village".to_string(), store, blacksmith, field, mine);

        Self {
            player: Player::default(),
            item_registry: ItemRegistry::new(),
            mob_registry: MobRegistry::new(),
            rock_registry: RockRegistry::new(),
            field_registry,
            consumable_registry: ConsumableRegistry::new(),
            recipe_registry: RecipeRegistry::new(),
            town,
            app: Application::init(
                EventListenerCfg::default()
                    .crossterm_input_listener(Duration::from_millis(20), 3)
                    .poll_timeout(Duration::from_millis(10)),
            ),
            terminal: Some(terminal),
            current_screen: Id::Menu,
            current_combat: None,
            active_combat: None,
            active_modal: ModalType::None,
            inventory_modal: InventoryModal::new(),
            show_item_details: false,
        }
    }
}
