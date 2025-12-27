
use tuirealm::{Application, Event, NoUserEvent};
use ratatui::layout::{Rect};
use ratatui::Frame;

use crate::ui::common::{Id, Screen};
use crate::ui::player_profile::PlayerProfile;


pub struct PlayerProfileScreen {}


impl PlayerProfileScreen {
    pub fn new(app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>) -> Self {
        let component = PlayerProfile::new();
        app.mount(Id::PlayerProfile, Box::new(component), vec![]).unwrap();
        //app.mount(Id::Store, Box::new(store_component), vec![]).unwrap();

        Self {}
    }

    pub fn view(&self, app: &mut Application<Id, Event<NoUserEvent>, NoUserEvent>, frame: &mut Frame, area: Rect) {
        app.view(&Id::PlayerProfile, frame, area);
    }
}

impl Screen for PlayerProfileScreen {
    fn active_id(&self) -> Id {
        Id::PlayerProfile
    }
}
