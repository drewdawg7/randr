use std::{cell::RefCell, rc::Rc};

use tuirealm::{Application, NoUserEvent, Event};
pub enum ScreenId {
    Menu,
    Store,
    Fight,
    Quit
}

#[derive(Eq, PartialEq, Clone)]
pub enum UIAction {
    Up,
    Down,
    Activate,
    Back,
    Quit,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    Menu,
    Store,
    StoreTable,
    StoreBack,
    AttackResult,
    Fight,
    FightBack,
}
pub trait Screen {
    fn active_id(&self) -> Id;
    fn selected_screen_mut(&mut self) -> &mut Rc<RefCell<Option<ScreenId>>>;
    fn tick(&mut self,  app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>) -> Option<ScreenId> {
        app.active(&self.active_id()).unwrap();
        let _ = app.tick(tuirealm::PollStrategy::BlockCollectUpTo(1));
        self.selected_screen_mut().borrow_mut().take()
    }
}



