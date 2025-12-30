use ratatui::{layout::Rect, Frame};
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::ui::components::blacksmith::menu::BlacksmithMenu;
use crate::ui::components::store::tab::StoreTab;
use crate::ui::components::wrappers::tabbed_container::{TabEntry, TabbedContainer};
use crate::ui::components::utilities::{ANVIL, STORE};

pub struct TownScreen {
    props: Props,
    tabs: TabbedContainer,
}

impl TownScreen {
    pub fn new() -> Self {
        let tabs = TabbedContainer::new(vec![
            TabEntry::new(format!("{} Store", STORE), StoreTab::new()),
            TabEntry::new(format!("{} Blacksmith", ANVIL), BlacksmithMenu::default()),
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
