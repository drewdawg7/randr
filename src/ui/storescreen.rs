use std::cell::RefCell;
use std::rc::Rc;

use tuirealm::{Application, Event, NoUserEvent};
use ratatui::layout::{Rect, Layout, Direction, Constraint};
use ratatui::Frame;

use crate::store::Store;
use crate::ui::{Header, Id, ScreenId, TableComponent};
use super::menu_component::{MenuComponent, MenuItem};

pub struct StoreScreen {
    selected_screen: Rc<RefCell<Option<ScreenId>>>,
}

impl StoreScreen {
    pub fn new(app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, store: &Store) -> Self {
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

        let selected_screen: Rc<RefCell<Option<ScreenId>>> = Rc::new(RefCell::new(None));
        let back_screen = Rc::clone(&selected_screen);

        let back_menu = MenuComponent::new(vec![
            MenuItem {
                label: "Back".to_string(),
                action: Box::new(move || {
                    *back_screen.borrow_mut() = Some(ScreenId::Menu);
                }),
            },
        ]);

        app.mount(Id::StoreTable, Box::new(table), vec![]).unwrap();
        app.mount(Id::StoreBack, Box::new(back_menu), vec![]).unwrap();

        Self { selected_screen }
    }

    pub fn tick(&mut self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>) -> Option<ScreenId> {
        app.active(&Id::StoreBack).unwrap();
        let _ = app.tick(tuirealm::PollStrategy::Once);
        self.selected_screen.borrow_mut().take()
    }

    pub fn view(&self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(area);

        app.view(&Id::StoreTable, frame, chunks[0]);
        app.view(&Id::StoreBack, frame, chunks[1]);
    }
}
