use tuirealm::{Application, Event, NoUserEvent};
use ratatui::layout::Rect;
use ratatui::Frame;


use crate::{system::game_state, ui::{common::{Id, Screen, ScreenId}, menu_component::{MenuComponent, MenuItem}}};


pub struct MenuScreen {}

impl MenuScreen {
    pub fn new(app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>) -> Self {

        let items = vec![
            MenuItem {
                label: "Fight".to_string(),
                action: Box::new(|| {
                    game_state().current_screen = ScreenId::Fight;
                })
            },
            MenuItem {
                label: "Store".to_string(),
                action: Box::new(|| {
                    game_state().current_screen = ScreenId::Store;
                })
            },
            MenuItem {
                label: "Quit".to_string(),
                action: Box::new(|| {
                    game_state().current_screen = ScreenId::Quit;
                })
            },
        ];

        app.mount(Id::Menu, Box::new(MenuComponent::new(items)), vec![]).unwrap();

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
