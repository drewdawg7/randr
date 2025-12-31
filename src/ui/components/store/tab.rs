use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
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
    inventory::{HasInventory, InventoryItem},
    loot::WorthGold,
    store::sell_player_item,
    system::game_state,
    ui::Id,
};
use crate::ui::components::player::item_details::render_item_details_with_price;
use crate::ui::components::utilities::{collect_player_items, item_display, list_move_down, list_move_up, lock_prefix, render_location_header, selection_prefix, store_header, RETURN_ARROW};
use crate::ui::theme as colors;

#[derive(Debug, Clone, Copy, PartialEq)]
enum StoreState {
    Menu,
    Buy,
    Sell,
}

pub struct StoreTab {
    props: Props,
    state: StoreState,
    list_state: ListState,
}

impl StoreTab {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            props: Props::default(),
            state: StoreState::Menu,
            list_state,
        }
    }

    fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
    }

    fn get_player_items(&self) -> Vec<InventoryItem> {
        collect_player_items()
    }
}

impl MockComponent for StoreTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        match self.state {
            StoreState::Menu => self.render_menu(frame, area),
            StoreState::Buy => self.render_buy(frame, area),
            StoreState::Sell => self.render_sell(frame, area),
        }
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.list_state.selected().unwrap_or(0)))
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
    fn render_menu(&mut self, frame: &mut Frame, area: Rect) {
        let store = game_state().store();
        let player_gold = game_state().player.gold();

        // Render header with store name and gold, get remaining area
        let header_lines = store_header(store, player_gold);
        let content_area = render_location_header(frame, area, header_lines, colors::STORE_BG, colors::WOOD_BROWN);

        // Menu options
        let selected = self.list_state.selected().unwrap_or(0);
        let menu_items: Vec<ListItem> = ["Buy", "Sell", "Back"]
            .iter()
            .enumerate()
            .map(|(i, label)| {
                let icon = if *label == "Back" { format!("{} ", RETURN_ARROW) } else { String::new() };
                ListItem::new(Line::from(vec![
                    selection_prefix(selected == i),
                    Span::raw(format!("{}{}", icon, label)),
                ]))
            })
            .collect();

        let menu = List::new(menu_items);
        frame.render_stateful_widget(menu, content_area, &mut self.list_state);
    }

    fn render_buy(&mut self, frame: &mut Frame, area: Rect) {
        let store = game_state().store();
        let player_gold = game_state().player.gold();

        // Render header and get remaining area
        let header_lines = store_header(store, player_gold);
        let content_area = render_location_header(frame, area, header_lines, colors::STORE_BG, colors::WOOD_BROWN);

        // Split into left (list) and right (details) panels
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_area);

        // Build list of store items + back
        let selected = self.list_state.selected().unwrap_or(0);
        let store_items = &store.inventory;
        let mut list_items: Vec<ListItem> = store_items
            .iter()
            .enumerate()
            .map(|(i, si)| {
                let available = if si.quantity > 0 { "" } else { " (Out of stock)" };
                ListItem::new(Line::from(vec![
                    selection_prefix(selected == i),
                    lock_prefix(&si.item),
                    item_display(&si.item, Some(si.quantity as u32)),
                    Span::raw(format!(" - {}g{}", si.purchase_price(), available)),
                ]))
            })
            .collect();

        // Add back option
        let back_selected = selected == store_items.len();
        list_items.push(ListItem::new(Line::from(vec![
            selection_prefix(back_selected),
            Span::raw(format!("{} Back", RETURN_ARROW)),
        ])));

        let list = List::new(list_items);
        frame.render_stateful_widget(list, panels[0], &mut self.list_state);

        // Render item details (auto-compares to equipped item)
        let selected_item = if selected < store_items.len() {
            let si = &store_items[selected];
            Some((&si.item, si.purchase_price()))
        } else {
            None
        };

        match selected_item {
            Some((item, price)) => {
                render_item_details_with_price(frame, panels[1], Some(item), Some((price, "Buy")));
            }
            None => {
                render_item_details_with_price(frame, panels[1], None, None);
            }
        }
    }

    fn render_sell(&mut self, frame: &mut Frame, area: Rect) {
        let player_gold = game_state().player.gold();
        let store = game_state().store();

        // Render header and get remaining area
        let header_lines = store_header(store, player_gold);
        let content_area = render_location_header(frame, area, header_lines, colors::STORE_BG, colors::WOOD_BROWN);

        // Split into left (list) and right (details) panels
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_area);

        // Build list of player items + back
        let player_items = self.get_player_items();
        let selected = self.list_state.selected().unwrap_or(0);

        let mut list_items: Vec<ListItem> = player_items
            .iter()
            .enumerate()
            .map(|(i, inv_item)| {
                ListItem::new(Line::from(vec![
                    selection_prefix(selected == i),
                    lock_prefix(&inv_item.item),
                    item_display(&inv_item.item, Some(inv_item.quantity)),
                    Span::raw(format!(" - {}g", inv_item.item.sell_price())),
                ]))
            })
            .collect();

        // Add back option
        let back_selected = selected == player_items.len();
        list_items.push(ListItem::new(Line::from(vec![
            selection_prefix(back_selected),
            Span::raw(format!("{} Back", RETURN_ARROW)),
        ])));

        let list = List::new(list_items);
        frame.render_stateful_widget(list, panels[0], &mut self.list_state);

        // Render item details
        let selected_item = if selected < player_items.len() {
            Some(&player_items[selected].item)
        } else {
            None
        };

        match selected_item {
            Some(item) => {
                render_item_details_with_price(
                    frame,
                    panels[1],
                    Some(item),
                    Some((item.sell_price(), "Sell")),
                );
            }
            None => {
                render_item_details_with_price(frame, panels[1], None, None);
            }
        }
    }

    fn handle_menu_cmd(&mut self, cmd: Cmd) -> CmdResult {
        const MENU_SIZE: usize = 3; // Buy, Sell, Back
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                list_move_up(&mut self.list_state, MENU_SIZE);
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                list_move_down(&mut self.list_state, MENU_SIZE);
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                let selected = self.list_state.selected().unwrap_or(0);
                match selected {
                    0 => {
                        // Buy
                        self.state = StoreState::Buy;
                        self.reset_selection();
                    }
                    1 => {
                        // Sell
                        self.state = StoreState::Sell;
                        self.reset_selection();
                    }
                    2 => {
                        // Back
                        game_state().current_screen = Id::Menu;
                    }
                    _ => {}
                }
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::None,
        }
    }

    fn handle_buy_cmd(&mut self, cmd: Cmd) -> CmdResult {
        let store_len = game_state().store().inventory.len();
        let total_items = store_len + 1; // items + back

        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                list_move_up(&mut self.list_state, total_items);
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                list_move_down(&mut self.list_state, total_items);
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                let selected = self.list_state.selected().unwrap_or(0);
                if selected == store_len {
                    // Back
                    self.state = StoreState::Menu;
                    self.reset_selection();
                } else if selected < store_len {
                    // Purchase item - need to avoid borrow conflicts
                    let gs = game_state();
                    let store_item = gs.store_mut().inventory.get(selected).cloned();
                    if let Some(store_item) = store_item {
                        let player_gold = gs.player.gold();
                        let item_cost = store_item.purchase_price();
                        if player_gold >= item_cost && store_item.quantity > 0 {
                            // Deduct gold and quantity
                            gs.player.dec_gold(item_cost);
                            gs.store_mut().inventory[selected].dec_quantity(1);
                            // Add to inventory
                            let _ = gs.player.add_to_inv(store_item.item.clone());
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
        let player_items = self.get_player_items();
        let total_items = player_items.len() + 1; // items + back

        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                list_move_up(&mut self.list_state, total_items);
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                list_move_down(&mut self.list_state, total_items);
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                let selected = self.list_state.selected().unwrap_or(0);
                if selected == player_items.len() {
                    // Back
                    self.state = StoreState::Menu;
                    self.reset_selection();
                } else if selected < player_items.len() {
                    let inv_item = &player_items[selected];
                    let gs = game_state();
                    sell_player_item(&mut gs.player, &inv_item.item);
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
            Event::Keyboard(KeyEvent { code: Key::Char('E'), .. }) => {
                // Shift+E to equip/unequip in sell mode
                if self.state == StoreState::Sell {
                    let player_items = self.get_player_items();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if selected < player_items.len() {
                        let inv_item = &player_items[selected];
                        let item = &inv_item.item;
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
            Event::Keyboard(KeyEvent { code: Key::Char('L'), modifiers: KeyModifiers::SHIFT }) => {
                // Shift+L to toggle lock in sell mode
                if self.state == StoreState::Sell {
                    let player_items = self.get_player_items();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if selected < player_items.len() {
                        let item_uuid = player_items[selected].item.item_uuid;
                        if let Some(inv_item) = game_state().player.find_item_by_uuid_mut(item_uuid) {
                            inv_item.item.toggle_lock();
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}
