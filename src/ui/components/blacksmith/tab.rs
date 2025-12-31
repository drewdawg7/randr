use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
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
    inventory::{EquipmentSlot, HasInventory, InventoryItem},
    system::game_state,
    ui::Id,
};
use crate::ui::components::player::item_details::render_item_details;
use crate::ui::components::utilities::{blacksmith_header, item_display, lock_prefix, render_location_header, selection_prefix, DOUBLE_ARROW_UP, RETURN_ARROW};
use crate::ui::utilities::HAMMER;
use crate::item::enums::{ItemKind, ItemQuality};
use crate::ui::theme as colors;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BlacksmithState {
    Menu,
    Items,
    ItemQuality,
}

pub struct BlacksmithTab {
    props: Props,
    state: BlacksmithState,
    list_state: ListState,
    cached_items: Vec<InventoryItem>,
}

impl BlacksmithTab {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            props: Props::default(),
            state: BlacksmithState::Menu,
            list_state,
            cached_items: Vec::new(),
        }
    }

    fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
    }

    fn rebuild_items(&mut self) {
        self.cached_items.clear();

        // Add equipped items
        for slot in EquipmentSlot::all() {
            if let Some(inv_item) = game_state().player.get_equipped_item(*slot) {
                self.cached_items.push(inv_item.clone());
            }
        }

        // Add inventory items (equipment only - materials can't be upgraded)
        for inv_item in game_state().player.get_inventory_items().iter() {
            if inv_item.item.item_type.is_equipment() {
                self.cached_items.push(inv_item.clone());
            }
        }
    }

    fn upgrade_item_at(&mut self, index: usize) {
        if index >= self.cached_items.len() {
            return;
        }

        let item_uuid = self.cached_items[index].item.item_uuid;
        let gs = game_state();
        let blacksmith = &gs.town.blacksmith;

        // Check equipped items first
        for slot in EquipmentSlot::all() {
            if let Some(equipped) = gs.player.get_equipped_item(*slot) {
                if equipped.item.item_uuid == item_uuid {
                    if let Some(mut inv_item) = gs.player.inventory_mut().equipment_mut().remove(slot) {
                        let _ = blacksmith.upgrade_item(&mut gs.player, &mut inv_item.item);
                        gs.player.inventory_mut().equipment_mut().insert(*slot, inv_item);
                    }
                    return;
                }
            }
        }

        // Check inventory items
        if let Some(idx) = gs.player.find_item_index_by_uuid(item_uuid) {
            let mut inv_item = gs.player.inventory_mut().items.remove(idx);
            let _ = blacksmith.upgrade_item(&mut gs.player, &mut inv_item.item);
            gs.player.inventory_mut().items.insert(idx, inv_item);
        }
    }

    fn upgrade_item_quality_at(&mut self, index: usize) {
        if index >= self.cached_items.len() {
            return;
        }

        let item_uuid = self.cached_items[index].item.item_uuid;
        let gs = game_state();
        let blacksmith = &gs.town.blacksmith;

        // Check equipped items first
        for slot in EquipmentSlot::all() {
            if let Some(equipped) = gs.player.get_equipped_item(*slot) {
                if equipped.item.item_uuid == item_uuid {
                    if let Some(mut inv_item) = gs.player.inventory_mut().equipment_mut().remove(slot) {
                        let _ = blacksmith.upgrade_item_quality(&mut gs.player, &mut inv_item.item);
                        gs.player.inventory_mut().equipment_mut().insert(*slot, inv_item);
                    }
                    return;
                }
            }
        }

        // Check inventory items
        if let Some(idx) = gs.player.find_item_index_by_uuid(item_uuid) {
            let mut inv_item = gs.player.inventory_mut().items.remove(idx);
            let _ = blacksmith.upgrade_item_quality(&mut gs.player, &mut inv_item.item);
            gs.player.inventory_mut().items.insert(idx, inv_item);
        }
    }
}

impl MockComponent for BlacksmithTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        match self.state {
            BlacksmithState::Menu => self.render_menu(frame, area),
            BlacksmithState::Items => self.render_items(frame, area),
            BlacksmithState::ItemQuality => self.render_item_quality(frame, area),
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
            BlacksmithState::Menu => self.handle_menu_cmd(cmd),
            BlacksmithState::Items => self.handle_items_cmd(cmd),
            BlacksmithState::ItemQuality => self.handle_item_quality_cmd(cmd),
        }
    }
}

