use std::cell::RefCell;
use std::rc::Rc;

use tuirealm::{Application, Event, NoUserEvent};
use ratatui::layout::Rect;
use ratatui::Frame;


use crate::ui::{menu_component::{MenuComponent, MenuItem}, common::{Id, Screen, ScreenId}};


pub struct MenuScreen {
    selected_screen: Rc<RefCell<Option<ScreenId>>>,
}

impl MenuScreen {
    pub fn new(app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>) -> Self {
        let selected_screen: Rc<RefCell<Option<ScreenId>>> = Rc::new(RefCell::new(None));

        let fight_screen = Rc::clone(&selected_screen);
        let store_screen = Rc::clone(&selected_screen);
        let quit_screen = Rc::clone(&selected_screen);

        let items = vec![
            MenuItem {
                label: "Fight".to_string(),
                action: Box::new(move || {
                    *fight_screen.borrow_mut() = Some(ScreenId::Fight);
                }),
            },
            MenuItem {
                label: "Store".to_string(),
                action: Box::new(move || {
                    *store_screen.borrow_mut() = Some(ScreenId::Store);
                }),
            },
            MenuItem {
                label: "Quit".to_string(),
                action: Box::new(move || {
                    *quit_screen.borrow_mut() = Some(ScreenId::Quit);
                }),
            },
        ];

        app.mount(Id::Menu, Box::new(MenuComponent::new(items)), vec![]).unwrap();

        Self { selected_screen }
    }


    pub fn view(&self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, frame: &mut Frame, area: Rect) {
        app.view(&Id::Menu, frame, area);
    }
}

impl Screen for MenuScreen {
    fn active_id(&self) -> Id {
        Id::Menu
    }
    fn selected_screen_mut(&mut self) -> &mut Rc<RefCell<Option<ScreenId>>> {
        &mut self.selected_screen
    }
}
