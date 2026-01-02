use ratatui::{
    layout::Rect,
    widgets::ListState,
    Frame,
};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::{
    combat::HasGold,
    inventory::HasInventory,
    location::sell_player_item,
    system::game_state,
    ui::components::player::item_details::render_item_details_beside,
    ui::components::utilities::{
        collect_player_items, render_location_header, store_header,
    },
    ui::components::widgets::item_list::{InventoryFilter, ItemList, ItemListConfig, SellableItem, StoreBuyItem},
    ui::theme as colors,
};

use super::{menu, wood_planks_art, StateChange};

#[derive(Debug, Clone, Copy, PartialEq)]
enum StoreState {
    Menu,
    Buy,
    Sell,
}

pub struct StoreTab {
    props: Props,
    state: StoreState,
    menu_list_state: ListState,
    buy_list: ItemList<StoreBuyItem, InventoryFilter>,
    sell_list: ItemList<SellableItem, InventoryFilter>,
}

impl StoreTab {
    pub fn new() -> Self {
        let mut menu_list_state = ListState::default();
        menu_list_state.select(Some(0));

        let buy_config = ItemListConfig {
            show_filter_button: true,
            show_scroll_indicators: true,
            visible_count: 10,
            show_back_button: true,
            back_label: "Back",
            background: None,
        };

        let sell_config = ItemListConfig {
            show_filter_button: true,
            show_scroll_indicators: true,
            visible_count: 10,
            show_back_button: true,
            back_label: "Back",
            background: None,
        };

        Self {
            props: Props::default(),
            state: StoreState::Menu,
            menu_list_state,
            buy_list: ItemList::new(buy_config),
            sell_list: ItemList::new(sell_config),
        }
    }

    fn reset_selection(&mut self) {
        self.menu_list_state.select(Some(0));
        self.buy_list.reset_selection();
        self.sell_list.reset_selection();
    }

    fn rebuild_buy_items(&mut self) {
        let store = game_state().store();
        let items: Vec<StoreBuyItem> = store
            .inventory
            .iter()
            .map(|si| StoreBuyItem {
                store_item: si.clone(),
                item_name: game_state().get_item_name(si.item_id),
            })
            .collect();
        self.buy_list.set_items(items);
    }

    fn rebuild_sell_items(&mut self) {
        let items: Vec<SellableItem> = collect_player_items()
            .into_iter()
            .map(|inv_item| SellableItem { inv_item })
            .collect();
        self.sell_list.set_items(items);
    }
}

impl MockComponent for StoreTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        match self.state {
            StoreState::Menu => {
                // Render background first, then menu on top
                wood_planks_art::render_wood_planks(frame, area);
                menu::render(frame, area, &mut self.menu_list_state);
            }
            StoreState::Buy => self.render_buy(frame, area),
            StoreState::Sell => self.render_sell(frame, area),
        }
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(
            match self.state {
                StoreState::Menu => self.menu_list_state.selected().unwrap_or(0),
                StoreState::Buy => self.buy_list.selected_index(),
                StoreState::Sell => self.sell_list.selected_index(),
            }
        ))
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match self.state {
            StoreState::Menu => self.handle_menu_cmd(cmd),
            StoreState::Buy => self.handle_buy_cmd(cmd),
            StoreState::Sell => self.handle_sell_cmd(cmd),
        }
    }
}

impl StoreTab {
    fn render_buy(&mut self, frame: &mut Frame, area: Rect) {
        self.rebuild_buy_items();

        let store = game_state().store();
        let player_gold = game_state().player.gold();

        // Render header and get remaining area
        let header_lines = store_header(store, player_gold);
        let content_area =
            render_location_header(frame, area, header_lines, colors::STORE_BG, colors::WOOD_BROWN);

        // Render the item list with back button
        self.buy_list.render(frame, content_area);

        // Render item details beside list if toggled on
        let selected_item = self.buy_list.selected_item().and_then(|si| si.store_item.display_item());
        render_item_details_beside(frame, content_area, selected_item);
    }

    fn render_sell(&mut self, frame: &mut Frame, area: Rect) {
        self.rebuild_sell_items();

        let player_gold = game_state().player.gold();
        let store = game_state().store();

        // Render header and get remaining area
        let header_lines = store_header(store, player_gold);
        let content_area =
            render_location_header(frame, area, header_lines, colors::STORE_BG, colors::WOOD_BROWN);

        // Render the item list with back button
        self.sell_list.render(frame, content_area);

        // Render item details beside list if toggled on
        let selected_item = self.sell_list.selected_item().map(|si| &si.inv_item.item);
        render_item_details_beside(frame, content_area, selected_item);
    }

