use tuirealm::{
    command::{Cmd, CmdResult},
    props::{AttrValue, Attribute, Props},
    Component, Event, Frame, MockComponent, NoUserEvent, State,
};
use ratatui::layout::Rect;

use crate::{combat::HasGold, system::game_state, ui::{utilities::HAMMER, Id}};
use crate::ui::components::widgets::menu::{Menu, MenuItem};
use crate::ui::components::utilities::{blacksmith_header, render_location_header, RETURN_ARROW};
use crate::ui::theme as colors;

pub struct BlacksmithMenu {
    props: Props,
    menu: Menu,
}


impl Default for BlacksmithMenu {
    fn default() -> Self {
        let items = vec![
            MenuItem {
                label: format!("{} Upgrade Items", HAMMER),
                action: Box::new(|| {
                    game_state().current_screen = Id::BlacksmithItems;
                }),
            },
            MenuItem {
                label: format!("{} Back", RETURN_ARROW),
                action: Box::new(|| {
                    game_state().current_screen = Id::Menu;
                }),
            },
        ];
        Self {
            props: Props::default(),
            menu: Menu::new(items),
        }
    }
}

impl MockComponent for BlacksmithMenu {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let player_gold = game_state().player.gold();
        let blacksmith = game_state().blacksmith();

        // Render header and get remaining area
        let header_lines = blacksmith_header(blacksmith, player_gold);
        let content_area = render_location_header(frame, area, header_lines, colors::FLAME_ORANGE);

        self.menu.view(frame, content_area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr).or_else(|| self.menu.query(attr))
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.menu.attr(attr, value);
    }

    fn state(&self) -> State {
        self.menu.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.menu.perform(cmd)
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for BlacksmithMenu {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.menu.on(ev)
    }
}
