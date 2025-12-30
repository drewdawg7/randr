use ratatui::{layout::Rect, Frame};
use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::store::Store;
use crate::ui::components::widgets::fitted_box::FittedBox;
use crate::ui::components::widgets::table::{Header, Table};

pub struct StoreDisplay {
    props: Props,
    table: Table,
}

impl StoreDisplay {
    pub fn new(store: &Store) -> Self {
        let table = Table::from_items(
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

        Self { props: Props::default(), table }
    }
}

impl MockComponent for StoreDisplay {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let width = self.table.content_width();
        let height = self.table.content_height();
        let fitted_table = FittedBox::new(self.table.to_widget(), width, height);
        frame.render_widget(fitted_table, area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for StoreDisplay {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}
