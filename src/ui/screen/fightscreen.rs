use ratatui::layout::{Rect};
use ratatui::Frame;
use tuirealm::Application;
use tuirealm::event::NoUserEvent;
use tuirealm::Event;

use crate::combat::CombatRounds;
use crate::ui::fight_component::FightComponent;
use crate::ui::common::{Id, Screen, ScreenId};

pub struct FightScreen {
    pub combat_rounds: CombatRounds,
}

impl FightScreen {
    pub fn new(
        _app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>,
        combat_rounds: CombatRounds,
    ) -> Self {
        
        Self { combat_rounds: combat_rounds.clone() }
    }
    

    pub fn add_fight(&mut self, combat_rounds: CombatRounds) {
        self.combat_rounds = combat_rounds;
    }
    pub fn view(&self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, frame: &mut Frame, area: Rect) {

        let _ = app.umount(&Id::Fight);
    
        let fight_component = FightComponent::new(self.combat_rounds.clone(), ScreenId::Menu);
        
        app.mount(Id::Fight, Box::new(fight_component), vec![]).unwrap();
        app.view(&Id::Fight, frame, area);
    }
}


impl Screen for FightScreen {
    fn active_id(&self) -> Id {
        Id::Fight
    }
}
