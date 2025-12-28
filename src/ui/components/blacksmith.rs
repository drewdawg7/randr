use tuirealm::{
    command::{Cmd, CmdResult},
    props::{AttrValue, Attribute, Props},
    Component, Event, Frame, MockComponent, NoUserEvent, State,
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};

use crate::{combat::HasGold, system::game_state, ui::Id};
use super::menu_component::{MenuComponent, MenuItem};
use super::utilities::{blacksmith_header, DOUBLE_ARROW_UP, RETURN_ARROW};

pub struct BlacksmithMenu {
    props: Props,
    menu: MenuComponent,
}

impl Default for BlacksmithMenu {
    fn default() -> Self {
        let items = vec![
            MenuItem {
                label: format!("{} Upgrade Items", DOUBLE_ARROW_UP),
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
            menu: MenuComponent::new(items),
        }
    }
}

impl MockComponent for BlacksmithMenu {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .split(area);

        // Render header with blacksmith name, gold, and max upgrades
        let player_gold = game_state().player.gold();
        let blacksmith = game_state().blacksmith();
        let header_line = blacksmith_header(&blacksmith, player_gold);
        frame.render_widget(Paragraph::new(header_line), chunks[0]);

        self.menu.view(frame, chunks[1]);
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
