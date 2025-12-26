use tuirealm::{Component, Event, MockComponent, Frame, command::{Cmd, CmdResult}, props::{Attribute, AttrValue}, State, NoUserEvent};
use tuirealm::event::{Key, KeyEvent};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::layout::Rect;
use ratatui::style::{Style, Color};

pub struct MenuItem {
    pub label: String,
    pub action: Box<dyn FnMut()>,
}

pub struct MenuComponent {
    pub list_state: ListState,
    pub items: Vec<MenuItem>,
}

impl MenuComponent {
    pub fn new(items: Vec<MenuItem>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { list_state, items }
    }


}

impl MockComponent for MenuComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if self.list_state.selected() == Some(i) {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };
                let prefix = if self.list_state.selected() == Some(i) { "> " } else { "  " };
                ListItem::new(format!("{}{}", prefix, item.label)).style(style)
            })
            .collect();

        let list = List::new(items);
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn query(&self, _attr: Attribute) -> Option<AttrValue> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {}

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for MenuComponent {
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
