use tuirealm::{Application, Event, NoUserEvent};
use ratatui::layout::{Rect};
use ratatui::Frame;

use crate::store::Store;
use crate::ui::common::{Id, Screen, ScreenId};
use crate::ui::store_component::StoreComponent;

pub struct StoreScreen {}


impl StoreScreen {
    pub fn new(app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, store: &Store) -> Self {

        let store_component = StoreComponent::new(store, ScreenId::Menu);

        app.mount(Id::Store, Box::new(store_component), vec![]).unwrap();

        Self {}
    }

    pub fn view(&self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, frame: &mut Frame, area: Rect) {
        app.view(&Id::Store, frame, area);
    }
}

impl Screen for StoreScreen {
    fn active_id(&self) -> Id {
        Id::Store
    }
}
