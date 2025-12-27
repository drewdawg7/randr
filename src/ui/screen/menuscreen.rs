use tuirealm::{Application, Event, NoUserEvent};
use ratatui::layout::Rect;
use ratatui::Frame;

use crate::ui::{common::{Id, Screen}, components::main_menu::MainMenu};

pub struct MenuScreen {}

impl MenuScreen {
    pub fn new(app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>) -> Self {
        app.mount(Id::Menu, Box::new(MainMenu::new()), vec![]).unwrap();
        Self {}
    }


    pub fn view(&self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, frame: &mut Frame, area: Rect) {
        app.view(&Id::Menu, frame, area);
    }
}

impl Screen for MenuScreen {
    fn active_id(&self) -> Id {
        Id::Menu
    }
}