impl BlacksmithTab {
    fn render_menu(&mut self, frame: &mut Frame, area: Rect) {
        let player_gold = game_state().player.gold();
        let blacksmith = game_state().blacksmith();
        let stones = game_state().player.find_item_by_kind(ItemKind::QualityUpgradeStone)
            .map(|inv| inv.quantity).unwrap_or(0);

        // Render header and get remaining area
        let header_lines = blacksmith_header(blacksmith, player_gold, stones);
        let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

        // Menu options
        let selected = self.list_state.selected().unwrap_or(0);
        let menu_items: Vec<ListItem> = vec![
            ListItem::new(Line::from(vec![
                selection_prefix(selected == 0),
                Span::raw(format!("{} Upgrade Items", HAMMER)),
            ])),
            ListItem::new(Line::from(vec![
                selection_prefix(selected == 1),
                Span::raw(format!("{} Upgrade Item Quality", DOUBLE_ARROW_UP)),
            ])),
            ListItem::new(Line::from(vec![
                selection_prefix(selected == 2),
                Span::raw(format!("{} Back", RETURN_ARROW)),
            ])),
        ];

        let menu = List::new(menu_items);
        frame.render_stateful_widget(menu, content_area, &mut self.list_state);
    }

    fn render_items(&mut self, frame: &mut Frame, area: Rect) {
        self.rebuild_items();

        let player_gold = game_state().player.gold();
        let blacksmith = game_state().blacksmith();
        let max_upgrades = blacksmith.max_upgrades;
        let stones = game_state().player.find_item_by_kind(ItemKind::QualityUpgradeStone)
            .map(|inv| inv.quantity).unwrap_or(0);

        // Render header and get remaining area
        let header_lines = blacksmith_header(blacksmith, player_gold, stones);
        let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

        // Split into left (list) and right (details) panels
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_area);

        let selected = self.list_state.selected().unwrap_or(0);

        let list_items: Vec<ListItem> = self.cached_items
            .iter()
            .enumerate()
            .map(|(i, inv_item)| {
                let item = &inv_item.item;
                let is_selected = selected == i;
                let upgrade_cost = blacksmith.calc_upgrade_cost(item);
                let at_max = item.num_upgrades >= max_upgrades;
                let can_afford = player_gold >= upgrade_cost;

                let line = if at_max {
                    Line::from(vec![
                        selection_prefix(is_selected),
                        lock_prefix(item),
                        item_display(item, None),
                        Span::styled(" - MAX", Style::default().fg(colors::DARK_GRAY)),
                    ])
                } else {
                    let cost_style = if can_afford {
                        Style::default()
                    } else {
                        Style::default().fg(colors::RED)
                    };

                    Line::from(vec![
                        selection_prefix(is_selected),
                        lock_prefix(item),
                        item_display(item, None),
                        Span::raw(" - "),
                        Span::styled(format!("{} gold", upgrade_cost), cost_style),
                    ])
                };

                ListItem::new(line)
            })
            .collect();

        // Add back button
        let back_selected = selected == self.cached_items.len();
        let mut all_items = list_items;
        all_items.push(ListItem::new(Line::from(vec![
            selection_prefix(back_selected),
            Span::raw(format!("{} Back", RETURN_ARROW)),
        ])));

        let list = List::new(all_items);
        frame.render_stateful_widget(list, panels[0], &mut self.list_state);

