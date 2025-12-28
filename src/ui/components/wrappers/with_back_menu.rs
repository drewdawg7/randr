use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;
use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::ui::Id;
use crate::ui::components::screens::menu_component::MenuComponent;
use crate::ui::components::utilities::back_button;

pub struct WithBackMenu<C: MockComponent> {
    props: Props,
    content: C,
    back_menu: MenuComponent,
}

impl<C: MockComponent> WithBackMenu<C> {
    pub fn new(content: C, back_screen: Id) -> Self {
        Self {
            props: Props::default(),
            content,
            back_menu: back_button(back_screen),
        }
    }
}

impl<C: MockComponent> MockComponent for WithBackMenu<C> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        self.content.view(frame, chunks[0]);
        self.back_menu.view(frame, chunks[1]);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr).or_else(|| self.content.query(attr))
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.content.attr(attr, value);
    }

    fn state(&self) -> State {
        self.back_menu.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.back_menu.perform(cmd)
    }
}

impl<C: MockComponent> Component<Event<NoUserEvent>, NoUserEvent> for WithBackMenu<C> {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.back_menu.on(ev)
    }
}
