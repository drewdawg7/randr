use std::cell::RefCell;
use std::rc::Rc;

use tuirealm::{Application, Event, NoUserEvent};
use ratatui::layout::{Rect};
use ratatui::Frame;

use crate::store::Store;
use crate::ui::common::{Id, Screen, ScreenId};
use crate::ui::store_component::StoreComponent;

pub struct StoreScreen {
    selected_screen: Rc<RefCell<Option<ScreenId>>>,
}


impl StoreScreen {
    pub fn new(app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, store: &Store) -> Self {
        let selected_screen: Rc<RefCell<Option<ScreenId>>> = Rc::new(RefCell::new(None));

        let store_component = StoreComponent::new(store, selected_screen.clone());

        app.mount(Id::Store, Box::new(store_component), vec![]).unwrap();

        Self { selected_screen }
    }

    pub fn view(&self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, frame: &mut Frame, area: Rect) {
        app.view(&Id::Store, frame, area);
    }
}

impl Screen for StoreScreen {
    fn selected_screen_mut(&mut self) -> &mut Rc<RefCell<Option<ScreenId>>> {
        &mut self.selected_screen
    }

    fn active_id(&self) -> Id {
        Id::Store
    }
}