        // Render item details panel on the right
        let selected_item = if selected < self.cached_items.len() {
            Some(&self.cached_items[selected].item)
        } else {
            None
        };
        render_item_details(frame, panels[1], selected_item);
    }

    fn handle_menu_cmd(&mut self, cmd: Cmd) -> CmdResult {
        const MENU_SIZE: usize = 3; // Upgrade Items, Upgrade Item Quality, Back
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = if current == 0 { MENU_SIZE - 1 } else { current - 1 };
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = (current + 1) % MENU_SIZE;
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                let selected = self.list_state.selected().unwrap_or(0);
                match selected {
                    0 => {
                        // Upgrade Items
                        self.state = BlacksmithState::Items;
                        self.reset_selection();
                    }
                    1 => {
                        // Upgrade Item Quality
                        self.state = BlacksmithState::ItemQuality;
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

    fn handle_items_cmd(&mut self, cmd: Cmd) -> CmdResult {
        self.rebuild_items();
        let total_items = self.cached_items.len() + 1; // items + back

        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = if current == 0 { total_items - 1 } else { current - 1 };
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = (current + 1) % total_items;
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                let selected = self.list_state.selected().unwrap_or(0);
                if selected == self.cached_items.len() {
                    // Back
                    self.state = BlacksmithState::Menu;
                    self.reset_selection();
                } else if selected < self.cached_items.len() {
                    // Upgrade item
                    self.upgrade_item_at(selected);
                }
                CmdResult::Submit(self.state())
            }
            Cmd::Cancel => {
                self.state = BlacksmithState::Menu;
                self.reset_selection();
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }

    fn render_item_quality(&mut self, frame: &mut Frame, area: Rect) {
        self.rebuild_items();

        let player_gold = game_state().player.gold();
        let blacksmith = game_state().blacksmith();
        let stones = game_state().player.find_item_by_kind(ItemKind::QualityUpgradeStone)
            .map(|inv| inv.quantity).unwrap_or(0);

        // Render header and get remaining area
        let header_lines = blacksmith_header(blacksmith, player_gold, stones);
        let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

        // Split into left (list) and right (details) panels
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(content_area);

        let selected = self.list_state.selected().unwrap_or(0);

        let list_items: Vec<ListItem> = self.cached_items
            .iter()
            .enumerate()
            .map(|(i, inv_item)| {
                let item = &inv_item.item;
                let is_selected = selected == i;
                let at_max = item.quality == ItemQuality::Mythic;
                let current_quality_color = colors::quality_color(item.quality);

                let line = if at_max {
                    Line::from(vec![
                        selection_prefix(is_selected),
                        lock_prefix(item),
                        item_display(item, None),
                        Span::raw(" - "),
                        Span::styled(item.quality.display_name(), Style::default().fg(current_quality_color)),
                        Span::styled(" (MAX)", Style::default().fg(colors::DARK_GRAY)),
                    ])
                } else {
                    let next_quality = item.quality.next_quality().unwrap();
                    let next_quality_color = colors::quality_color(next_quality);
                    Line::from(vec![
                        selection_prefix(is_selected),
                        lock_prefix(item),
                        item_display(item, None),
                        Span::raw(" - "),
                        Span::styled(item.quality.display_name(), Style::default().fg(current_quality_color)),
                        Span::raw(" -> "),
                        Span::styled(next_quality.display_name(), Style::default().fg(next_quality_color)),
                    ])
                };

                ListItem::new(line)
            })
            .collect();

        // Add back button
        let back_selected = selected == self.cached_items.len();
        let mut all_items = list_items;
        all_items.push(ListItem::new(Line::from(vec![
            selection_prefix(back_selected),
            Span::raw(format!("{} Back", RETURN_ARROW)),
        ])));

        let list = List::new(all_items);
        frame.render_stateful_widget(list, panels[0], &mut self.list_state);

        // Render item details panel on the right
        let selected_item = if selected < self.cached_items.len() {
            Some(&self.cached_items[selected].item)
        } else {
            None
        };
        render_item_details(frame, panels[1], selected_item);
    }

    fn handle_item_quality_cmd(&mut self, cmd: Cmd) -> CmdResult {
        self.rebuild_items();
        let total_items = self.cached_items.len() + 1; // items + back

        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = if current == 0 { total_items - 1 } else { current - 1 };
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = (current + 1) % total_items;
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                let selected = self.list_state.selected().unwrap_or(0);
                if selected == self.cached_items.len() {
                    // Back
                    self.state = BlacksmithState::Menu;
                    self.reset_selection();
                } else if selected < self.cached_items.len() {
                    // Upgrade item quality
                    self.upgrade_item_quality_at(selected);
                }
                CmdResult::Submit(self.state())
            }
            Cmd::Cancel => {
                self.state = BlacksmithState::Menu;
                self.reset_selection();
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for BlacksmithTab {
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
                // Shift+E to equip/unequip in items mode
                if self.state == BlacksmithState::Items || self.state == BlacksmithState::ItemQuality {
                    self.rebuild_items();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if selected < self.cached_items.len() {
                        let inv_item = &self.cached_items[selected];
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
                // Shift+L to toggle lock in items mode
                if self.state == BlacksmithState::Items || self.state == BlacksmithState::ItemQuality {
                    self.rebuild_items();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if selected < self.cached_items.len() {
                        let item_uuid = self.cached_items[selected].item.item_uuid;
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
