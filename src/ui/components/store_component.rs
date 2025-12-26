use ratatui::layout::{Constraint, Direction, Layout};
use tuirealm::{command::CmdResult, Component, Event, MockComponent, NoUserEvent, State};

use crate::store::Store;
use crate::ui::common::ScreenId;
use crate::ui::components::utilities::back_button;
use crate::ui::table::{Header, TableComponent};
use crate::ui::{menu_component::*};
pub struct StoreComponent {
    table: TableComponent,
    back_menu: MenuComponent,
}

impl StoreComponent {
    pub fn new(store: &Store, back_screen: ScreenId) -> Self {
        let table = TableComponent::from_items(
            [
                Header::new("Item"),
                Header::new("Price"),
                Header::new("Quantity"),
            ],
            &store.inventory,
            |si| [
                si.item.name.to_string(),
                format!("{}g", si.price),
                si.quantity.to_string(),
            ],
        );

        let back_menu = back_button(back_screen);
        Self { table, back_menu }
    }
}

impl MockComponent for StoreComponent {
    fn view(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        self.table.view(frame, chunks[0]);
        self.back_menu.view(frame, chunks[1]);
    }

    fn query(&self, _attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        None
    }

    fn attr(&mut self, _attr: tuirealm::Attribute, _value: tuirealm::AttrValue) {}

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: tuirealm::command::Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for StoreComponent {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.back_menu.on(ev)
    }
}
