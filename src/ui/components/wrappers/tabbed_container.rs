use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

/// A tab entry with a label and content component
pub struct TabEntry {
    pub label: String,
    pub content: Box<dyn MockComponent>,
}

impl TabEntry {
    pub fn new<C: MockComponent + 'static>(label: impl Into<String>, content: C) -> Self {
        Self {
            label: label.into(),
            content: Box::new(content),
        }
    }
}

/// A reusable tabbed container component
pub struct TabbedContainer {
    props: Props,
    tabs: Vec<TabEntry>,
    active_tab: usize,
}

impl TabbedContainer {
    pub fn new(tabs: Vec<TabEntry>) -> Self {
        Self {
            props: Props::default(),
            tabs,
            active_tab: 0,
        }
    }

    fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    fn switch_tab(&mut self, delta: i32) {
        let count = self.tab_count();
        if count == 0 {
            return;
        }
        let new_idx = (self.active_tab as i32 + delta).rem_euclid(count as i32) as usize;
        self.active_tab = new_idx;
    }
}

impl MockComponent for TabbedContainer {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Layout: [Tabs (height 2)] [Content (flex)]
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Tab bar
                Constraint::Min(0),    // Content area
            ])
            .split(area);

        // Render tab bar with styled spans
        let tab_spans: Vec<Span> = self.tabs
            .iter()
            .enumerate()
            .flat_map(|(i, tab)| {
                let is_selected = i == self.active_tab;
                let tab_style = if is_selected {
                    Style::default().on_color(colors::BLUE).color(colors::WHITE).bold()
                } else {
                    Style::default().color(colors::DARK_GRAY)
                };
                let label = format!(" {} ", tab.label);

                if i == 0 {
                    vec![Span::styled(label, tab_style)]
                } else {
                    vec![
                        Span::styled(" | ", Style::default().color(colors::DARK_GRAY)),
                        Span::styled(label, tab_style),
                    ]
                }
            })
            .collect();

        let tabs_line = Line::from(tab_spans);
        frame.render_widget(Paragraph::new(tabs_line), chunks[0]);

        // Render active tab content
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.content.view(frame, chunks[1]);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.active_tab))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        // Forward commands to active tab content
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.content.perform(cmd)
        } else {
            CmdResult::None
        }
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for TabbedContainer {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            // Tab switching with Left/Right
            Event::Keyboard(KeyEvent { code: Key::Left, .. }) => {
                self.switch_tab(-1);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Right, .. }) => {
                self.switch_tab(1);
                None
            }
            // Forward Up/Down/Enter to active tab content
            Event::Keyboard(KeyEvent { code: Key::Up, .. })
            | Event::Keyboard(KeyEvent { code: Key::Down, .. })
            | Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    // Convert event to command for MockComponent
                    let cmd = match ev {
                        Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                            Cmd::Move(tuirealm::command::Direction::Up)
                        }
                        Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                            Cmd::Move(tuirealm::command::Direction::Down)
                        }
                        Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => Cmd::Submit,
                        _ => return None,
                    };
                    tab.content.perform(cmd);
                }
                None
            }
            _ => None,
        }
    }
}
