use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, Frame, MockComponent, NoUserEvent, State};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::Paragraph};

use crate::{combat::{start_fight, Named}, entities::mob::MobKind, system::game_state, ui::{utilities::{ANVIL, CROSSED_SWORDS, OPEN_DOOR, PERSON, SHIRT, STORE}, Id}};
use super::menu_component::{MenuComponent, MenuItem};

pub struct MainMenu {
    props: Props,
    menu: MenuComponent,
}


impl Default for MainMenu {
   fn default() -> Self {
        let items = vec![
            MenuItem {
                label: format!("{} Fight", CROSSED_SWORDS).to_string(),
                action: Box::new(|| { start_fight(MobKind::Goblin); })
            },
            MenuItem {
                label: format!("{} Store", STORE).to_string(),
                action: Box::new(|| { game_state().current_screen = Id::Store; })
            },
            MenuItem {
                label: format!("{} Profile", PERSON).to_string(),
                action: Box::new(|| { game_state().current_screen = Id::Profile; })
            },

            MenuItem {
                label: format!("{} Equipment", SHIRT).to_string(),
                action: Box::new(|| { game_state().current_screen = Id::Equipment; })
            },
            MenuItem {
                label: format!("{} Blacksmith", ANVIL).to_string(),
                action: Box::new(|| { game_state().current_screen = Id::Blacksmith; })
            },
            MenuItem {
                label: format!("{} Quit", OPEN_DOOR).to_string(),
                action: Box::new(|| { game_state().current_screen = Id::Quit; })
            },
        ];
        Self {
            props: Props::default(),
            menu: MenuComponent::new(items),
        }
   } 
}

impl MockComponent for MainMenu {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let player_name = self.props
            .get(Attribute::Title)
            .map(|v| v.unwrap_string())
            .unwrap_or_else(|| game_state().player.name().to_string());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let style = Style::default().bold().green().underlined();
        let line = Line::from(vec![
            Span::raw("Hello, "),
            Span::styled(player_name, style)
        ]);
        frame.render_widget(Paragraph::new(line), chunks[0]);
        self.menu.view(frame, chunks[1]);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr).or_else(|| self.menu.query(attr))
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Title => self.props.set(attr, value),
            _ => self.menu.attr(attr, value),
        }
    }

    fn state(&self) -> State {
        self.menu.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.menu.perform(cmd)
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for MainMenu {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.menu.on(ev)
    }
}
