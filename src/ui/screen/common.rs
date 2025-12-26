
use ratatui::{layout::Rect, Frame};
use tuirealm::{Application, NoUserEvent, Event};

use crate::{system::{game_state, GameState}, ui::{fightscreen::FightScreen, menuscreen::MenuScreen, storescreen::StoreScreen}};
#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
pub enum ScreenId {
    Menu,
    Store,
    Fight,
    Quit
}

pub enum ScreenKind {
    MainMenu(MenuScreen),
    Store(StoreScreen),
    Fight(FightScreen)
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


impl ScreenKind {
    pub fn id(&self) -> ScreenId {
        match self {
            ScreenKind::MainMenu(_) => ScreenId::Menu,
            ScreenKind::Store(_)    => ScreenId::Store,
            ScreenKind::Fight(_)    => ScreenId::Fight,
        }
    }
    pub fn view(&mut self, gs: &mut GameState, frame: &mut Frame, area: Rect) {
        match self {
            ScreenKind::MainMenu(s) => s.view(gs.app_mut(), frame, area),
            ScreenKind::Store(s) => s.view(gs.app_mut(), frame, area),
            ScreenKind::Fight(s) => s.view(gs.app_mut(), frame, area),
        }
    }

    pub fn tick(&mut self, gs: &mut GameState) -> Option<ScreenId> {
        match self {
            ScreenKind::MainMenu(s) => s.tick(gs.app_mut()),
            ScreenKind::Store(s) => s.tick(gs.app_mut()),
            ScreenKind::Fight(s) => s.tick(gs.app_mut()),
        }
    }
}
pub trait Screen {
    fn active_id(&self) -> Id;
    fn tick(&mut self,  app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>) -> Option<ScreenId> {
        app.active(&self.active_id()).unwrap();
        let _ = app.tick(tuirealm::PollStrategy::BlockCollectUpTo(1));
        Some(game_state().current_screen)
    }
}



