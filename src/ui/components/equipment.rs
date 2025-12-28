use ratatui::{style::{Color, Style}, widgets::{List, ListItem, ListState}};
use tuirealm::{command::{Cmd, CmdResult}, event::{Key, KeyEvent}, Component, Event, MockComponent, NoUserEvent, Props, State, StateValue};

use crate::{inventory::{EquipmentSlot, HasInventory}, item::{definition::ItemType, Item}, system::game_state, ui::{utilities::{B_DIAMOND, CHECKED, UNCHECKED, W_DIAMOND}, Id}};


pub struct EquipmentItem {
    pub item: Item,
    pub action: Box<dyn FnMut()>

}

impl EquipmentItem {
    pub fn new(item: Item, slot: EquipmentSlot) -> Self {
        let kind = item.kind;
        let action: Box<dyn FnMut()> = if item.is_equipped {
            Box::new(move || {
                let _ = game_state().player.unequip_item(slot);
            })
        } else {
            Box::new(move || {
                game_state().player.equip_from_inventory(kind, slot);
            })
        };
        Self { item, action }
    }
}

pub struct Equipment {
    props: Props,
    items: Vec<EquipmentItem>,
    list_state: ListState
}


impl Equipment {}

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

        if let Some(item) = game_state().player.get_equipped_item(EquipmentSlot::Weapon) {
            self.items.push(EquipmentItem::new(*item, EquipmentSlot::Weapon));
        }

        for item in game_state().player.get_inventory_items().iter() {
            if item.item_type == ItemType::Weapon {
                self.items.push(EquipmentItem::new(*item, EquipmentSlot::Weapon));
            }
        }

        let selected = self.list_state.selected().unwrap_or(0);
        let mut list_items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if selected == i {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                let prefix = if selected == i { "> " } else { "  " };
                ListItem::new(
                    format!("{}{} {}", prefix, if item.item.is_equipped {CHECKED} else {UNCHECKED},item.item.name)
                ).style(style)
            })
            .collect();

        // Add back button
        let back_selected = selected == self.items.len();
        let back_style = if back_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let back_prefix = if back_selected { "> " } else { "  " };
        list_items.push(ListItem::new(format!("{}Back", back_prefix)).style(back_style));

        let list = List::new(list_items);
        frame.render_stateful_widget(list, area, &mut self.list_state);
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
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = if current == 0 { self.items.len() - 1 } else { current - 1 };
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = (current + 1) % self.items.len();
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                let selected = self.list_state.selected().unwrap_or(0);
                let action = &mut self.items[selected].action;
                action();
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
                    let action = &mut self.items[selected].action;
                    action();
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                game_state().current_screen = Id::Menu;
                None
            }
            _ => None
        }
    }
}
