use std::cell::RefCell;
use std::rc::Rc;
use ratatui::layout::{Rect};
use ratatui::Frame;
use tuirealm::Application;
use tuirealm::event::NoUserEvent;
use tuirealm::Event;

use crate::combat::CombatRounds;
use crate::ui::fight_component::FightComponent;
use crate::ui::common::{Id, Screen, ScreenId};

pub struct FightScreen {
    selected_screen: Rc<RefCell<Option<ScreenId>>>,
}

impl FightScreen {
    pub fn new(
        app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>,
        combat_rounds: CombatRounds,
    ) -> Self {
        let _ = app.umount(&Id::Fight);
        let selected_screen: Rc<RefCell<Option<ScreenId>>> = Rc::new(RefCell::new(None));

        let fight_component = FightComponent::new(combat_rounds, selected_screen.clone());

        app.mount(Id::Fight, Box::new(fight_component), vec![]).unwrap();

        Self { selected_screen }
    }


    pub fn view(&self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, frame: &mut Frame, area: Rect) {
        app.view(&Id::Fight, frame, area);
    }
}


impl Screen for FightScreen {
    fn selected_screen_mut(&mut self) -> &mut Rc<RefCell<Option<ScreenId>>> {
        &mut self.selected_screen
    }
    fn active_id(&self) -> Id {
        Id::Fight
    }
}
