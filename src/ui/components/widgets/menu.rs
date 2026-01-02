use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, Frame, MockComponent, NoUserEvent, State, StateValue};
use tuirealm::event::{Key, KeyEvent};
use ratatui::{style::Style, widgets::{List, ListItem, ListState}};
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};

use crate::ui::components::utilities::{list_move_down, list_move_up, selection_prefix};
use crate::ui::theme as colors;

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
        let text_style = Style::default().fg(colors::WHITE);
        let items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                ListItem::new(Line::from(vec![
                    selection_prefix(self.list_state.selected() == Some(i)),
                    Span::styled(item.label.clone(), text_style),
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
                list_move_up(&mut self.list_state, self.items.len());
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                list_move_down(&mut self.list_state, self.items.len());
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
                list_move_up(&mut self.list_state, self.items.len());
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                list_move_down(&mut self.list_state, self.items.len());
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
