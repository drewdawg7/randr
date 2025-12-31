use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    Frame,
};
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::ui::components::blacksmith::tab::BlacksmithTab;
use crate::ui::components::field::FieldTab;
use crate::ui::components::store::tab::StoreTab;
use crate::ui::components::widgets::border::BorderTheme;
use crate::ui::components::wrappers::tabbed_container::{TabEntry, TabbedContainer};
use crate::ui::components::utilities::{ANVIL, CROSSED_SWORDS, STORE};
use crate::ui::theme::ColorExt;

pub struct TownScreen {
    props: Props,
    tabs: TabbedContainer,
}

impl TownScreen {
    pub fn new() -> Self {
        use crate::ui::theme;

        let tabs = TabbedContainer::new(vec![
            TabEntry::with_border(
                Line::from(vec![
                    Span::styled(format!("{}", STORE), Style::default().color(theme::YELLOW)),
                    Span::styled(" Store", Style::default().color(theme::WHITE)),
                ]),
                StoreTab::new(),
                BorderTheme::Wood,
            ),
            TabEntry::with_border(
                Line::from(vec![
                    Span::styled(format!("{}", ANVIL), Style::default().color(theme::RED)),
                    Span::styled(" Blacksmith", Style::default().color(theme::ORANGE)),
                ]),
                BlacksmithTab::new(),
                BorderTheme::Ember,
            ),
            TabEntry::with_border(
                Line::from(vec![
                    Span::styled(format!("{}", CROSSED_SWORDS), Style::default().color(theme::WHITE)),
                    Span::styled(" Field", Style::default().color(theme::GREEN)),
                ]),
                FieldTab::new(),
                BorderTheme::Forest,
            ),
        ]);
        Self {
            props: Props::default(),
            tabs,
        }
    }
}

impl MockComponent for TownScreen {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.tabs.view(frame, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        self.tabs.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.tabs.perform(cmd)
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for TownScreen {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.tabs.on(ev)
    }
}