    fn handle_menu_cmd(&mut self, cmd: Cmd) -> CmdResult {
        let (result, state_change) = menu::handle(cmd, &mut self.menu_list_state);

        if let Some(change) = state_change {
            match change {
                StateChange::ToBuy => {
                    self.state = StoreState::Buy;
                    self.buy_list.reset_selection();
                }
                StateChange::ToSell => {
                    self.state = StoreState::Sell;
                    self.sell_list.reset_selection();
                }
                StateChange::ToMenu => {
                    self.state = StoreState::Menu;
                    self.reset_selection();
                }
            }
        }

        result
    }

    fn handle_buy_cmd(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                self.buy_list.move_up();
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                self.buy_list.move_down();
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                if self.buy_list.is_back_selected() {
                    // Back
                    self.state = StoreState::Menu;
                    self.reset_selection();
                } else if let Some(buy_item) = self.buy_list.selected_item() {
                    // Find the index of this item in the store inventory
                    let item_id = buy_item.store_item.item_id;
                    let gs = game_state();
                    if let Some(idx) = gs.store().inventory.iter().position(|si| si.item_id == item_id) {
                        let store = &mut gs.town.store;
                        let player = &mut gs.player;
                        match store.purchase_item(player, idx) {
                            Ok(item) => gs.toasts.success(format!("Purchased {}!", item.name)),
                            Err(e) => {
                                use crate::location::StoreError;
                                let msg = match e {
                                    StoreError::OutOfStock => "Out of stock",
                                    StoreError::NotEnoughGold => "Not enough gold",
                                    StoreError::InventoryFull => "Inventory is full",
                                    StoreError::InvalidIndex => "Item not found",
                                };
                                gs.toasts.error(msg);
                            }
                        }
                    }
                }
                CmdResult::Submit(self.state())
            }
            Cmd::Cancel => {
                self.state = StoreState::Menu;
                self.reset_selection();
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }

    fn handle_sell_cmd(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                self.sell_list.move_up();
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                self.sell_list.move_down();
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                if self.sell_list.is_back_selected() {
                    // Back
                    self.state = StoreState::Menu;
                    self.reset_selection();
                } else if let Some(sell_item) = self.sell_list.selected_item() {
                    let gs = game_state();
                    let item_name = sell_item.inv_item.item.name;
                    sell_player_item(&mut gs.player, &sell_item.inv_item.item);
                    gs.toasts.success(format!("Sold {}!", item_name));
                }
                CmdResult::Submit(self.state())
            }
            Cmd::Cancel => {
                self.state = StoreState::Menu;
                self.reset_selection();
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for StoreTab {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(tuirealm::command::Direction::Up));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                self.perform(Cmd::Move(tuirealm::command::Direction::Down));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                self.perform(Cmd::Submit);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                self.perform(Cmd::Cancel);
                None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('E'),
                ..
            }) => {
                // Shift+E to equip/unequip in sell mode
                if self.state == StoreState::Sell {
                    if let Some(sell_item) = self.sell_list.selected_item() {
                        let item = &sell_item.inv_item.item;
                        let item_uuid = item.item_uuid;
                        if let Some(slot) = item.item_type.equipment_slot() {
                            if item.is_equipped {
                                let _ = game_state().player.unequip_item(slot);
                            } else {
                                game_state().player.equip_from_inventory(item_uuid, slot);
                            }
                        }
                    }
                }
                None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('L'),
                modifiers: KeyModifiers::SHIFT,
            }) => {
                // Shift+L to toggle lock in sell mode
                if self.state == StoreState::Sell {
                    if let Some(sell_item) = self.sell_list.selected_item() {
                        let item_uuid = sell_item.inv_item.item.item_uuid;
                        if let Some(inv_item) = game_state().player.find_item_by_uuid_mut(item_uuid) {
                            inv_item.item.toggle_lock();
                        }
                    }
                }
                None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('d'),
                ..
            }) => {
                // Toggle item details in buy/sell mode
                if self.state == StoreState::Buy || self.state == StoreState::Sell {
                    let gs = game_state();
                    gs.show_item_details = !gs.show_item_details;
                }
                None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('f') | Key::Char('F'),
                ..
            }) => {
                // Cycle filter in buy/sell mode
                if self.state == StoreState::Buy {
                    self.buy_list.cycle_filter();
                } else if self.state == StoreState::Sell {
                    self.sell_list.cycle_filter();
                }
                None
            }
            // Pass unhandled events back to parent (for tab switching)
            _ => Some(ev),
        }
    }
}
