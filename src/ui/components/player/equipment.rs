use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
};
use tuirealm::{command::{Cmd, CmdResult}, event::{Key, KeyEvent, KeyModifiers}, Component, Event, MockComponent, NoUserEvent, Props, State, StateValue};

use crate::{inventory::{EquipmentSlot, HasInventory}, item::Item, system::game_state, ui::{utilities::{CHECKED, UNCHECKED, RETURN_ARROW}, Id}};
use crate::ui::components::utilities::{item_display, lock_prefix, selection_prefix};

use super::item_details::render_item_details;
use crate::ui::components::wrappers::with_action::WithAction;

pub struct EquipmentItem {
    pub item: Item,
    pub slot: EquipmentSlot,
}

impl MockComponent for EquipmentItem {
    fn view(&mut self, _frame: &mut ratatui::Frame, _area: ratatui::prelude::Rect) {}
    fn query(&self, _attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> { None }
    fn attr(&mut self, _attr: tuirealm::Attribute, _value: tuirealm::AttrValue) {}
    fn state(&self) -> State { State::None }
    fn perform(&mut self, _cmd: Cmd) -> CmdResult { CmdResult::None }
}

impl EquipmentItem {
    pub fn new(item: Item, slot: EquipmentSlot) -> WithAction<Self> {
        let item_uuid = item.item_uuid;
        let is_equipped = item.is_equipped;
        let inner = Self { item, slot };

        if is_equipped {
            WithAction::new(inner, move || {
                let _ = game_state().player.unequip_item(slot);
            })
        } else {
            WithAction::new(inner, move || {
                game_state().player.equip_from_inventory(item_uuid, slot);
            })
        }
    }
}

pub struct Equipment {
    props: Props,
    items: Vec<WithAction<EquipmentItem>>,
    list_state: ListState
}


impl Default for Equipment {
    fn default() -> Self {
        
        let props = Props::default();
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let items = Vec::new();
        Self {
            props,
            items,
            list_state
        }
    }
}


impl MockComponent for Equipment {
    fn view(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        self.items.clear();

        if let Some(inv_item) = game_state().player.get_equipped_item(EquipmentSlot::Weapon) {
            self.items.push(EquipmentItem::new(inv_item.item.clone(), EquipmentSlot::Weapon));
        }

        if let Some(inv_item) = game_state().player.get_equipped_item(EquipmentSlot::OffHand) {
            self.items.push(EquipmentItem::new(inv_item.item.clone(), EquipmentSlot::OffHand));
        }
        for inv_item in game_state().player.get_inventory_items().iter() {
            let item = &inv_item.item;
            if let Some(slot) = item.item_type.equipment_slot() {
                self.items.push(EquipmentItem::new(item.clone(), slot));
            }
        }

        // Split into left (list) and right (details) panels
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let selected = self.list_state.selected().unwrap_or(0);
        let mut list_items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let inner = item.inner();
                let checkbox = if inner.item.is_equipped { CHECKED } else { UNCHECKED };
                ListItem::new(Line::from(vec![
                    selection_prefix(selected == i),
                    Span::raw(format!("{} ", checkbox)),
                    lock_prefix(&inner.item),
                    item_display(&inner.item, None),
                ]))
            })
            .collect();

        // Add back button
        let back_selected = selected == self.items.len();
        list_items.push(ListItem::new(Line::from(vec![
            selection_prefix(back_selected),
            Span::raw(format!("{} Back", RETURN_ARROW)),
        ])));

        let list = List::new(list_items);
        frame.render_stateful_widget(list, chunks[0], &mut self.list_state);

        // Render item details panel on the right (auto-compares to equipped item)
        let selected_item = if selected < self.items.len() {
            Some(&self.items[selected].inner().item)
        } else {
            None
        };
        render_item_details(frame, chunks[1], selected_item);
    }

    fn query(&self, attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: tuirealm::Attribute, value: tuirealm::AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> tuirealm::State {
        State::One(StateValue::Usize(self.list_state.selected().unwrap_or(0)))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        let total_items = self.items.len() + 1; // equipment + back button
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
                if selected == self.items.len() {
                    // Back button
                    game_state().current_screen = Id::Menu;
                } else if selected < self.items.len() {
                    self.items[selected].perform(Cmd::Submit);
                }
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::None
        }
    }

}

impl Component<Event<NoUserEvent>, NoUserEvent> for Equipment {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        let total_items = self.items.len() + 1; // equipment + back button
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = if current == 0 { total_items - 1 } else { current - 1 };
                self.list_state.select(Some(new_idx));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = (current + 1) % total_items;
                self.list_state.select(Some(new_idx));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                let selected = self.list_state.selected().unwrap_or(0);
                if selected == self.items.len() {
                    // Back button
                    game_state().current_screen = Id::Menu;
                } else if selected < self.items.len() {
                    self.items[selected].perform(Cmd::Submit);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Char('E'), .. }) => {
                // Shift+E to equip/unequip
                let selected = self.list_state.selected().unwrap_or(0);
                if selected < self.items.len() {
                    self.items[selected].perform(Cmd::Submit);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Char('L'), modifiers: KeyModifiers::SHIFT }) => {
                // Shift+L to toggle lock
                let selected = self.list_state.selected().unwrap_or(0);
                if selected < self.items.len() {
                    let item_uuid = self.items[selected].inner().item.item_uuid;
                    if let Some(inv_item) = game_state().player.find_item_by_uuid_mut(item_uuid) {
                        inv_item.item.toggle_lock();
                    }
                }
                None
            }
            _ => None
        }
    }
}
