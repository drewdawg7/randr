use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, Frame, MockComponent, NoUserEvent, State, StateValue};
use tuirealm::event::{Key, KeyEvent};
use ratatui::{widgets::{List, ListItem, ListState}};
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};

use crate::ui::components::utilities::selection_prefix;

pub struct MenuItem {
    pub label: String,
    pub action: Box<dyn FnMut()>,
}

pub struct Menu {
    props: Props,
    list_state: ListState,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { props: Props::default(), list_state, items }
    }

}

impl MockComponent for Menu {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                ListItem::new(Line::from(vec![
                    selection_prefix(self.list_state.selected() == Some(i)),
                    Span::raw(item.label.clone()),
                ]))
            })
            .collect();

        let list = List::new(items);
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
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

impl Component<Event<NoUserEvent>, NoUserEvent> for Menu {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = if current == 0 {
                    self.items.len() - 1
                } else {
                    current - 1
                };
                self.list_state.select(Some(new_idx));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = (current + 1) % self.items.len();
                self.list_state.select(Some(new_idx));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                let selected = self.list_state.selected().unwrap_or(0);
                let action = &mut self.items[selected].action;
                action();
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                None
            }
            _ => None
        }
    }
}
